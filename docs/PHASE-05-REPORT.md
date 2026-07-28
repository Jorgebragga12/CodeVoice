# CodeVoice — PHASE-05-REPORT (Transcrição Whisper)

> Executada em 24/07/2026 por Claude (Opus 4.8), com validação ao vivo pelo Jorge (voz/áudio PT real).

## 1. Resumo

Transcrição local funcionando de ponta a ponta: whisper.cpp (via `whisper-rs`) transcreve WAV PT em texto, com normalização de volume, detecção de silêncio, download de modelo verificado por SHA-256, progresso na UI, e exclusão automática do áudio após processar. **Validado ao vivo** — o app transcreveu áudio PT real com nomes próprios e pontuação corretos.

## 2. Critérios de aceite (MASTER-PLAN §3, Fase 5)

- [x] **Falar em PT → termos/nomes preservados** — validado ao vivo: uma transcrição de fala clara em PT saiu com frases completas, pontuação e nomes próprios corretos (ex.: "Atena", "Hades", "Máscara da Morte", "Saga"). O engine claramente lida bem com PT + substantivos próprios. O `initial_prompt` injeta o glossário técnico do projeto ativo (stack) para enviesar a grafia de termos como `package.json`, `useEffect`.
- [x] **Primeiro uso baixa modelo com progresso + valida SHA-256** — validado: o Jorge baixou o `large-v3-turbo` (1.6 GB) pela UI, com barra de progresso; o SHA-256 é conferido contra o hash fixado antes de o arquivo ser aceito (arquivo baixado para `.part`, renomeado só após verificar).
- [x] **Silêncio → sem crash** — `SILENCE_RMS_THRESHOLD` corta áudio abaixo de ~-52 dBFS antes do Whisper (que "alucina" sobre silêncio), devolvendo `NoSpeech`. Coberto por testes de `rms`.
- [x] **Transcrição registrada em `transcriptions`** — validado lendo o banco diretamente: linhas com `text`, `model_name`, `duration_ms` (tempo de processamento), `engine`.

**Verificação independente**: `cargo test` → **115 unitários + 1 integração**; `npm run lint`/`typecheck` limpos; `npm run test` → **15 testes**; build de produção anterior ok. Validação de voz feita com o Jorge, lendo os resultados direto do SQLite.

## 3. Achados da validação ao vivo (importantes e honestos)

1. **Qualidade do engine é boa em áudio claro.** A transcrição de fala PT nítida saiu excelente (nomes, pontuação). Os testes iniciais ruins ("aтICrível", texto embolado) tinham **duas causas de áudio, não do app**: (a) o microfone do Jorge capta muito baixo — a normalização precisou de **12x de ganho** (o teto), sinal de entrada fraca; (b) frases casuais/contar números fazem o Whisper entrar em loop de repetição. Fala clara e num volume normal transcreve bem.

2. **Velocidade é limitada pelo CPU.** No notebook do Jorge (Intel i7-10510U, série U de baixo consumo, 1.8 GHz), o modelo `large-v3-turbo` (809M params) levou **65–187 s** por clipe de ~10 s. Não é bug nem falta de otimização — o build.rs do `whisper-rs-sys` já compila o whisper.cpp otimizado (`RelWithDebInfo`) mesmo em modo dev. É o modelo grande sendo pesado nesse hardware. **Mitigação**: o modelo quantizado `large-v3-turbo-q5_0` (574 MB, no catálogo) é ~2x mais rápido com qualidade quase idêntica; o `small` (488 MB) é ~4–5x mais rápido. A escolha fica com o usuário nas Configurações. Aceleração por GPU (Vulkan/CUDA) foi avaliada e **descartada por ora** — a GPU do Jorge (MX110, 2 GB VRAM) é fraca e o setup (CUDA toolkit) não compensa.

## 4. Dependências e configuração de build

