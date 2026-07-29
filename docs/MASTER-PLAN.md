# CodeVoice — MASTER-PLAN

> Versão 0.1 · Fase 0 · 22/07/2026
> Plano mestre em 10 fases. Cada fase é executável por um modelo (Opus/Sonnet) de forma autônoma, **uma fase por sessão**, seguindo o protocolo do §2. Não pule fases; não misture escopos.

## 1. Visão geral das fases

| Fase | Nome                   | Depende de | Entrega verificável                         |
| ---- | ---------------------- | ---------- | ------------------------------------------- |
| 0    | Fundação (docs)        | —          | ✅ concluída (este repositório)             |
| 1    | Scaffold e qualidade   | 0          | ✅ concluída — ver PHASE-01-REPORT.md       |
| 2    | Banco e storage        | 1          | ✅ concluída — ver PHASE-02-REPORT.md       |
| 3    | Cadastro de projetos   | 2          | ✅ concluída — ver PHASE-03-REPORT.md       |
| 4    | Gravação de áudio      | 1          | 🟡 implementada — validação de voz c/ Jorge |
| 5    | Transcrição Whisper    | 4          | ✅ concluída — ver PHASE-05-REPORT.md       |
| 6    | Geração de prompts     | 3, 5       | ✅ concluída — ver PHASE-06-REPORT.md       |
| 7    | Editor e refinamento   | 6          | ações de edição/refino + salvar modelo      |
| 8    | Histórico              | 2, 6       | busca FTS, filtros, favoritos               |
| 9    | Terminal e Claude Code | 6          | copiar/abrir terminal/colar sob ação        |
| 10   | Polimento e instalador | todas      | bandeja, autostart, logs, instalador NSIS   |

Ordem recomendada: sequencial. Fases 3↔4 podem inverter se conveniente (independentes entre si).

## 2. Protocolo de execução de cada fase (obrigatório)

1. Ler `docs/` (este plano, ARCHITECTURE, SECURITY-MODEL, DATABASE-SCHEMA) e o relatório da fase anterior.
2. Listar dúvidas/riscos **antes** de codar; se algo contradiz os docs, parar e perguntar.
3. Implementar somente o escopo da fase. Não refatorar áreas alheias.
4. Ao final: `npm run lint && npm run test && npm run tauri build` (ou `cargo test` para Rust) — tudo verde.
5. Escrever `docs/PHASE-NN-REPORT.md`: o que foi feito, arquivos criados/modificados, dependências adicionadas (com justificativa), decisões tomadas, checklist de segurança (SECURITY-MODEL §6), pendências.
6. Commits pequenos e descritivos **somente quando autorizado pelo Jorge**.
7. Não declarar concluído sem executar os critérios de aceite manualmente.

## 3. Detalhamento

### Fase 1 — Scaffold e qualidade

**Escopo**: criar app Tauri 2 + React + TS strict + Vite + Tailwind 4; ESLint + Prettier; Vitest (TS) e `cargo test` (Rust); estrutura de diretórios de ARCHITECTURE.md §3–4 (módulos vazios com `mod.rs` e TODOs); tema escuro base; janela principal simples; `single-instance`; error boundary; tauri-plugin-log configurado com filtro de secrets (security/); scripts npm (`dev`, `lint`, `test`, `build`); `.editorconfig`.
**Comandos previstos**: `npm create tauri-app@latest` (template react-ts), `npm i -D tailwindcss @tailwindcss/vite eslint prettier vitest`, `cargo add` dos plugins base.
**Critérios de aceite**:

- [x] `npm run tauri dev` abre janela escura "CodeVoice"
- [x] `npm run lint`, `npm run test`, `cargo test`, `npm run tauri build` verdes
- [x] TS `strict: true`; CI local documentada no README
- [x] Segunda execução do exe foca a janela existente

✅ **Concluída em 22/07/2026** — detalhes em [PHASE-01-REPORT.md](PHASE-01-REPORT.md).

### Fase 2 — Banco e storage

**Escopo**: `rusqlite` bundled; `storage/` com pool, migrations 001 e 002 (DATABASE-SCHEMA.md §3–4), repositórios (`ProjectRepo`, `HistoryRepo`, `SettingsRepo`…); commands CRUD mínimos expostos com tauri-specta; testes de migration (do zero e incremental) e de repositório (em `:memory:` ou arquivo temp).
**Critérios de aceite**:

- [x] App cria `%APPDATA%/com.jorgebraga.codevoice/codevoice.db` com todas as tabelas na 1ª execução
- [x] Testes: aplicar migrations do zero; CRUD de projects; FTS insere/pesquisa; transação `save_flow` atômica
- [x] Bindings TS gerados e usados por uma tela de debug simples

