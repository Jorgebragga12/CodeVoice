# CodeVoice — PHASE-03-REPORT (Cadastro de projetos)

> Executada em 22/07/2026 por Claude (Sonnet 5 implementando, Opus 4.8 orquestrando e verificando), via workflow multi-agente com revisão adversarial de segurança.

## 1. Resumo

Cadastro de projetos completo: validação/canonicalização de path, scanner de importação assistida respeitando a allowlist/denylist do SECURITY-MODEL §3, CRUD de regras de projeto, e a tela real de Projetos substituindo a `ProjectsDebug.tsx` temporária da Fase 2.

**Esta fase foi executada com verificação adversarial**: depois da implementação, 4 agentes independentes atacaram em paralelo a validação de path e o scanner (path traversal, symlink/junction escape, vazamento de segredos, peculiaridades de nomes reservados do Windows), tentando bypasses **reais e executados**, não revisão de leitura. Foram 14 tentativas de ataque, **4 confirmadas como falhas reais** — todas corrigidas com teste de regressão. Ver §5.

## 2. Critérios de aceite (MASTER-PLAN §3, Fase 3)

- [x] **Cadastrar projeto real importando CLAUDE.md/README/package.json com preview** — teste de integração `src-tauri/tests/phase3_e2e.rs` registra o **próprio repositório CodeVoice** pelo pipeline completo (validação → scan → `ProjectRepo::create`). Como este repo não tem `CLAUDE.md` na raiz, esse arquivo específico é coberto pelo teste unitário `reads_claude_md_readme_and_package_json` com fixture sintético.
- [x] **Testes do scanner**: `.env` na raiz ignorado (`ignores_env_file_at_root`), symlink fora do projeto rejeitado (`rejects_symlink_pointing_outside_project_root`), `..\` rejeitado (`rejects_path_with_traversal_segment`), arquivo >512KB ignorado (`ignores_file_larger_than_512kb`).
- [x] **Excluir projeto pede confirmação e não órfã o histórico** — `ConfirmDialog` na UI; teste `deleting_project_sets_history_project_id_to_null_instead_of_orphaning_rows` prova o `ON DELETE SET NULL`.

**Verificação executada de forma independente pelo orquestrador** (não apenas relatada pelos agentes): `cargo test` → **59 unitários + 1 integração, 0 falhas**; `npm run lint` → limpo; `npm run typecheck` → limpo; `npm run test` → **11 testes, 3 arquivos**; `npm run tauri build` → release + MSI + NSIS.

## 3. Arquivos criados

**Rust**: `security/path_validation.rs`, `projects/scanner.rs`, `commands/scanner.rs`, `storage/project_rule_repo.rs`, `tests/phase3_e2e.rs`.

**Frontend**: `stores/projectStore.ts` (Zustand — primeira store do projeto), `components/ConfirmDialog.tsx`, `windows/main/Projects.tsx`, `ProjectForm.tsx`, `ProjectContextFields.tsx`, `ProjectRulesEditor.tsx`, `ImportPreviewPanel.tsx`, `Projects.test.tsx`, `ProjectForm.test.tsx`.

**Modificados**: `Cargo.toml`/`Cargo.lock`, `security/mod.rs`, `projects/mod.rs`, `domain/mod.rs`, `storage/mod.rs`, `storage/project_repo.rs`, `commands/projects.rs`, `commands/mod.rs`, `lib.rs`, `src/ipc/bindings.ts` (regenerado), `src/test/setup.ts` (ver §4.6), `windows/main/Home.tsx`.

**Removidos**: `windows/main/ProjectsDebug.tsx` (temporária da Fase 2, como planejado); `src-tauri/scanner.rs.bak` (backup órfão deixado por um agente — confirmado byte-idêntico ao `scanner.rs` atual antes de remover).

## 4. Dependências e decisões

| Crate                             | Justificativa                                                                                                                                                                                                                                                                                                                                                                                                         |
| --------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `dunce`                           | canonicalização sem os prefixos `\\?\` do Windows (já previsto no PHASE-00-REPORT §4, "dunce: Fase 3")                                                                                                                                                                                                                                                                                                                |
| `windows-sys` (só `cfg(windows)`) | detectar hardlinks NTFS via `GetFileInformationByHandle`/`nNumberOfLinks`. Necessário porque `std::os::windows::fs::MetadataExt::number_of_links` continua atrás da feature instável `windows_by_handle`. Já é dependência transitiva profunda (tauri/rusqlite), então não adiciona nova árvore de supply-chain. **Não estava previsto na Fase 0** — entrou como consequência direta de um achado de segurança (§5.1) |

Decisões não especificadas nos docs:

1. **UNC e `\\?\` verbatim tratados juntos** (ambos começam com `\\`).
2. **"2 níveis"** interpretado como: filhos diretos = nível 1, netos = nível 2; sem recursão além disso.
3. **`docker-compose*.yml` literal** — só `.yml`, não `.yaml`, seguindo o texto exato do SECURITY-MODEL §3.
4. **Conteúdo não-UTF8 descartado** como binário (defesa extra além do filtro por extensão).
5. **"Usar conteúdo nas notas"** só concatena um digest bruto no campo Notas — nenhum parsing automático de `package.json` pra pré-preencher Stack/Arquitetura, o que fugiria do espírito de importação _assistida_.
6. **Bug pré-existente de infra de teste corrigido**: `vitest.config.ts` não usa `test.globals: true`, então o `afterEach` que o `@testing-library/react` procura pra registrar limpeza automática nunca existiu — invisível até agora porque cada arquivo de teste tinha um único `it()`. Sem o fix, todo teste multi-`it()` desta fase em diante vazaria DOM entre casos. Adicionado `afterEach(cleanup)` explícito em `src/test/setup.ts`.

## 5. Achados de segurança — revisão adversarial (4 confirmados, 4 corrigidos)

Cada correção tem um teste de regressão que foi **validado como load-bearing**: desativando a guarda, o teste volta a falhar reproduzindo o bypass original.

### 5.1 Hardlink disfarçado sob nome permitido (mais grave)

Um hardlink NTFS **não requer privilégio elevado** e é indistinguível de arquivo comum via `DirEntry::file_type().is_symlink()` (retorna `false`). Um `.env`/`id_rsa`/`secrets.json` hardlinkado dentro do projeto sob o nome `README.md` tinha o conteúdo lido integralmente pro preview. Isso furava **as duas** defesas ao mesmo tempo: a de symlink-fora-da-raiz (nunca acionava) e a denylist-por-nome (o nome checado é o do link escolhido pelo atacante, não o do arquivo real). Como a importação assistida existe justamente pra ler projetos de terceiros, era um vetor de exfiltração de arquivo arbitrário no mesmo volume, sem elevação.
**Correção**: `has_multiple_hard_links()` recusa qualquer arquivo com `nNumberOfLinks > 1` (Windows) / `nlink() > 1` (Unix). Não há forma portátil de enumerar todos os nomes que apontam pro mesmo inode, então >1 link = não confiável.

### 5.2 Diretório com nome sensível não filtrado

`is_denylisted_dir()` só comparava contra a lista técnica fixa (`.git`, `node_modules`, …) e nunca chamava `is_denied_by_name()`. Um diretório literalmente chamado `secrets/` ou `.credentials/` (a) tinha o nome exposto no preview e (b) era percorrido normalmente, com qualquer arquivo allowlisted lá dentro tendo o conteúdo lido.
**Correção**: `is_denylisted_dir()` agora encadeia `is_denied_by_name()`.

### 5.3 Path longo devolvia valor não-re-validável

`dunce::canonicalize` só consegue remover o prefixo `\\?\` quando o resultado fica abaixo de ~260 chars. Para um diretório local legítimo com caminho mais longo (comum: monorepo aninhado, OneDrive, usuário com nome grande), `validate_project_root` devolvia `Ok("\\?\C:\...")` — e a **própria função** rejeitava esse valor na chamada seguinte como "UNC não suportado". Um projeto validaria uma vez e nunca mais.
**Correção**: re-check de `is_unc_or_verbatim()` sobre o resultado da canonicalização, preservando a invariante "o que a função devolve, ela também aceita de volta".

### 5.4 Nomes de dispositivo reservados com espaço/ponto/ADS

O parser DOS legado ignora espaços e pontos finais: `"CON "`, `"CON. "` e a sintaxe de stream `CON:stream` também resolvem pro dispositivo `CON`, mas o pre-check original não pegava essas formas.
**Correção**: `reserved_device_name()` corta ADS (`split(':')`) e aplica `trim_end_matches([' ', '.'])` no stem.

**Não confirmados** (10 tentativas que a implementação já resistia corretamente): junction NTFS (`mklink /J` — `is_symlink()` felizmente retorna `true` pra junctions nesta toolchain), barras mistas, `..` com espaço/ponto final, traversal URL-encoded, `\\?\` escondendo traversal, byte NUL embutido, path drive-relative, confusão de prefixo de diretório irmão (`Foo` vs `FooBar`), e extensão composta `config.env.local` (pega por outra regra).

## 6. Checklist de segurança (SECURITY-MODEL §6)

- [x] Nenhum secret em logs — filtro da Fase 1 intacto; scanner nunca loga conteúdo de arquivo
- [x] **Nenhum caminho aceito sem canonicalização** — resolvido nesta fase; `create_project` valida/canonicaliza antes de gravar, fechando o risco documentado no PHASE-02-REPORT §7
- [x] Nenhum texto de usuário interpolado em linha de comando — N/A (nenhum spawn de processo nesta fase)
- [x] Capabilities Tauri revisadas — nenhuma permissão nova; o acesso ao filesystem é 100% Rust-side, o frontend só recebe o preview
- [x] Nenhuma dependência nova sem justificativa — ver §4
- [x] Ações destrutivas com confirmação — `ConfirmDialog` na exclusão de projeto

## 7. Pendências e pontos para o Jorge validar

1. **Exclusão de regra individual não tem diálogo de confirmação** (só a de projeto tem). O SECURITY-MODEL §5 lista explicitamente projeto/histórico/modelo de prompt/modelo Whisper, e regras de projeto não estão nessa lista — são texto de uma linha, fácil de recriar. É a decisão de menor confiança da fase; vale o Jorge confirmar se concorda.
2. **`cargo fmt` não é parte do fluxo** — `cargo fmt --check` acusa diffs inclusive em arquivos já commitados nas Fases 1–2 (não existe `rustfmt.toml`; o código foi escrito com largura ~100, o rustfmt default quer ~90). Rodar agora reformataria o projeto inteiro. Decidir em algum momento: adotar `rustfmt.toml` com `max_width = 100` e formatar tudo de uma vez, ou seguir sem.
3. **`AppError` estruturado** (ARCHITECTURE §8) continua não implementado — commands ainda retornam `Result<T, String>`. A UI de Projetos já mostra mensagens de erro do backend, mas não diferencia tipos programaticamente. Ainda não bloqueia nada.