| Item                                      | Nota                                                                                                                                                                                                                                                   |
| ----------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `whisper-rs` 0.16 + `whisper-rs-sys`      | whisper.cpp via FFI (ADR-001). Compila do zero via **CMake + libclang + MSVC**.                                                                                                                                                                        |
| **Toolchain de build (crítico)**          | A máquina não tinha CMake nem LLVM. Resolvido **sem instalar nada no sistema**: `pip install cmake libclang` (ambiente Python do usuário, sem admin). O build exige os env vars `PATH` (cmake) e `LIBCLANG_PATH` — documentados na memória do projeto. |
| `reqwest` 0.13 (`native-tls`, `blocking`) | download dos modelos. **native-tls (SChannel)** de propósito: evita compilar `aws-lc-rs`/`ring` (que precisariam de NASM no Windows).                                                                                                                  |
| `sha2`                                    | verificação de integridade dos modelos.                                                                                                                                                                                                                |

## 5. Arquitetura e decisões

1. **Trait `TranscriptionEngine`** (ADR-001) com impl única `WhisperEngine`. Mantém a troca de engine possível sem tocar em commands/UI.
2. **Normalização de volume** (`normalize.rs`): pico → ~-3 dBFS, teto de 12x, silêncio intocado. Resolve o áudio baixo automaticamente (foi o que amplificou o sinal fraco do Jorge). 6 testes.
3. **`ProgressSink = Arc<dyn Fn(u8)+Send+Sync>`** (não `Box` com lifetime): o `set_progress_callback_safe` do whisper-rs exige `'static`; a camada de commands satisfaz capturando um `AppHandle` clonado.
4. **Barra de progresso indeterminada na transcrição** (não %): o Whisper não reporta % confiável em clipes curtos (vai de 0 a 100 no fim), então uma barra animada é mais honesta que um número travado. O download, esse sim, mostra % real (byte a byte).
5. **Logs do whisper.cpp silenciados** (`install_logging_hooks` via `Once`): sem isso, cada transcrição despejava ~500 linhas de debug no stdout/log.
6. **Transcrição em `spawn_blocking`**: o Whisper prende a CPU por dezenas de segundos; travaria o worker do Tauri e toda a UI.
7. **`recording:stopped` dispara a transcrição de um lugar só** (RecordBar) — cobre botão, atalho global, janela recorder e limite de 10 min, sem duplicar.

## 6. Segurança e privacidade (SECURITY-MODEL §6)

- [x] **Áudio apagado após virar texto** — `transcribe_recording` remove o WAV e limpa `audio_path` quando `keep_audio` está off (padrão). Silêncio/falha também apagam.
- [x] **Modelo verificado** — SHA-256 fixado, conferido antes de aceitar; download atômico via `.part`.
- [x] **HTTPS** no download (native-tls/SChannel).
- [x] Nenhum secret em logs; conteúdo de transcrição não é logado em nível `info` (só o ganho de normalização).
- [x] Nenhuma dependência nova sem justificativa (ver §4).

## 7. Pendências / polimento para depois (Fase 10)

- Rótulo "Nenhum projeto selecionado" na janela recorder é cosmético (a janela pequena não recebe o nome do projeto; o backend associa o projeto corretamente). Corrigir no acabamento.
- Deixar o modelo quantizado como recomendado/destacado na UI de seleção.
- Considerar um indicador visual de nível de áudio na janela de gravação, pra o usuário perceber quando o microfone está baixo (a causa nº 1 de transcrição ruim aqui).
- Aceleração por GPU: reavaliar só se houver hardware adequado.

## 8. Estado

Fase 5 **funcionalmente concluída e validada com voz PT real**. Os limites encontrados (velocidade no CPU do Jorge, qualidade dependente do volume do microfone) são de hardware/ambiente, não do app, e têm mitigação (modelo quantizado, volume do mic). Pronta para a Fase 6 (geração de prompts), que consome o texto transcrito.