✅ **Concluída em 22/07/2026** — detalhes em [PHASE-02-REPORT.md](PHASE-02-REPORT.md).

### Fase 3 — Cadastro de projetos

**Escopo**: telas Projects (lista, criar, editar, excluir com confirmação); validação/canonicalização de path (security/); `scanner.rs` com allowlist/denylist do SECURITY-MODEL §3; fluxo de importação assistida com preview e autorização; regras de projeto (project_rules) ordenáveis.
**Critérios de aceite**:

- [x] Cadastrar projeto real (ex.: o próprio CodeVoice) importando CLAUDE.md/README/package.json com preview
- [x] Testes do scanner: `.env` ignorado, symlink fora rejeitado, `..\` rejeitado, arquivo >512KB ignorado
- [x] Excluir projeto pede confirmação e não órfã o histórico (`ON DELETE SET NULL` verificado)

✅ **Concluída em 22/07/2026** — detalhes em [PHASE-03-REPORT.md](PHASE-03-REPORT.md). Executada com revisão adversarial de segurança: 4 vulnerabilidades reais encontradas e corrigidas com testes de regressão.

### Fase 4 — Gravação de áudio

**Escopo**: `audio/` com cpal (listar dispositivos, capturar 16 kHz mono) + hound (WAV em `%APPDATA%/com.jorgebraga.codevoice/tmp/`); atalho global configurável (tauri-plugin-global-shortcut) com captura de conflito; janela recorder (always-on-top, frameless) com indicador + contador + projeto ativo; `Esc` cancela; limite 10 min; exclusão automática do WAV pós-processamento + limpeza de órfãos no startup; metadados em `recordings`.
**Critérios de aceite**:

- [ ] Com app minimizado na bandeja: atalho abre recorder < 300 ms, grava, atalho encerra — ⚠️ implementado, **validação manual pendente (Jorge)**
- [ ] `Esc` cancela e apaga o WAV; troca de microfone funciona — ⚠️ implementado, **validação manual pendente (Jorge)**
- [x] WAV válido 16 kHz mono confirmado (smoke-test de hardware); registro em `recordings` correto (testado)
- [x] Teste unitário do ciclo de estado da gravação (idle→recording→stopped/cancelled) — 10 testes

🟡 **Implementada em 23/07/2026** — código completo e testado sem hardware; ver [PHASE-04-REPORT.md](PHASE-04-REPORT.md). Os 2 critérios de voz/teclado ao vivo aguardam validação manual do Jorge (roteiro no §7 do relatório).

### Fase 5 — Transcrição Whisper

**Escopo**: `transcription/` com trait + impl whisper-rs; `model_manager.rs` (download HTTPS com progresso + SHA-256, diretório `%APPDATA%/com.jorgebraga.codevoice/models/`); seleção de modelo nas configurações (**large-v3-turbo padrão, confirmado — ADR-001**; medium/small como alternativas para hardware fraco); transcrição com `language=pt` + initial_prompt com glossário técnico (nomes de arquivos, comandos, tecnologias do projeto ativo); eventos de progresso; erros: silêncio (RMS abaixo de limiar), modelo ausente, áudio inválido, falha.
**Spike no início da fase**: benchmark de large-v3-turbo (e opcionalmente medium/small) com áudio PT real na máquina do Jorge (tempo + qualidade), registrado no relatório — **não decide mais o modelo padrão** (já travado), serve para validar desempenho aceitável e decidir se medium/small valem a pena oferecer como opção.
**Critérios de aceite**:

- [x] Falar em PT → termos/nomes preservados (validado ao vivo; qualidade boa em áudio claro)
- [x] Primeiro uso baixa modelo com progresso e valida SHA-256 (download atômico `.part`)
- [x] Silêncio → "nenhuma fala detectada", sem crash (limiar RMS)
- [x] Transcrição registrada em `transcriptions` com engine/model/duração

✅ **Concluída em 24/07/2026** — validada com voz PT real (Jorge). Ver [PHASE-05-REPORT.md](PHASE-05-REPORT.md). Limites de velocidade (CPU i7-U) e qualidade (volume do mic) são de hardware/ambiente, com mitigação (modelo quantizado).

### Fase 6 — Geração de prompts

**Escopo**: `promptgen/` com trait + `TemplateGenerator` (determinístico, 10 modos, seções do PRODUCT-SPEC §5.4, contexto do projeto injetado) + `ClaudeCliGenerator` (spawn `claude` headless, texto via stdin, JSON output, timeout 60 s, sem tools; validar flags exatas da versão instalada); detecção de disponibilidade do CLI; fallback automático CLI→template com aviso ao usuário; meta-prompts dos 10 modos versionados em arquivos Rust/markdown embutidos.
**Fora do escopo padrão desta fase** (ADR-002b): `OpenAiGenerator` (ChatGPT via API) — só entra se o Jorge confirmar explicitamente antes do início da fase; se confirmado, adiciona escopo: chave de API da OpenAI via `keyring` (adiantada da Fase 10) + seletor de provedor em Settings.
**Critérios de aceite**:

- [x] Cada um dos 10 modos gera saída a partir de transcrição real (template validado por inspeção visual; CLI validado no contrato)
- [x] Prompt técnico contém as seções aplicáveis e omite vazias; contexto do projeto (stack/regras/proibições) presente
- [x] `claude` indisponível/não logado → fallback para template com aviso, sem erro fatal
- [x] Nenhum texto de usuário em argv (testado com `"; rm -rf` — vai por stdin)
- [x] Registro em `generated_prompts` + `prompt_history` (mesma transação + índice FTS)

