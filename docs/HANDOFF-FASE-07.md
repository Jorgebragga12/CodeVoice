# Handoff — continuar o CodeVoice na Fase 7

> Gerado em 24/07/2026 ao fim da sessão que concluiu a Fase 6 e a biblioteca de templates.
> **Cole o bloco da seção "Prompt para colar" numa sessão nova do Claude Code.**

---

## Prompt para colar

```
Vou continuar o desenvolvimento do CodeVoice, um app desktop Windows que transforma fala em
prompts técnicos estruturados para o Claude Code. O projeto está em
C:\Users\Jorge Braga\Documents\CLAUDETE\CodeVoice e as Fases 0 a 6 já estão concluídas e
commitadas. Agora é a Fase 7 (Editor e refinamento).

ANTES DE ESCREVER QUALQUER CÓDIGO, leia nesta ordem:
1. docs/MASTER-PLAN.md — o plano de 10 fases; o §2 tem o protocolo obrigatório de execução de
   fase e o §3 tem o escopo e os critérios de aceite da Fase 7
2. docs/PHASE-06-REPORT.md — o que foi feito na fase anterior e o que ficou pendente
3. docs/ARCHITECTURE.md e docs/SECURITY-MODEL.md — decisões travadas e regras inegociáveis
4. src-tauri/src/promptgen/ e src-tauri/src/commands/promptgen.rs — o que já existe

O REPOSITÓRIO É A FONTE DA VERDADE, não este resumo. Se algo aqui divergir do código, confie
no código e me avise da divergência.

=== O QUE JÁ EXISTE (não reimplemente) ===

O backend do refinamento está PRONTO desde a Fase 6:
- `RefineAction` (promptgen/mod.rs) com 4 variantes: Shorten, Expand, MoreTechnical,
  SplitIntoSteps — cada uma com sua instrução em português
- command `refine_prompt` — aplica uma RefineAction sobre o conteúdo atual
- command `update_prompt_content` — persiste o texto editado
- `PromptRepo::update_content` e `create_with_history` (grava generated_prompts +
  prompt_history + índice FTS na mesma transação)
- `generated_prompts.original_content` guarda o texto como gerado, separado de `content` —
  é o que permite reverter para o original

Falta essencialmente o FRONTEND do editor e toda a parte de prompt_templates.

=== ESCOPO DA FASE 7 ===

A. Editor (frontend)
   - Tela com transcrição original (somente leitura) + prompt editável lado a lado
   - Ações: copiar, regenerar, encurtar, detalhar, deixar mais técnico, dividir em etapas
     (as 4 últimas chamam refine_prompt, que já existe)
   - Desfazer com pilha em memória, no mínimo 10 níveis
   - Editar → copiar deve levar o texto EDITADO para o clipboard, e persistir em
     generated_prompts.content

B. Salvar como modelo + usar modelo salvo
   - Salvar o prompt atual em prompt_templates
   - Listar modelos e usar um como base de nova geração

C. Importar a biblioteca templates/ (117 modelos em 18 categorias)

=== O QUE PRECISA SER RESOLVIDO EM C (levantado por análise multi-agente) ===

A tabela prompt_templates hoje é: id, name, mode, content, project_id, created_at, updated_at.
Nenhum código Rust lê ou escreve nela — só existe o CREATE TABLE na migration 001.

Problemas concretos a resolver antes de importar:
1. FALTA coluna `category` — as 18 pastas não têm onde ir, e uma lista chapada de 117 itens é
   inutilizável na UI. Falta também `description` (a linha "> Uso:" de cada arquivo é a melhor
   affordance da biblioteca e hoje se perderia).
   → Precisa de uma migration 003. NUNCA edite as migrations 001/002 já aplicadas.
2. O cabeçalho de 4 linhas de cada arquivo (`# Título`, `> Modo: ... · Área: ...`, `> Uso: ...`,
   `---`) é METADADO, não corpo de prompt. Se gravar o arquivo verbatim em content, todo prompt
   gerado vai começar com "> Modo: new_feature · Área: desenvolvimento". O importador tem que
   fazer o parse: título → name, Modo → mode, Uso → description, Área → category, e só o que
   vem depois do `---` → content.
3. Os templates usam `<<SUA FALA>>` como marcador da transcrição. O DATABASE-SCHEMA.md linha 63
   documenta `{{transcript}}` e `{{project_context}}`. Decida UMA convenção e reconcilie doc e
   código — o descasamento de sintaxe é o menor problema.
4. 85 dos 117 templates pedem `[nome do projeto]`, e 9 pedem `[comando de teste]` — dados que o
   app JÁ TEM no banco (projects.name, projects.test_commands) e que ProjectContext::render()
   já sabe renderizar. Auto-substituir esses poucos campos resolve a maioria das ocorrências
   com uma tabela de ~6 entradas e string replace. Os outros ~445 campos [entre colchetes] são
   decisões, não dados — deixe literais para o usuário preencher.
5. `prompt_templates.mode` NÃO tem CHECK (compare com generated_prompts.mode, que tem). Um
   importador com bug gravaria mode inválido sem erro. Considere adicionar na migration 003.

