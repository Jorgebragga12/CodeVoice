# CodeVoice — PHASE-06-REPORT (Geração de prompts)

> Executada em 24/07/2026 por Claude (Opus 4.8).

## 1. Resumo

O núcleo do produto: a transcrição vira **prompt técnico estruturado** para o Claude Code. 10 modos (PRODUCT-SPEC §5.4), contexto do projeto injetado (stack, regras, tecnologias proibidas, comandos de teste), geração via `claude` CLI com **fallback determinístico por template**, persistência em `generated_prompts` + `prompt_history` + índice de busca.

## 2. Critérios de aceite (MASTER-PLAN §3, Fase 6)

- [x] **Cada um dos 10 modos gera saída a partir de transcrição real** — `every_mode_produces_non_empty_output` cobre os 10; a saída foi **inspecionada visualmente** via `cargo run --example show_prompt <modo>` (não só assert de "não vazio").
- [x] **Prompt técnico contém as seções aplicáveis e omite vazias; contexto do projeto presente** — `injects_project_context_and_rules` e `omits_empty_sections_when_there_is_no_project`. Verificado no exemplo real: stack, regras, tecnologias proibidas e comandos de teste aparecem no prompt.
- [x] **`claude` ausente → fallback para template com aviso, sem erro fatal** — o `DefaultPromptGenerator` captura qualquer erro do CLI (ausente/offline/**não logado**) e devolve o template com `fallback_reason` preenchido; a UI mostra o aviso. Testado de fato: **este ambiente não tem sessão logada no CLI**, então o caminho de fallback é o que efetivamente roda aqui.
- [x] **Nenhum texto de usuário em argv** — `hostile_transcript_stays_inside_the_delimiters` usa `"; rm -rf / && echo pwned` como transcrição; o texto vai por **stdin** (`claude_cli::generate`), os argumentos são lista fixa.
- [x] **Registro em `generated_prompts` + `prompt_history`** — `create_with_history` grava os dois **na mesma transação**, mais a linha de `history_fts`; testado incluindo busca por texto.

**Verificação**: `cargo test` → **153 unitários + 1 integração** (+38 nesta fase); `npm run test` → **21 testes** (+6); `npm run lint`/`typecheck` limpos; `npm run tauri build` ok.

## 3. O que EU validei vs. o que depende do Jorge

**Validado por mim, de ponta a ponta**: o `TemplateGenerator` (caminho offline) — inclusive lendo a saída real gerada. O contrato do CLI foi validado contra o binário instalado (v2.1.201): flags `--print --output-format json --disallowed-tools` aceitas, formato de resposta confirmado, e o comportamento de erro **observado na prática** (`is_error: true` com exit 0 quando não há sessão).

**Não validável aqui**: a _qualidade_ do prompt reescrito pelo Claude, porque este ambiente não tem login no CLI. O Jorge, que tem sessão ativa, verá o caminho `claude_cli` quando usar o app — e o rótulo na UI ("via Claude" / "via template") deixa explícito qual foi usado.

## 4. Arquitetura e decisões

1. **`build_llm_prompt` delimita a transcrição** com marcadores `--- TRANSCRIÇÃO DA FALA ---`. A fala do usuário é **dado**, não instrução: sem a delimitação, algo dito em voz alta ("ignore as regras acima") teria mais chance de ser lido como comando pelo modelo.

2. **Segurança do spawn** (SECURITY-MODEL §2): o prompt inteiro vai por **stdin**; `argv` é uma lista fixa. `std::process::Command` no Windows não passa por `cmd.exe`, então não há interpretação de shell. Além disso, **todas as ferramentas do CLI são bloqueadas** (`--disallowed-tools Bash,Read,Write,Edit,...`) — gerar prompt é tarefa de texto puro, o CLI não deve tocar em arquivos. **Nenhuma flag de bypass de permissão é usada** (proibido pelo PRODUCT-SPEC §5.8).

3. **`is_error` é checado além do exit code.** O CLI devolve exit 0 mesmo em falha lógica (ex.: "Not logged in"), sinalizando por `is_error` no JSON. Confiar só no status de saída faria a mensagem de erro ser salva **como se fosse o prompt gerado** — foi observado na prática ao testar o binário real.

4. **"Transcrição limpa" nunca passa por LLM** (ADR-002). É limpeza determinística: remove hesitações, pontua e capitaliza, sem reescrever. Mandar ao modelo arriscaria a reescrita/invenção que este modo existe justamente para evitar.

5. **Template não inventa conteúdo.** Nas seções que dependem de interpretação (requisitos funcionais, arquivos relacionados), o template emite **instruções para o agente derivá-las** em vez de texto inventado — um requisito falso é pior que uma seção honestamente genérica.

6. **Refino sem CLI devolve o prompt intacto + aviso.** Encurtar/detalhar exige um LLM; não há equivalente determinístico. Fingir sucesso alterando o texto de qualquer jeito seria pior que dizer "não deu".

7. **`original_content` preservado na edição** — permite comparar/reverter ao texto gerado (Fase 7).

## 5. Arquivos criados

**Rust**: `promptgen/modes.rs` (10 modos + seções), `promptgen/context.rs` (contexto do projeto), `promptgen/templates.rs` (gerador determinístico), `promptgen/claude_cli.rs` (spawn + parse), `promptgen/mod.rs` (trait + fallback), `commands/promptgen.rs`, `storage/prompt_repo.rs`, `examples/show_prompt.rs` (inspeção humana da saída).

**Frontend**: `stores/promptStore.ts`, `windows/main/PromptPanel.tsx`, `windows/main/PromptPanel.test.tsx`.

**Modificados**: `domain/mod.rs` (+`GeneratedPrompt`, `NewGeneratedPrompt`, `PromptMode::from_db_str`), `storage/mod.rs`, `commands/mod.rs`, `lib.rs`, `stores/transcriptionStore.ts` (+`transcriptionId`), `windows/main/RecordBar.tsx`, `MainWindow.tsx`, `src/ipc/bindings.ts` (regenerado).

**Dependências**: nenhuma nova.

## 6. Segurança (SECURITY-MODEL §6)

- [x] Nenhum texto de usuário em argv — stdin sempre; teste com payload hostil
- [x] Nenhuma execução implícita — todas as ferramentas do CLI bloqueadas; nenhuma flag de bypass
- [x] Texto ≠ comando — a transcrição é delimitada como dado no prompt do LLM
- [x] Nenhum secret em logs — o filtro da Fase 1 cobre; o conteúdo do prompt não é logado
- [x] Nenhuma dependência nova
- [x] Ações destrutivas com confirmação — N/A nesta fase

## 7. Pendências

- **Qualidade da saída via CLI** só será observável quando rodar num ambiente logado (o do Jorge).
- **Refino (`refine_prompt`) já existe no backend** mas ainda não tem botões na UI — é escopo da Fase 7 (editor), junto com edição livre, desfazer e "salvar como modelo".
- `prompt_templates` (modelos salvos pelo usuário) continua sem uso — Fase 7.
