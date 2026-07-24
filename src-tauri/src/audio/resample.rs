/// Taxa exigida pelo whisper.cpp na Fase 5 (PRODUCT-SPEC §5.2/§5.3).
pub const TARGET_SAMPLE_RATE: u32 = 16_000;

/// Converte um quadro intercalado (todos os canais de um instante) em um único valor mono,
/// pela média dos canais. Quadro vazio vira silêncio em vez de divisão por zero.
pub fn mix_to_mono(frame: &[f32]) -> f32 {
    if frame.is_empty() {
        return 0.0;
    }
    frame.iter().sum::<f32>() / frame.len() as f32
}

fn to_i16(sample: f32) -> i16 {
    // Clamp antes de escalar: acima de 1.0 o cast estouraria e daria wraparound (um pico
    // positivo viraria um estalo negativo).
    (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16
}

/// Reamostrador streaming para `TARGET_SAMPLE_RATE`, por média de janela (box filter).
///
/// **Por que média e não simples descarte de amostras**: jogar fora amostras (decimação
/// ingênua) de 48 kHz para 16 kHz dobra tudo acima de 8 kHz de volta pra dentro da banda de
/// voz como aliasing — sibilantes ("s", "ch") viram ruído espúrio, justamente o que atrapalha
/// a transcrição. Promediar as amostras que caem em cada janela de saída é um passa-baixa
/// grosseiro porém real, que atenua essas frequências em vez de rebatê-las.
///
/// É deliberadamente simples (não é um resampler polifásico com janela de Kaiser, como o do
/// crate `rubato`). Se a Fase 5 mostrar perda de qualidade mensurável na transcrição em PT,
/// trocar por `rubato` é uma mudança local a este arquivo.
pub struct Resampler {
    /// Progresso de saída por amostra de entrada (`16000 / taxa_de_entrada`).
    step: f64,
    position: f64,
    accumulator: f32,
    accumulated: u32,
}

impl Resampler {
    pub fn new(input_rate: u32) -> Self {
        let input_rate = input_rate.max(1); // evita divisão por zero em device degenerado
        Self {
            step: TARGET_SAMPLE_RATE as f64 / input_rate as f64,
            position: 0.0,
            accumulator: 0.0,
            accumulated: 0,
        }
    }

    /// Consome uma amostra mono de entrada, escrevendo em `out` as amostras de saída que ela
    /// completar (zero, uma, ou — em upsampling — várias).
    pub fn push(&mut self, sample: f32, out: &mut Vec<i16>) {
        self.accumulator += sample;
        self.accumulated += 1;
        self.position += self.step;

        if self.position >= 1.0 {
            let average = self.accumulator / self.accumulated as f32;
            // `while` cobre o caso de upsampling (entrada abaixo de 16 kHz), em que uma amostra
            // de entrada gera mais de uma de saída — repetimos o valor (sample-and-hold).
            while self.position >= 1.0 {
                out.push(to_i16(average));
                self.position -= 1.0;
            }
            self.accumulator = 0.0;
            self.accumulated = 0;
        }
    }

}

// Nota: não existe `flush()` da janela parcial pendente ao encerrar. O resampler vive dentro do
// callback de áudio (thread de tempo real do driver), então expor um flush externo exigiria
// envolvê-lo num mutex — trancar um mutex nesse callback é justamente o que se evita em áudio,
// por risco de bloqueio/inversão de prioridade. O que se perde é a janela incompleta final:
// no máximo uma amostra de saída, ou seja 1/16000 s ≈ 0,06 ms. Inaudível e irrelevante para
// transcrição.

#[cfg(test)]
mod tests {
    use super::*;

    fn run(input_rate: u32, samples: &[f32]) -> Vec<i16> {
        let mut resampler = Resampler::new(input_rate);
        let mut out = Vec::new();
        for &s in samples {
            resampler.push(s, &mut out);
        }
        out
    }

    #[test]
    fn mixes_stereo_frame_to_mono_average() {
        assert_eq!(mix_to_mono(&[1.0, 0.0]), 0.5);
        assert_eq!(mix_to_mono(&[0.5]), 0.5);
        assert_eq!(mix_to_mono(&[]), 0.0);
    }

    #[test]
    fn downsamples_48khz_to_a_third_of_the_samples() {
        let input = vec![0.0_f32; 4800]; // 100 ms a 48 kHz
        let out = run(48_000, &input);
        // 100 ms a 16 kHz = 1600 amostras (tolerância de 1 pela janela parcial do flush).
        assert!(
            (out.len() as i64 - 1600).abs() <= 1,
            "esperava ~1600 amostras, obteve {}",
            out.len()
        );
    }

    #[test]
    fn downsamples_44100hz_to_the_expected_count() {
        let input = vec![0.0_f32; 44_100]; // 1 s a 44.1 kHz
        let out = run(44_100, &input);
        assert!(
            (out.len() as i64 - 16_000).abs() <= 1,
            "esperava ~16000 amostras, obteve {}",
            out.len()
        );
    }

    #[test]
    fn passthrough_at_16khz_keeps_sample_count() {
        let input = vec![0.0_f32; 16_000];
        let out = run(16_000, &input);
        assert_eq!(out.len(), 16_000);
    }

    #[test]
    fn upsamples_when_input_rate_is_below_target() {
        let input = vec![0.0_f32; 8_000]; // 1 s a 8 kHz
        let out = run(8_000, &input);
        assert!(
            (out.len() as i64 - 16_000).abs() <= 1,
            "esperava ~16000 amostras, obteve {}",
            out.len()
        );
    }

    #[test]
    fn preserves_a_constant_signal_level() {
        let input = vec![0.5_f32; 4_800];
        let out = run(48_000, &input);
        let expected = (0.5 * i16::MAX as f32) as i16;
        // Sinal constante tem que sair constante — média de janela não pode distorcer DC.
        for sample in &out {
            assert!(
                (*sample - expected).abs() <= 1,
                "esperava ~{expected}, obteve {sample}"
            );
        }
    }

    #[test]
    fn clamps_instead_of_wrapping_on_overdriven_input() {
        let out = run(16_000, &[5.0, -5.0]);
        assert_eq!(out, vec![i16::MAX, i16::MIN + 1]);
    }

    #[test]
    fn averaging_attenuates_content_above_the_new_nyquist() {
        // Onda alternando +1/-1 a 48 kHz = 24 kHz, muito acima do Nyquist de 8 kHz do alvo.
        // Com decimação ingênua isso viraria um sinal forte "rebatido" (aliasing); com média
        // de janela tem que ser fortemente atenuado — é o ponto do filtro existir.
        let input: Vec<f32> = (0..4_800).map(|i| if i % 2 == 0 { 1.0 } else { -1.0 }).collect();
        let out = run(48_000, &input);
        let peak = out.iter().map(|s| s.unsigned_abs()).max().unwrap_or(0);
        assert!(
            peak < (i16::MAX as f32 * 0.4) as u16,
            "conteúdo acima do Nyquist deveria ser atenuado, mas o pico foi {peak}"
        );
    }
}
