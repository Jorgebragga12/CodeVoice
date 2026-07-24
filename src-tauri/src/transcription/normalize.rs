/// Alvo de pico após a normalização: ~-3 dBFS (0.7 do fundo de escala). Não vai a 0 dBFS de
/// propósito — deixar uma folga evita que qualquer arredondamento estoure em clipping audível.
const TARGET_PEAK: f32 = 0.7;

/// Abaixo disto o ganho não é aplicado: ou é silêncio (nada a amplificar) ou o áudio já está
/// alto o suficiente. Também evita explodir ruído de fundo de uma gravação vazia.
const MIN_PEAK_TO_NORMALIZE: f32 = 0.02; // ~-34 dBFS

/// Teto de ganho. Amplificar mais que isto transformaria o chiado de fundo de uma gravação
/// muito baixa num rugido — pior para o Whisper do que o áudio original.
const MAX_GAIN: f32 = 12.0;

/// Normaliza o volume de amostras PCM i16 in-place, elevando o pico ao alvo.
///
/// Motivação concreta: no teste real da Fase 4 a gravação saiu com pico ~-30 dBFS (1085/32767),
/// nível baixo que degrada a acurácia do Whisper. Amplificar o pico para ~-3 dBFS antes de
/// transcrever recupera esse sinal. Áudio já alto (ou silêncio) passa intocado.
///
/// Devolve o ganho aplicado (1.0 = nenhum), útil para log/diagnóstico.
pub fn normalize_peak(samples: &mut [i16]) -> f32 {
    let peak = samples
        .iter()
        .map(|s| (*s as f32 / i16::MAX as f32).abs())
        .fold(0.0_f32, f32::max);

    if peak < MIN_PEAK_TO_NORMALIZE {
        return 1.0;
    }

    let gain = (TARGET_PEAK / peak).min(MAX_GAIN);
    if gain <= 1.0 {
        return 1.0; // já está no nível ou acima; não atenuamos
    }

    for sample in samples.iter_mut() {
        let amplified = (*sample as f32) * gain;
        *sample = amplified.clamp(i16::MIN as f32, i16::MAX as f32) as i16;
    }
    gain
}

/// RMS normalizado (0.0–1.0) do buffer. Usado para detectar silêncio antes de gastar tempo
/// mandando o áudio pro Whisper (que "alucina" texto em cima de silêncio).
pub fn rms(samples: &[i16]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    let sum_sq: f64 = samples
        .iter()
        .map(|s| {
            let v = *s as f64 / i16::MAX as f64;
            v * v
        })
        .sum();
    ((sum_sq / samples.len() as f64).sqrt()) as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amplifies_quiet_audio_towards_the_target_peak() {
        // Pico em ~0.033 (o nível baixo real medido na Fase 4).
        let mut samples = vec![(0.033 * i16::MAX as f32) as i16; 100];
        let gain = normalize_peak(&mut samples);
        assert!(gain > 1.0, "áudio baixo deveria ser amplificado");

        let new_peak = samples.iter().map(|s| (*s as f32 / i16::MAX as f32).abs()).fold(0.0, f32::max);
        // Chega perto do alvo (limitado pelo teto de ganho, mas 0.033*12 ≈ 0.4 < 0.7, então
        // aqui o ganho é o teto de 12x → pico ~0.4).
        assert!(new_peak > 0.3, "pico após normalização muito baixo: {new_peak}");
    }

    #[test]
    fn respects_the_gain_ceiling_on_very_quiet_audio() {
        let mut samples = vec![(0.03 * i16::MAX as f32) as i16; 50];
        let gain = normalize_peak(&mut samples);
        assert!(gain <= super::MAX_GAIN + 0.001, "ganho passou do teto: {gain}");
    }

    #[test]
    fn leaves_already_loud_audio_untouched() {
        let original = vec![(0.8 * i16::MAX as f32) as i16; 50];
        let mut samples = original.clone();
        let gain = normalize_peak(&mut samples);
        assert_eq!(gain, 1.0);
        assert_eq!(samples, original, "áudio já alto não deveria mudar");
    }

    #[test]
    fn leaves_silence_untouched_instead_of_amplifying_noise() {
        let original = vec![3_i16; 50]; // ruído ínfimo, ~-80 dBFS
        let mut samples = original.clone();
        let gain = normalize_peak(&mut samples);
        assert_eq!(gain, 1.0);
        assert_eq!(samples, original);
    }

    #[test]
    fn never_clips_after_normalizing() {
        // Sinal assimétrico perto do limite negativo.
        let mut samples = vec![-8000_i16, 6000, -7000, 5000];
        normalize_peak(&mut samples);
        // Nenhuma amostra pode ter estourado o range de i16 (o clamp garante isso).
        assert!(samples.iter().all(|s| *s >= i16::MIN && *s <= i16::MAX));
    }

    #[test]
    fn rms_of_silence_is_zero_and_of_signal_is_positive() {
        assert_eq!(rms(&[]), 0.0);
        assert_eq!(rms(&[0, 0, 0]), 0.0);
        assert!(rms(&[10_000, -10_000, 10_000]) > 0.0);
    }
}