=== CONVENÇÕES TRAVADAS (não são óbvias lendo o código) ===

- IDs e durações que cruzam a fronteira IPC são i32, NUNCA i64: specta-typescript recusa
  exportar i64/BigInt e a versão usada não tem flag para mudar isso. Converta
  last_insert_rowid() com `as i32` na borda do repositório.
- Testes de repositório usam arquivo temporário via tempfile (storage::test_pool()), NUNCA
  `:memory:` — cada conexão do pool r2d2 abriria um banco isolado e o teste quebraria.
- Os bindings TypeScript (src/ipc/bindings.ts) são GERADOS: rode o binário debug por ~3s e
  mate o processo. O export roda em #[cfg(debug_assertions)] antes de a janela abrir. Não
  edite bindings.ts à mão.
- O whisper-rs precisa destes env vars para compilar (cmake e libclang vieram via pip):
    export PATH="/c/Users/Jorge Braga/AppData/Local/Programs/Python/Python311/Lib/site-packages/cmake/data/bin:$PATH"
    export LIBCLANG_PATH="C:\Users\Jorge Braga\AppData\Local\Programs\Python\Python311\Lib\site-packages\clang\native"
  Só são necessários se o whisper-rs-sys precisar recompilar (mudança em Cargo.toml/lock).
- O `claude` CLI retorna exit 0 MESMO EM ERRO LÓGICO, sinalizando por `is_error` no JSON.
  Checar só o status faria uma mensagem de erro ser salva como se fosse o prompt gerado.
- Este ambiente de dev NÃO tem sessão logada no claude CLI, então o caminho que roda aqui é
  sempre o gerador por template. A qualidade da reescrita via Claude só o Jorge observa.
- Frontend: componentes finos, sem lógica de domínio; nada de acesso a fs/banco/rede pelo
  TypeScript; tema escuro único (paleta zinc do Tailwind).
- Arquivos: máx ~300 linhas TS, ~400 Rust. Se passar, divida.

=== PROTOCOLO (docs/MASTER-PLAN.md §2) ===

1. Liste dúvidas e riscos ANTES de codar. Se algo contradiz os docs, pare e pergunte.
2. Implemente só o escopo da fase. Não refatore área alheia.
3. Ao final, tudo verde: `npm run lint`, `npm run typecheck`, `npm run test`,
   `cargo test` (dentro de src-tauri) e `npm run tauri build`.
4. Escreva docs/PHASE-07-REPORT.md no mesmo padrão dos anteriores: o que foi feito, arquivos
   criados/modificados, dependências novas com justificativa, decisões tomadas, checklist de
   segurança (SECURITY-MODEL §6) e pendências.
5. Commits pequenos e descritivos, em português. Rode `npm run format` antes.
6. NÃO declare concluído sem executar os critérios de aceite de verdade.

Estado atual verificado: 153 testes Rust + 21 frontend passando, lint e typecheck limpos,
build de produção gerando MSI e NSIS.

Comece lendo os documentos e me apresentando o plano da Fase 7 antes de implementar.
```

---

## Contexto extra (para você, não precisa colar)

### Onde o projeto está

| Fase  | Estado                                               |
| ----- | ---------------------------------------------------- |
| 0–3   | Fundação, banco SQLite, cadastro de projetos ✅      |
| 4     | Gravação de áudio ✅ (validada ao vivo com voz real) |
| 5     | Transcrição Whisper ✅ (validada ao vivo)            |
| 6     | Geração de prompts, 10 modos ✅                      |
| **7** | **Editor e refinamento — próxima**                   |
| 8–10  | Histórico, terminal/Claude Code, polimento           |

Commits recentes: `755286d` (Fase 6), `b87f4a1` (versiona templates), `c6fbedd` (8 correções),
`777d2b9` (+12 templates, 105 → 117).

### Coisas que valem lembrar no uso

- Seu microfone capta baixo — a normalização compensa até 12x, mas subir o volume de entrada
  no Windows melhora bastante a transcrição.
- O modelo `large-v3-turbo` (1.6 GB) leva 65–187 s por clipe no seu CPU. O quantizado
  `large-v3-turbo-q5_0` (574 MB) é ~2x mais rápido com qualidade quase igual — trocar em
  Configurações.
- Contar números em voz alta faz o Whisper entrar em loop de repetição; frases normais
  transcrevem bem.

### Melhorias sugeridas que ficaram fora do plano

Levantadas na análise mas **não** commitadas nem incluídas na Fase 7 — decidir depois:

1. **Detectar o modo pela fala** (classificação automática): você fala "tá dando erro" e o app
   já escolhe `bug_fix`, eliminando o dropdown. Combina com as novas categorias `depuracao/`.
2. **Anexar erro/log ao prompt**: ninguém fala um stack trace. O fluxo real é falar + colar.
   Hoje a transcrição é o único input.
3. **Busca nos templates**: com 117, o índice em markdown não escala.
4. Favoritos / mais usados.
