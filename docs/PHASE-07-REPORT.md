# CodeVoice — PHASE-07-REPORT (Editor e refinamento)

> Executada em 29/07/2026 por Claude (Opus 5).

## 1. Resumo

O prompt gerado deixou de ser um texto somente-leitura e virou um **editor**: transcrição original
ao lado do prompt editável, seis ações (copiar, regenerar, encurtar, detalhar, mais técnico,
dividir em etapas), desfazer com pilha em memória e "salvar como modelo". Junto veio a parte que
estava só no repositório e nunca no app: a **biblioteca de 117 modelos** de `templates/`,
importada para `prompt_templates` via migration 003 e navegável por categoria na aba **Modelos**.

## 2. Critérios de aceite (MASTER-PLAN §3, Fase 7)

- [x] **Editar → copiar → clipboard = editado; `content`/`updated_at` persistidos** — `copies the
edited text and persists it` (PromptEditor.test.tsx) digita no editor, clica em Copiar e lê o
      **clipboard de volta** (stub do `userEvent`), além de checar a chamada a
      `update_prompt_content`. O botão Copiar salva antes de copiar, sempre; há também autosave de
      1,2 s para o caso de o usuário fechar o app sem clicar em nada.
- [x] **Cada ação de refino altera o prompt de forma coerente; desfazer ≥10 níveis** — as 4 ações
      chegam ao `refine_prompt` com a `RefineAction` correta (`runs each refine action through the
backend`); `keeps at least 10 undo levels` faz 12 edições e desfaz as 12 uma a uma, checando
      o texto em cada passo. Refino e regeneração também entram na pilha.
- [x] **Modelo salvo aparece na lista e é utilizável em nova geração** — `stores the prompt as a
reusable template` cobre o salvar; `generates a prompt from the chosen template` cobre o
      usar, com `generate_prompt_from_template` recebendo transcrição e modelo.

**Verificação executada**: `cargo test` → **183 unitários + 1 integração** (+30); `npm run test` →
**45 testes** (+24); `npm run lint`, `npm run typecheck` e `npm run format:check` limpos;
`npm run tauri build` gerando MSI e NSIS.

**Verificação no app real, não só em teste**: rodando o binário sobre o banco existente (que
estava na versão 002), `PRAGMA user_version` foi para 3, `schema_migrations` registrou a
003_prompt_templates, e a tabela ficou com **117 modelos embutidos em 18 categorias**. Conferido
que o conteúdo gravado começa no corpo (`Estou com um erro no projeto **[nome do projeto]**.`) e
não no cabeçalho. Segundo startup: continua 117 — o seed é idempotente de fato, não só no teste.

## 3. O que EU validei vs. o que depende do Jorge

**Validado aqui de ponta a ponta**: edição, persistência, desfazer, reverter ao original, salvar
como modelo, excluir modelo com confirmação, e a biblioteca inteira — os 117 arquivos são
parseados e conferidos por teste (modo válido, descrição presente, metadado fora do corpo,
marcador `<<SUA FALA>>` presente em todos).

**Não validável aqui**: a _qualidade_ do texto que volta das 4 ações de refino, porque elas passam
pelo `claude` CLI e este ambiente não tem sessão logada. O contrato está testado e, sem CLI, o
backend devolve o prompt **intacto com aviso** em vez de fingir que alterou — a UI mostra esse
aviso e o painel já avisa antes, quando o CLI nem existe.

## 4. Decisões

1. **Categoria = pasta, não o campo `Área`.** O handoff pedia `Área → category`, mas a biblioteca
   tem **22 valores distintos de `Área` para 18 pastas** (`negocios-produto/` produz "negócios" e
   "produto"; `dados-ia/` produz "dados", "IA" e "automação"). A pasta é estável e casa com a
   navegação do repositório. Divergência levada ao Jorge e confirmada antes de codar.

2. **`<<SUA FALA>>` venceu `{{transcript}}`.** O `DATABASE-SCHEMA.md` documentava
   `{{transcript}}`/`{{project_context}}`, que nunca viraram código; `<<SUA FALA>>` está nos 117
   arquivos versionados e é legível para humano. O doc foi corrigido (§5.1), não os modelos.

3. **Auto-substituição de 2 campos, não 6.** Só `[nome do projeto]` (97 arquivos) e `[comando de
teste]` (14) são **dado** que o app tem. Os demais campos frequentes (`[N]`, `[valor]`,
   `[comando]`, `[período]`) são decisões — preenchê-los com chute seria pior que deixar o literal
   pedindo preenchimento. Sem projeto ativo, o literal também sobrevive: um `[nome do projeto]`
   visível é um lembrete; um vazio silencioso é um prompt pela metade.

4. **Biblioteca embutida no binário via `build.rs` + `include_str!`.** A alternativa (resource do
   Tauri lido em runtime) acrescentaria acesso a fs para conteúdo que é estático entre execuções.
   Zero dependência nova; `cargo:rerun-if-changed` por arquivo garante rebuild ao editar um modelo.

5. **Seed reescreve os `builtin` a cada startup.** Mantém banco e binário em sincronia após um
   update (inclusive renomeações e remoções) sem versionamento próprio, e o `WHERE source =
'builtin'` garante que nada do usuário seja tocado. Como consequência, **excluir um embutido é
   recusado** pelo repositório em vez de "funcionar" e voltar no próximo startup; para alterar um,
   o usuário usa "salvar como modelo" e cria a própria cópia.

