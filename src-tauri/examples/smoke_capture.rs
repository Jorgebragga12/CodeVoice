//! Teste de fumaça manual da captura real de áudio (não roda no `cargo test` porque depende de
//! hardware). Abre o microfone padrão, grava ~1 s, e imprime o header do WAV resultante.
//!
//! Uso: `cargo run --example smoke_capture` dentro de `src-tauri`.
//! Serve para provar, sem precisar falar nada, que o cpal abre o device nesta máquina e que o
//! arquivo sai como 16 kHz mono 16-bit. A validação de que a VOZ é capturada corretamente
//! continua dependendo do Jorge (Fase 4, critério de aceite manual).

use std::time::Duration;

fn main() {
    let tmp = std::env::temp_dir().join("codevoice_smoke.wav");
    println!("gravando 1s do microfone padrão para {}", tmp.display());

    let handle = match codevoice_lib::audio::__smoke_start_capture(tmp.clone()) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("falha ao abrir o microfone: {e}");
            std::process::exit(1);
        }
    };

    std::thread::sleep(Duration::from_secs(1));
    let samples = handle.stop().expect("falha ao encerrar captura");
    println!("amostras gravadas (16 kHz): {samples}");

    let reader = hound::WavReader::open(&tmp).expect("WAV não abriu");
    let spec = reader.spec();
    println!(
        "header: {} canal(is), {} Hz, {} bits",
        spec.channels, spec.sample_rate, spec.bits_per_sample
    );
    assert_eq!(spec.channels, 1, "esperava mono");
    assert_eq!(spec.sample_rate, 16_000, "esperava 16 kHz");
    assert_eq!(spec.bits_per_sample, 16, "esperava 16-bit");
    println!("OK: WAV 16 kHz mono 16-bit gerado com sucesso");

    let _ = std::fs::remove_file(&tmp);
}
