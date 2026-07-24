# CodeVoice — PHASE-04-REPORT (Gravação de áudio)

> Executada em 23/07/2026 por Claude (Opus 4.8). **Fase com verificação parcial**: a lógica é
> testável sem hardware e foi testada; a captação real de voz depende do Jorge falar num
> microfone — ver §2 e §7.

## 1. Resumo

Pipeline de gravação completo: captura via `cpal` numa thread dedicada, mixdown para mono +
reamostragem para 16 kHz, escrita WAV com `hound`, máquina de estados do ciclo de gravação,
atalho global configurável, janela compacta de gravação (frameless, always-on-top), tela de
configurações (microfone/atalho/manter-áudio), limite de 10 min vigiado no backend, e limpeza
de áudios órfãos no startup.

## 2b. Validação ao vivo (24/07/2026, com o Jorge)

O teste manual encontrou e corrigiu **um bug real** que os testes sem hardware não pegariam:
a janela recorder criada em runtime com `WebviewUrl::App` não resolvia a URL contra o dev
server em modo dev — abria **em branco** e às vezes crashava o processo (exit `0xcfffffff`).
Corrigido declarando a janela em `tauri.conf.json` (oculta, criada no startup) — commit
`fbf402d`. Depois da correção:

- ✅ Gravação real de **69 s** capturada; WAV validado byte a byte: header `16 kHz / mono /
  16-bit`, RIFF finalizado corretamente, **voz presente no sinal** (pico 1085, sinal em ~6% das
  amostras = os trechos falados). Ou seja, captura + resample + WAV + stop funcionam de ponta a
  ponta com voz real.
- ✅ Sem crash; janela recorder renderiza contador/botões corretamente.
- 📌 **Áudio ficou baixo** (pico ~-30 dB) — é nível de microfone do Windows, não bug. A Fase 5
  vai normalizar o volume antes de mandar pro Whisper, o que resolve isso automaticamente.
- ⏳ Ainda leves: cronometrar o "< 300 ms" do atalho, `Esc` cancelando, e troca entre dois
  microfones físicos.

## 2. Critérios de aceite (MASTER-PLAN §3, Fase 4) — estado honesto

| Critério                                                                 | Estado                                                                                                                                                                                                                       |
| ------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Com app minimizado: atalho abre recorder < 300 ms, grava, atalho encerra | ⚠️ **Precisa do Jorge** — implementado (atalho global no backend, independente da UI estar visível), mas "abre em < 300 ms" e "grava de verdade" só se confirmam com teclado + microfone reais                               |
| `Esc` cancela e apaga o WAV; troca de microfone funciona                 | ⚠️ **Precisa do Jorge** — lógica implementada e testada (o cancel apaga o arquivo; testes cobrem isso), mas a interação de teclado na janela e a troca real de device pedem uso manual                                       |
| WAV válido 16 kHz mono confirmado; registro em `recordings` correto      | 🟡 **Parcialmente por mim** — o registro em `recordings` é testado; o header WAV (16 kHz/mono/16-bit) foi confirmado por um smoke-test de hardware real nesta máquina (ver §5), mas com silêncio/ruído de fundo, não com voz |
| Teste unitário do ciclo de estado (idle→recording→stopped/cancelled)     | ✅ **Verificado por mim** — 10 testes em `audio::session` cobrindo todas as transições e erros                                                                                                                               |

**O que EU verifiquei** (sem microfone): toda a lógica de estado, a matemática de reamostragem
(incluindo atenuação de aliasing), a gestão de arquivos temporários, os repositórios, as
settings, e que o cpal abre o device padrão e produz um WAV com o header correto nesta máquina.

**O que só o Jorge pode verificar**: que a voz é capturada com qualidade utilizável, que o
atalho global responde rápido com a janela minimizada, que a troca de microfone entre dois
dispositivos físicos funciona, e que `Esc`/botões da janela recorder se comportam ao vivo.

## 3. Arquitetura da gravação (decisões que valem registro)

1. **Captura em thread dedicada.** `cpal::Stream` não é `Send` — precisa nascer e morrer na
   mesma thread, então não cabe no estado gerenciado do Tauri (compartilhado entre threads). A
   thread possui o stream e o `WavWriter`; o resto do app fala com ela só por canais
   (`capture.rs`). `start_capture` só retorna depois que a thread confirmou que o microfone
   abriu, para que um device inválido vire erro imediato em vez de uma gravação natimorta.

2. **Reamostragem por média de janela, não descarte de amostras** (`resample.rs`). Decimar
   48 kHz→16 kHz jogando amostras fora dobra tudo acima de 8 kHz de volta pra dentro da banda de
   voz como aliasing (sibilantes viram ruído) — exatamente o que atrapalha a transcrição.
   Promediar as amostras de cada janela de saída é um passa-baixa grosseiro que atenua essas
   frequências. Há um teste (`averaging_attenuates_content_above_the_new_nyquist`) que prova a
   atenuação. Se a Fase 5 mostrar perda de qualidade mensurável em PT, trocar por um resampler
   polifásico (`rubato`) é uma mudança local a este arquivo.