6. **Migration 003 recria a tabela.** SQLite não tem `ALTER TABLE ADD CONSTRAINT`, e a 001 não
   tinha `CHECK` no `mode` de `prompt_templates` (ao contrário de `generated_prompts`). As linhas
   existentes são copiadas — na prática nenhuma, porque até a Fase 6 nenhum código escrevia nela.

7. **Desfazer coalesce por tempo (600 ms).** Um nível por tecla tornaria o desfazer inútil; a
   janela agrupa uma rajada de digitação em um nível só. Teto de 50 níveis (o mínimo pedido é 10).

8. **Refino salva antes de chamar o backend.** `refine_prompt` lê `generated_prompts.content` do
   banco; sem salvar primeiro, o refino rodaria sobre uma versão velha e a edição pendente sumiria.

9. **Modelo não passa pelo LLM.** Ao usar um modelo, o app só encaixa a fala e os dados do
   projeto — o modelo **é** o prompt que o usuário escolheu deliberadamente; mandar ao Claude só
   arriscaria reescrevê-lo.

## 5. Arquivos

**Criados (Rust)**: `storage/template_repo.rs` (`PromptTemplateRepo` + `BuiltinTemplate`),
`promptgen/library.rs` (parser do cabeçalho, `render`, `builtins`), `commands/templates.rs`
(5 commands + rótulos das categorias).

**Criados (frontend)**: `windows/main/PromptEditor.tsx`, `RefineToolbar.tsx`,
`SaveAsTemplateDialog.tsx`, `TemplateLibrary.tsx`, e os testes `stores/promptStore.test.ts`,
`PromptEditor.test.tsx`, `TemplateLibrary.test.tsx`.

**Modificados**: `src-tauri/build.rs` (embute `templates/`), `storage/migrations.rs` (+003 e
testes), `storage/mod.rs`, `domain/mod.rs` (+`PromptTemplate`, `NewPromptTemplate`),
`commands/mod.rs`, `commands/promptgen.rs` (`load_context` virou `pub(super)`), `lib.rs` (seed +
5 commands), `stores/promptStore.ts` (pilha de desfazer, edição, persistência),
`windows/main/PromptPanel.tsx` (dividido: agora só modo + geração), `MainWindow.tsx` (aba
Modelos), `PromptPanel.test.tsx`, `src/ipc/bindings.ts` (regenerado),
`docs/DATABASE-SCHEMA.md` (§5 nova), `docs/MASTER-PLAN.md`, `templates/README.md`.

**Dependências novas**: nenhuma.

## 6. Segurança (SECURITY-MODEL §6)

- [x] **Nenhum secret em logs** — filtro da Fase 1 intocado; o seed loga só a contagem de modelos,
      nunca conteúdo de prompt ou transcrição.
- [x] **Nenhum caminho aceito sem canonicalização** — nenhum caminho novo cruza a fronteira IPC
      nesta fase; os arquivos de `templates/` são resolvidos em tempo de compilação pelo `build.rs`
      (não há leitura de fs em runtime).
- [x] **Nenhum texto de usuário interpolado em linha de comando** — o refino continua indo por
      stdin (`claude_cli::generate`); a renderização de modelo é substituição de string pura, e
      nada do texto é executado (SECURITY-MODEL §1.3: texto ≠ comando).
- [x] **Capabilities Tauri revisadas** — nenhuma permissão nova. A janela recorder segue sem
      shell/fs/clipboard; a cópia usa `navigator.clipboard` da própria WebView, como desde a Fase 6.
- [x] **Nenhuma dependência nova** — a biblioteca foi embutida com `build.rs` + `include_str!`
      justamente para evitar `include_dir` ou similar.
- [x] **Ações destrutivas com confirmação** — excluir modelo passa por `ConfirmDialog`
      (SECURITY-MODEL §5), e modelos embutidos nem oferecem o botão.
- [x] **SQL só com prepared statements** — todos os novos statements usam `params![]`.

Observação: a UI não consegue forjar um modelo `builtin`. `NewPromptTemplate` não tem os campos
`source`/`slug`; o repositório grava `'user'`/NULL fixos. Sem isso, um modelo forjado seria apagado
pelo seed do startup seguinte.

## 7. Pendências

- **Qualidade do refino via CLI** só é observável no ambiente do Jorge (mesma pendência da Fase 6).
- **Busca nos modelos**: 117 itens navegam bem por categoria, mas uma busca por texto seria
  melhor. Fica natural junto da tela de Histórico (Fase 8), que já terá FTS.
- **`generated_prompts.mode` de um modelo** herda o modo declarado no arquivo; o dropdown de modo
  do painel não reflete isso depois de usar um modelo (cosmético).
- **Regenerar cria uma linha nova** em `generated_prompts`/`prompt_history` a cada clique — é o
  comportamento herdado da Fase 6 e faz sentido para o histórico, mas vale reavaliar na Fase 8 se
  poluir a lista.
- As melhorias sugeridas no handoff (detectar modo pela fala, anexar log ao prompt, favoritos)
  continuam **fora** de escopo, sem decisão tomada.

**Encontrado de passagem, não corrigido** (área alheia, ARCHITECTURE §9): `cargo clippy
--all-targets` falha com dois erros em `transcription/normalize.rs:106` (Fase 5) —
`assert!(*s >= i16::MIN && *s <= i16::MAX)` é tautológico e o lint
`absurd_extreme_comparisons` é `deny` por padrão. O teste passa, mas quem rodar clippy leva erro
de compilação. Fora isso, `cargo fmt` nunca foi aplicado no repositório (o estilo é manual desde a
Fase 1, com linhas de ~100 colunas) — os arquivos desta fase seguem o mesmo estilo, e rodar
`cargo fmt` agora reformataria ~40 arquivos de todas as fases.
