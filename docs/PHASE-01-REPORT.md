# CodeVoice — PHASE-01-REPORT (Scaffold e qualidade)

> Executada em 22/07/2026 por Claude (Sonnet 5), autorizada pelo Jorge após validação da Fase 0.

## 1. Resumo

Scaffold completo do app Tauri 2 + React 19 + TypeScript strict criado, mesclado com os documentos da Fase 0 sem perdas. Qualidade (lint/format/testes) configurada e verde. Esqueleto dos 9 módulos Rust de domínio criado com TODOs apontando a fase responsável. 7 plugins Tauri 2 registrados. Filtro de secrets no logger implementado de ponta a ponta (não só a função — está de fato ligado ao formatter do `tauri-plugin-log`) e testado. Build de produção (release + instaladores MSI e NSIS) concluído com sucesso.

## 2. Critérios de aceite (MASTER-PLAN §3, Fase 1)

- [x] `npm run tauri dev`/binário release abre janela escura "CodeVoice" — verificado rodando `src-tauri/target/release/codevoice.exe`: título da janela confirmado via PowerShell (`MainWindowTitle='CodeVoice'`); tema escuro é incondicional no CSS (não existe caminho de tema claro no código, então não há como renderizar "não escuro")
- [x] `npm run lint`, `npm run test`, `cargo test`, `npm run tauri build` verdes — todos executados nesta fase, saída registrada abaixo
- [x] TS `strict: true` (herdado do template `react-ts`, confirmado em `tsconfig.json`); comandos de qualidade documentados no [README.md](../README.md#desenvolvimento)
- [x] Segunda execução do exe foca a janela existente — verificado: 2ª instância (PID separado) teve `HasExited=True` em ~2s, apenas 1 processo `codevoice` permaneceu rodando

Verificação visual foi feita por processo + título via PowerShell (`Start-Process` + `Get-Process`), não por screenshot — sessão não-interativa evitou abrir fluxo de permissão de computer-use. O Jorge pode confirmar visualmente a qualquer momento com `npm run tauri dev`.

## 3. Comandos executados ao final (todos verdes)

```
npm run lint        # ESLint — 0 problemas
npm run typecheck   # tsc --noEmit — 0 erros
npm run test        # Vitest — 1 arquivo, 1 teste
npm run format:check # Prettier — codebase inteira formatada
cargo check          # src-tauri — compila limpo
cargo test           # src-tauri — 8 testes (log_filter), todos ok
npm run tauri build   # release + MSI + NSIS — sucesso
```

`npm run tauri build` produziu:

- `src-tauri/target/release/codevoice.exe`
- `src-tauri/target/release/bundle/msi/CodeVoice_0.1.0_x64_en-US.msi`
- `src-tauri/target/release/bundle/nsis/CodeVoice_0.1.0_x64-setup.exe`

(WiX e NSIS foram baixados automaticamente pelo Tauri CLI na primeira build, com verificação de hash.)

## 4. Arquivos criados

**Config/tooling**: `package.json`, `package-lock.json`, `tsconfig.json`, `tsconfig.node.json`, `vite.config.ts`, `vitest.config.ts`, `eslint.config.js`, `.prettierrc.json`, `.prettierignore`, `.editorconfig`, `index.html`, `.vscode/extensions.json`

**Frontend** (`src/`): `app/App.tsx`, `app/App.test.tsx`, `app/ErrorBoundary.tsx`, `windows/main/Home.tsx`, `main.tsx` (atualizado), `index.css`, `vite-env.d.ts` (atualizado), `test/setup.ts`

**Backend Rust** (`src-tauri/`): `Cargo.toml`, `Cargo.lock`, `tauri.conf.json`, `capabilities/default.json`, `src/lib.rs`, `src/main.rs` (atualizado), `src/security/mod.rs`, `src/security/log_filter.rs` (com 8 testes), e stubs `src/{domain,commands,audio,transcription,promptgen,projects,storage,terminal,settings}/mod.rs`

**Docs**: este relatório; pequenos ajustes em ARCHITECTURE.md (§0 identidade do app, §8 log em vez de tracing), MASTER-PLAN.md, .gitignore (`.claude/` e padrões Node adicionais)

Não removidos, apenas herdados do scaffold sem alteração: `src-tauri/icons/*`, `src-tauri/build.rs`, `src-tauri/.gitignore`.

Removidos do template padrão (cruft de demo): `src/App.css`, `src/App.tsx` (raiz, substituído por `src/app/App.tsx`), `src/assets/react.svg`, `public/tauri.svg`, `public/vite.svg`, comando `greet` de exemplo em `lib.rs`.

## 5. Dependências adicionadas — justificativa

**Rust** (`src-tauri/Cargo.toml`):

| Crate                            | Justificativa                                                                                                                                                    |
| -------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `tauri-plugin-single-instance`   | requisito do MVP: 2ª execução foca janela existente (PRODUCT-SPEC §5.1)                                                                                          |
| `tauri-plugin-log`               | logs locais com rotação (PRODUCT-SPEC §5.1); formatter customizado já aplica `security::log_filter::redact()` em toda linha                                      |
| `tauri-plugin-autostart`         | inicialização automática opcional (PRODUCT-SPEC §5.1) — plugin registrado nesta fase, toggle de UI fica pra Fase 10                                              |
| `tauri-plugin-clipboard-manager` | copiar prompt para área de transferência (PRODUCT-SPEC §5.8) — plugin registrado, uso real na Fase 6/9                                                           |
| `tauri-plugin-global-shortcut`   | atalho global de gravação (PRODUCT-SPEC §5.2) — plugin registrado, binding real na Fase 4                                                                        |
| `tauri-plugin-store`             | `app_settings` não sensíveis (ARCHITECTURE §3) — uso real a partir da Fase 2                                                                                     |
| `tauri-plugin-shell`             | spawn restrito de terminal e do `claude` CLI (ARCHITECTURE §6) — uso real nas Fases 6/9, nenhuma permissão concedida ainda em `capabilities/` (menor privilégio) |
| `regex`, `once_cell`             | usados por `security::log_filter` (compilação lazy dos padrões de redação)                                                                                       |
| `log`                            | facade usada pelo `tauri-plugin-log`; necessária para `log::LevelFilter`/`log::Record` no formatter customizado                                                  |

**Decisão registrada**: `thiserror`, `tracing` e `dunce` — cogitados no PHASE-00-REPORT — foram **removidos** ao final desta fase por estarem sem nenhum código que os referenciasse ainda (nenhum `AppError` foi criado, nenhuma canonicalização de path existe até a Fase 3, e o logger usa a facade `log` diretamente, não `tracing`). Serão readicionados nas fases que realmente os consomem (thiserror + AppError: Fase 2; dunce: Fase 3). Ver ADR atualizado em ARCHITECTURE.md §8.

**JS/TS** (`package.json`): exatamente a lista prevista no PHASE-00-REPORT §4 — `tailwindcss`/`@tailwindcss/vite`, `zustand`, `eslint`+plugins (`@eslint/js`, `typescript-eslint`, `eslint-plugin-react-hooks`, `eslint-plugin-react-refresh`, `eslint-config-prettier`, `globals`), `prettier`, `vitest`+`@vitest/ui`+`jsdom`, `@testing-library/react`+`jest-dom`+`user-event`. Nenhuma dependência fora do previsto foi adicionada. `@tauri-apps/plugin-opener` veio do template padrão do `create-tauri-app`, não foi escolha desta fase.

**Deliberadamente NÃO adicionado ainda** (evita instalar plugin JS sem uso real): `@tauri-apps/plugin-*` para global-shortcut/clipboard-manager/store/autostart no lado JS — só entram quando a fase que efetivamente os invoca do frontend chegar (Fase 4, 6, 9, 10).

## 6. Decisões arquiteturais tomadas nesta fase

1. **Estrutura TS não foi criada 100% conforme o desenho literal de ARCHITECTURE.md §4.** Pastas `components/`, `stores/`, `ipc/`, `lib/` e `windows/recorder/` **não foram criadas vazias** — git não rastreia diretórios vazios e um placeholder sem conteúdo real vira lixo morto até a fase que o preenche. Serão criadas organicamente quando a fase correspondente escrever o primeiro arquivo real ali (`ipc/` na Fase 2, `stores/` + `windows/recorder/` na Fase 4, etc.). ARCHITECTURE.md §4 continua sendo a referência de onde cada coisa deve morar.
2. **Logs usam a facade `log`, não `tracing`** (correção do plano da Fase 0 — ver §5 acima e ADR em ARCHITECTURE.md §8).
3. **`capabilities/default.json` mantido mínimo** (`core:default`, `opener:default`, herdado do template) — nenhuma permissão nova concedida porque nenhum código frontend chama os plugins novos ainda. Cada fase futura adiciona exatamente a permissão que passa a usar.
4. **Sem `invoke_handler`/commands ainda** — não faria sentido registrar um handler vazio; entra na Fase 2 com os primeiros commands reais de `projects`.
5. **Janela principal**: 1000×680, mínimo 720×480, label `"main"` explícito (usado por `single-instance` e futuramente por outras janelas para diferenciar da `recorder`).

## 7. Checklist de segurança (SECURITY-MODEL §6)

- [x] Nenhum secret em logs — `security::log_filter::redact()` implementado e testado (8 testes: chave Anthropic, tokens GitHub, AWS key, Bearer, password/token=, JWT, bloco PEM, texto comum inalterado); **ligado de fato** ao formatter do `tauri-plugin-log`, não é só uma função solta
- [x] Nenhum caminho aceito sem canonicalização — N/A nesta fase (nenhum código ainda lida com paths de projeto; entra na Fase 3)
- [x] Nenhum texto de usuário interpolado em linha de comando — N/A nesta fase (nenhum spawn de processo com input do usuário ainda)
- [x] Capabilities Tauri revisadas — mínimo herdado do template, nada novo concedido (ver §6.3)
- [x] Nenhuma dependência nova sem justificativa — ver §5, incluindo a remoção do que ficou sem uso
- [x] Ações destrutivas com confirmação — N/A nesta fase (nenhuma ação destrutiva existe ainda)

## 8. Pendências para a Fase 2

- Nenhum bloqueio conhecido. `storage/mod.rs` está vazio e pronto para receber `rusqlite` + migrations conforme DATABASE-SCHEMA.md.
- Ao adicionar os primeiros commands (Fase 2), lembrar de registrar `.invoke_handler(tauri::generate_handler![...])` em `lib.rs` (removido nesta fase por não ter nenhum command ainda) e configurar `tauri-specta` (ADR-004) para gerar os bindings TS em `src/ipc/`.
- `thiserror` volta no `Cargo.toml` junto com o primeiro `AppError` (ver §5).