3. **Limite de 10 min vigiado no backend, não na UI** (`spawn_recording_limit_watchdog` em
   `lib.rs`). Depender do frontend deixaria o limite furado justamente quando mais importa:
   janela de gravação fechada, app na bandeja, webview travada — os casos em que uma gravação
   esquecida encheria o disco silenciosamente.

4. **Lógica de start/stop/toggle compartilhada entre command e atalho** (`recording.rs`,
   funções `do_*`). O atalho global (disparado pelo SO) e os botões da UI chamam exatamente o
   mesmo código, então não há como os dois caminhos divergirem. O projeto ativo fica no backend
   (`RecorderHandle::active_project`) porque o atalho precisa saber a que projeto associar sem
   ida e volta ao frontend, que pode estar minimizado.

5. **Cancelar não grava nada em `recordings`.** Cancelar significa "isso não aconteceu"; uma
   linha registrando que o usuário falou 5 s e desistiu seria metadado comportamental sem valor
   de uso, contra o espírito de privacidade do PRODUCT-SPEC §6. Já o `stop` persiste os
   metadados mas deixa o WAV no disco — quem o apaga é a transcrição (Fase 5).

## 4. Dependências adicionadas

| Crate           | Justificativa                                                                                                                                                                                                                                                                                     |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `cpal` **0.16** | captura de áudio multiplataforma (previsto na Fase 0). **Fixado em 0.16, não 0.18**: a 0.18.1 puxa `windows-core` 0.61 e 0.62 ao mesmo tempo e não compila (as macros de uma versão geram código que a outra não satisfaz). A 0.16 usa `windows` 0.54, resolve limpo e tem a mesma API que usamos |
| `hound`         | escrita/leitura de WAV (previsto na Fase 0)                                                                                                                                                                                                                                                       |

## 5. Smoke-test de hardware (`examples/smoke_capture.rs`)

Como o `cargo test` não pode depender de microfone, a validação de que o cpal realmente abre o
device e escreve um WAV correto **nesta máquina** fica num exemplo rodável à mão:
`cargo run --example smoke_capture` (dentro de `src-tauri`). Ele grava 1 s do microfone padrão e
assere que o arquivo sai 16 kHz / mono / 16-bit. **Resultado nesta máquina (23/07/2026): abriu o
microfone padrão, gravou 16000 amostras (= exatamente 1 s a 16 kHz), header `1 canal, 16000 Hz,
16 bits` — passou.** Não valida voz, só o formato e o fato de o device abrir.

## 6. Segurança e privacidade (SECURITY-MODEL §6)

- [x] Áudio temporário isolado em `%APPDATA%/com.jorgebraga.codevoice/tmp/`; **limpeza de
      órfãos no startup** (`cleanup_orphans`) — um WAV sobrando ali é voz de uma sessão que morreu
      antes de processar, e não pode acumular
- [x] `cancel` apaga o WAV; `clear_audio_path` esquece o caminho no banco após a exclusão
- [x] "Manter áudio" **desligado por padrão** (`RecordingSettings::default`), com teste
- [x] Nenhum secret em logs; nenhum caminho de usuário interpolado em comando
- [x] **Janela recorder com capability mínima** (`capabilities/recorder.json`): só `event` e
      `start-dragging`, **sem** shell/fs/opener/clipboard — ela só mostra o contador e dispara
      parar/cancelar (ARCHITECTURE §6, menor privilégio)
- [x] Nenhuma dependência nova sem justificativa

## 7. Como o Jorge valida (roteiro de teste manual)

1. `npm run tauri dev` (ou instalar o build de produção).
2. Em **Configurações**, confirmar que o microfone aparece na lista e escolher um.
3. Minimizar a janela. Pressionar **Ctrl+Shift+Space** — a janelinha de gravação deve aparecer
   quase instantânea. Falar algo citando termos técnicos ("tauri", "package.json").
4. Pressionar o atalho de novo (ou "Parar") — a gravação encerra.
5. Testar **Esc** durante uma gravação — deve cancelar e sumir.
6. Se tiver dois microfones, trocar entre eles nas Configurações e gravar com cada um.
7. Reportar se algo travou, demorou, ou não capturou. (A qualidade da transcrição em si é
   avaliada na Fase 5.)

## 8. Verificação executada (comandos, 23/07/2026)

- `cargo test` → **100 unitários + 1 integração, 0 falhas**
- `npm run lint` / `typecheck` → limpos; `npm run test` → **15 testes, 4 arquivos**
- `npm run tauri build` → sucesso (release + MSI + NSIS)
- `cargo run --example smoke_capture` → **passou** (microfone abriu, WAV 16 kHz/mono/16-bit)

Fronteira honesta: tudo acima roda sem voz. Os 3 primeiros critérios de aceite do §2 só ficam
100% confirmados quando o Jorge seguir o roteiro do §7 com microfone e teclado reais.
