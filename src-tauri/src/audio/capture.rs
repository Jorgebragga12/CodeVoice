use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::JoinHandle;

use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::SampleFormat;

use super::devices::resolve_input_device;
use super::error::AudioError;
use super::resample::{mix_to_mono, Resampler, TARGET_SAMPLE_RATE};

pub enum CaptureCommand {
    Stop,
    Cancel,
}

/// Alça para a thread de captura.
///
/// A captura roda em thread própria porque `cpal::Stream` não é `Send`: ele precisa ser criado
/// e destruído na mesma thread, então não dá para guardá-lo no estado gerenciado do Tauri (que
/// é compartilhado entre threads). A thread possui o stream e o writer; o resto do app fala com
/// ela só por canais.
pub struct CaptureHandle {
    command_tx: Sender<CaptureCommand>,
    result_rx: Receiver<Result<u32, AudioError>>,
    thread: JoinHandle<()>,
}

impl CaptureHandle {
    /// Encerra a captura e devolve quantas amostras (a 16 kHz) foram gravadas.
    pub fn finish(self, command: CaptureCommand) -> Result<u32, AudioError> {
        // Se a thread já morreu sozinha (erro no device, por ex.), o send falha — nesse caso o
        // resultado real do erro ainda vem pelo result_rx logo abaixo, então seguimos.
        let _ = self.command_tx.send(command);
        let outcome = self.result_rx.recv().unwrap_or_else(|_| {
            Err(AudioError::Stream(
                "thread de captura encerrou sem resposta".into(),
            ))
        });
        let _ = self.thread.join();
        outcome
    }
}

/// Inicia a captura em background gravando WAV mono 16 kHz em `output`.
///
/// Só retorna depois que a thread confirmou que o microfone abriu — assim um device inválido
/// vira erro imediato para o usuário, em vez de uma gravação que parece ter começado e depois
/// não produz nada.
pub fn start_capture(
    device_name: Option<String>,
    output: PathBuf,
) -> Result<CaptureHandle, AudioError> {
    let (command_tx, command_rx) = mpsc::channel();
    let (result_tx, result_rx) = mpsc::channel();
    let (ready_tx, ready_rx) = mpsc::channel();

    let thread = std::thread::spawn(move || {
        run_capture(device_name, output, command_rx, ready_tx, result_tx);
    });

    match ready_rx.recv() {
        Ok(Ok(())) => Ok(CaptureHandle {
            command_tx,
            result_rx,
            thread,
        }),
        Ok(Err(err)) => {
            let _ = thread.join();
            Err(err)
        }
        Err(_) => Err(AudioError::Stream(
            "thread de captura encerrou antes de inicializar".into(),
        )),
    }
}

fn run_capture(
    device_name: Option<String>,
    output: PathBuf,
    command_rx: Receiver<CaptureCommand>,
    ready_tx: Sender<Result<(), AudioError>>,
    result_tx: Sender<Result<u32, AudioError>>,
) {
    let setup = setup_stream(device_name, &output);

    let (stream, sample_rx, mut writer) = match setup {
        Ok(parts) => {
            let _ = ready_tx.send(Ok(()));
            parts
        }
        Err(err) => {
            let _ = ready_tx.send(Err(err));
            return;
        }
    };

    if let Err(err) = stream.play().map_err(|e| AudioError::Stream(e.to_string())) {
        let _ = result_tx.send(Err(err));
        return;
    }

    let mut written: u32 = 0;
    let outcome = loop {
        // Drena tudo que já chegou do callback de áudio antes de olhar comandos, para não
        // perder o rabicho final do áudio ao parar.
        while let Ok(chunk) = sample_rx.try_recv() {
            for sample in chunk {
                if writer.write_sample(sample).is_err() {
                    break;
                }
                written += 1;
            }
        }

        match command_rx.try_recv() {
            Ok(CaptureCommand::Stop) => break Ok(written),
            Ok(CaptureCommand::Cancel) => break Ok(written),
            Err(mpsc::TryRecvError::Disconnected) => break Ok(written),
            Err(mpsc::TryRecvError::Empty) => {
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        }
    };

    // Para o stream antes de fechar o writer: sem isso o callback poderia continuar mandando
    // amostras para um canal cujo receptor já sumiu.
    drop(stream);
    while let Ok(chunk) = sample_rx.try_recv() {
        for sample in chunk {
            if writer.write_sample(sample).is_err() {
                break;
            }
            written += 1;
        }
    }

    let finalized = writer
        .finalize()
        .map_err(|e| AudioError::Wav(e.to_string()))
        .map(|_| written);

    let _ = result_tx.send(match outcome {
        Ok(_) => finalized,
        Err(err) => Err(err),
    });
}

type StreamParts = (
    cpal::Stream,
    Receiver<Vec<i16>>,
    hound::WavWriter<std::io::BufWriter<std::fs::File>>,
);

fn setup_stream(device_name: Option<String>, output: &PathBuf) -> Result<StreamParts, AudioError> {
    let device = resolve_input_device(device_name.as_deref())?;
    let config = device
        .default_input_config()
        .map_err(|e| AudioError::UnsupportedConfig(e.to_string()))?;

    let sample_format = config.sample_format();
    let stream_config: cpal::StreamConfig = config.into();
    let channels = stream_config.channels as usize;
    let input_rate = stream_config.sample_rate.0;

    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent).map_err(|source| AudioError::Io {
            path: parent.display().to_string(),
            source,
        })?;
    }

    let writer = hound::WavWriter::create(
        output,
        hound::WavSpec {
            channels: 1,
            sample_rate: TARGET_SAMPLE_RATE,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        },
    )
    .map_err(|e| AudioError::Wav(e.to_string()))?;

    let (sample_tx, sample_rx) = mpsc::channel::<Vec<i16>>();
    let mut resampler = Resampler::new(input_rate);
    let err_fn = |err| log::error!("erro no stream de áudio: {err}");

    // O callback roda em thread de tempo real do driver: só faz aritmética e um send por bloco.
    // Nada de I/O de disco aqui — quem escreve o WAV é a thread de captura.
    macro_rules! build {
        ($sample:ty, $to_f32:expr) => {{
            let tx = sample_tx.clone();
            device.build_input_stream(
                &stream_config,
                move |data: &[$sample], _: &cpal::InputCallbackInfo| {
                    let mut out = Vec::with_capacity(data.len() / channels.max(1) + 1);
                    for frame in data.chunks(channels.max(1)) {
                        let mono: Vec<f32> = frame.iter().copied().map($to_f32).collect();
                        resampler.push(mix_to_mono(&mono), &mut out);
                    }
                    if !out.is_empty() {
                        let _ = tx.send(out);
                    }
                },
                err_fn,
                None,
            )
        }};
    }

    let stream = match sample_format {
        SampleFormat::F32 => build!(f32, |s: f32| s),
        SampleFormat::I16 => build!(i16, |s: i16| s as f32 / i16::MAX as f32),
        SampleFormat::U16 => build!(u16, |s: u16| (s as f32 / u16::MAX as f32) * 2.0 - 1.0),
        other => {
            return Err(AudioError::UnsupportedSampleFormat(format!("{other:?}")));
        }
    }
    .map_err(|e| AudioError::Stream(e.to_string()))?;

    Ok((stream, sample_rx, writer))
}