✅ **Concluída em 24/07/2026** — ver [PHASE-06-REPORT.md](PHASE-06-REPORT.md). A qualidade da reescrita via CLI só é observável em ambiente com sessão logada (o do Jorge); o caminho template foi validado integralmente aqui.

### Fase 7 — Editor e refinamento

**Escopo**: tela principal com transcrição original + prompt editável; ações: copiar, regenerar, encurtar, detalhar, deixar mais técnico, dividir em etapas (via `RefineAction` do PromptGenerator), desfazer (pilha em memória), salvar como modelo (`prompt_templates`); usar modelo salvo como base de geração.
**Critérios de aceite**:

- [ ] Editar → copiar → conteúdo do clipboard = editado; `updated_at`/`content` persistidos
- [ ] Cada ação de refino altera o prompt de forma coerente; desfazer restaura estado anterior (≥10 níveis)
- [ ] Modelo salvo aparece na lista e é utilizável em nova geração

### Fase 8 — Histórico

**Escopo**: tela History com lista paginada (mais recentes primeiro), busca FTS5 (sem acentos), filtros por projeto/modo/favorito, ações copiar/editar (abre no editor)/favoritar/excluir (confirmação).
**Critérios de aceite**:

- [ ] Buscar palavra do meio de um prompt antigo → encontra; com acento trocado → encontra
- [ ] Filtros combinados funcionam; favoritar persiste; excluir some da lista e do banco
- [ ] 200 itens no histórico → lista continua fluida (paginação/virtualização)

### Fase 9 — Terminal e Claude Code

**Escopo**: `terminal/`: detectar Windows Terminal (`wt.exe`) com fallback PowerShell; abrir na pasta do projeto (path validado); detectar `claude` no PATH (`where claude`); botão "Abrir no Claude Code" (abre terminal + inicia `claude`); botão "Colar no terminal" que envia o paste **apenas no clique** (ADR-005); status `sent_to_terminal`/`copied` no histórico.
**Critérios de aceite**:

- [ ] Abrir terminal cai na pasta correta (projeto com espaço e acento no caminho)
- [ ] `claude` detectado; iniciar Claude Code funciona; sem flags de bypass de permissão em lugar nenhum
- [ ] Nenhuma colagem/execução sem clique explícito; nada de Enter automático

### Fase 10 — Polimento e instalador

**Escopo**: bandeja (ícone, menu: abrir/gravar/sair); autostart opcional (off por padrão); tela Settings completa (atalho, microfone, modelo, manter áudio, modo padrão, autostart, nível de log); tratamento global de erros revisado; docs de uso (README utilizador); instalador NSIS via `tauri build` (x64) testado em máquina limpa (ou VM); revisão final do checklist de segurança em todas as áreas.
**Critérios de aceite**:

- [ ] Instalador instala/desinstala limpo no Windows 11; app funciona sem toolchain de dev
- [ ] Fluxo completo do PRODUCT-SPEC §3 executado de ponta a ponta em build de produção
- [ ] Fechar janela minimiza para bandeja; autostart liga/desliga de verdade (registro verificado)
- [ ] `npm run lint && npm run test && cargo test && npm run tauri build` verdes

## 4. Critérios para iniciar a Fase 1

- [x] Docs da Fase 0 criados e consistentes
- [x] Decisões validadas em 22/07/2026: ADR-001 (whisper-rs, large-v3-turbo), ADR-002 (claude CLI + templates; OpenAI adiado — ADR-002b), idioma dos docs (PT), nome do app (CodeVoice, definitivo), conta GitHub (Jorgebragga12)
- [ ] Jorge autoriza a criação do scaffold (instala dependências e cria ~centenas de arquivos)
