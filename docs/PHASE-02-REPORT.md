# CodeVoice — PHASE-02-REPORT (Banco e storage)

> Executada em 22/07/2026 por Claude (Sonnet 5), em continuação autônoma via `/loop` após a Fase 1.

## 1. Resumo

Camada de persistência completa: `rusqlite` com pool `r2d2`, migrations 001 (schema completo, 8 tabelas) e 002 (busca full-text) aplicadas automaticamente na inicialização, `ProjectRepo` (CRUD completo) e `HistoryRepo` (gravação atômica do fluxo completo + busca FTS), primeiros 5 commands expostos via `tauri-specta` com bindings TypeScript gerados, e uma tela de debug no frontend provando o pipeline Rust → SQLite → IPC → React de ponta a ponta.

## 2. Critérios de aceite (MASTER-PLAN §3, Fase 2)

- [x] **App cria o banco com todas as tabelas na 1ª execução** — verificado rodando o binário: arquivo criado em `%APPDATA%\com.jorgebraga.codevoice\codevoice.db` (+ `.db-wal`/`.db-shm`, confirmando WAL mode ativo). _Nota_: o caminho real usa o **identifier** do bundle, não o nome de exibição "CodeVoice" como os docs da Fase 0 assumiam — corrigido em ARCHITECTURE.md, DATABASE-SCHEMA.md, MASTER-PLAN.md e SECURITY-MODEL.md nesta fase.
- [x] **Testes**: migrations do zero (`applies_from_zero_creates_every_table`), incremental (`applies_incrementally_from_partial_state`) e idempotência (`applying_twice_is_idempotent`); CRUD completo de `ProjectRepo` (6 testes); FTS insere/pesquisa incluindo busca sem acento (`search_ignores_accents`); `save_flow` atômico verificado (4 tabelas populadas em 1 transação). **20 testes Rust, todos verdes.**
- [x] **Bindings TS gerados e usados por uma tela de debug simples** — `src/ipc/bindings.ts` gerado automaticamente a cada build debug; `src/windows/main/ProjectsDebug.tsx` lista projetos e cria um projeto de teste usando esses bindings, montada em `Home.tsx`.

`npm run lint`, `npm run typecheck`, `npm run test` (frontend), `cargo test` (20/20) e `npm run tauri build` (release + MSI + NSIS) — todos verdes.

## 3. Arquivos criados/modificados

**Rust** (`src-tauri/src/`): `storage/mod.rs`, `storage/error.rs`, `storage/migrations.rs` (com testes), `storage/project_repo.rs` (com testes), `storage/history_repo.rs` (com testes), `domain/mod.rs` (reescrito — tipos reais), `commands/projects.rs`, `commands/mod.rs` (atualizado), `lib.rs` (wiring do specta builder, pool, managed state).

**Frontend**: `src/ipc/bindings.ts` (gerado, não editar à mão — ignorado por ESLint/Prettier), `src/windows/main/ProjectsDebug.tsx`, `src/windows/main/Home.tsx` (atualizado).

**Docs**: este relatório; correções em ARCHITECTURE.md, DATABASE-SCHEMA.md (§4 reescrita — ver §5 abaixo), MASTER-PLAN.md, SECURITY-MODEL.md (caminho real do app data dir); MASTER-PLAN.md com Fase 2 marcada concluída.

## 4. Dependências adicionadas — justificativa

| Crate                                           | Justificativa                                                                                                                                   |
| ----------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| `rusqlite` (bundled)                            | SQLite embutido, sem dependência de sistema (ADR-003)                                                                                           |
| `r2d2` + `r2d2_sqlite` (bundled)                | pool de conexões (ARCHITECTURE §3); feature `bundled` explícita no r2d2_sqlite pra não linkar contra um SQLite de sistema diferente do rusqlite |
| `thiserror`                                     | volta ao projeto para `StorageError` (primeiro uso real — ver PHASE-01-REPORT §8)                                                               |
| `specta` (2.0.0-rc.25, feature `derive`)        | deriva `Type` nos structs de domínio pra gerar os bindings TS                                                                                   |
| `tauri-specta` (2.0.0-rc, feature `typescript`) | builder que coleta os commands e exporta os bindings (ADR-004)                                                                                  |
| `specta-typescript`                             | backend de exportação TS usado pelo tauri-specta                                                                                                |
| `tempfile` (dev-dependency)                     | banco de teste isolado por arquivo temporário — ver §6                                                                                          |

**Nenhuma dependência fora do previsto**, mas duas armadilhas de versão valem registro:

- `cargo add specta`/`tauri-specta` sem pin de versão resolve para **v1.x**, uma linha antiga incompatível com o Tauri 2.11.x atual (erro de resolução envolvendo `webkit2gtk`, mesmo em build Windows). A correção foi pinar explicitamente `specta@2.0.0-rc.25` e `tauri-specta@2.0.0-rc` — ambos da mesma linha `2.0.0-rc`, que resolve limpo.
- `r2d2_sqlite` não herda a feature `bundled` do `rusqlite` automaticamente — precisa ser habilitada explicitamente nele também, senão o Cargo pode tentar linkar contra SQLite de sistema.

## 5. Decisões arquiteturais tomadas nesta fase

1. **IDs em `i32`, não `i64`.** `specta-typescript` recusa exportar `i64`/`u64` por padrão (risco de precisão acima de 2^53 em JS/BigInt) e a versão 0.0.12 não expõe uma API pública pra configurar esse comportamento (só existe suporte a "bigint" como anotação por-tipo, não uma flag global de builder). Como um app local nunca vai chegar perto do teto de `i32` (2,1 bilhões de linhas), todos os IDs e durações que cruzam a fronteira IPC usam `i32`. `conn.last_insert_rowid()` (que retorna `i64`) é convertido com `as i32` na borda do repositório.
2. **`history_fts` é uma tabela FTS5 populada pelo código, não por triggers SQL.** O design original em DATABASE-SCHEMA.md §4 (Fase 0) previa "external content" com triggers sincronizando `transcriptions`/`generated_prompts` → `history_fts`. Isso foi abandonado: o texto buscável vem de duas tabelas que não compartilham rowid com `prompt_history`, o que tornaria os triggers frágeis e difíceis de testar corretamente numa única fase. A tabela agora guarda uma cópia do texto + `history_id` (pra join), inserida explicitamente por `HistoryRepo::save_flow` na mesma transação. DATABASE-SCHEMA.md §4 foi reescrita pra refletir isso.
3. **Banco de teste por arquivo temporário, não `:memory:`.** Cada conexão de um pool r2d2 abriria seu próprio banco `:memory:` isolado (elas não compartilham estado por padrão), quebrando qualquer teste que pegue mais de uma conexão do pool. `tempfile::NamedTempFile` resolve isso com o mesmo comportamento de produção.
4. **Caminho real do banco corrigido nos docs**: `%APPDATA%\<bundle-identifier>\`, não `%APPDATA%\CodeVoice\` como a Fase 0 assumia. Tauri v2 resolve `app_data_dir()` pelo _identifier_ (`com.jorgebraga.codevoice`), não pelo nome de exibição.
5. **`ProjectsDebug.tsx` é temporária.** Existe só pra provar o pipeline Rust→SQLite→IPC→React funcionando; será substituída pela tela real de Projetos na Fase 3 (que também adiciona validação de path, scanner com denylist e UI de verdade).
6. **`AppError` estruturado (ARCHITECTURE §8) ainda não implementado.** Os commands desta fase retornam `Result<T, String>` (o `.to_string()` do `StorageError`) — suficiente pra tela de debug. O tipo `AppError { code, message_pt, detail }` fica para quando a UI precisar diferenciar tipos de erro pro usuário (path duplicado vs. não encontrado vs. validação), provavelmente Fase 3.

## 6. Checklist de segurança (SECURITY-MODEL §6)

- [x] Nenhum secret em logs — nenhuma mudança nesta fase; filtro da Fase 1 continua ativo
- [x] Nenhum caminho aceito sem canonicalização — N/A ainda (path de projeto só é validado na Fase 3); `NewProject.path` é armazenado como veio, sem tratamento — **risco conhecido e aceito para esta fase**, documentado como bloqueio explícito da Fase 3
- [x] Nenhum texto de usuário interpolado em linha de comando — N/A (nenhum spawn de processo nesta fase)
- [x] Capabilities Tauri revisadas — nenhuma nova permissão concedida (commands de projects não passam por `invoke` restrito por capability, são commands normais do specta; o acesso ao banco é só Rust-side)
- [x] Nenhuma dependência nova sem justificativa — ver §4
- [x] Ações destrutivas com confirmação — `delete_project` existe no repo/command mas **a tela de debug não tem botão de excluir**; confirmação de UI fica pra Fase 3 quando a exclusão for exposta de verdade

## 7. Pendências para a Fase 3

- `NewProject.path` chega sem validação/canonicalização — Fase 3 precisa resolver isso **antes** de expor qualquer forma de cadastro real (ver SECURITY-MODEL §3).
- `ProjectsDebug.tsx` deve ser removida/substituída pela tela real de Projetos.
- Nenhum bloqueio técnico conhecido: `storage::ProjectRepo` já está pronto para a Fase 3 consumir diretamente.
