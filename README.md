# CodeVoice

App desktop para Windows que transforma fala desorganizada em prompts técnicos estruturados para o
Claude Code. Fale sobre um bug, uma feature ou uma ideia; o CodeVoice transcreve localmente
(Whisper) e devolve um prompt de engenharia pronto para colar no terminal.

**Tudo local por padrão**: o áudio é gravado, transcrito e apagado na sua máquina. Nada sai daqui,
exceto quando você usa o `claude` CLI — que roda com a sua própria assinatura.

## Estado atual

**Fases 0–7 concluídas de 10.** O fluxo principal roda de ponta a ponta: atalho global → gravação →
transcrição local (Whisper) → prompt técnico estruturado → editor com refino, biblioteca de 117
modelos e "salvar como modelo".

| Fase | Entrega                | Estado                                         |
| ---- | ---------------------- | ---------------------------------------------- |
| 0    | Fundação (docs)        | ✅                                             |
| 1    | Scaffold e qualidade   | ✅                                             |
| 2    | Banco e storage        | ✅                                             |
| 3    | Cadastro de projetos   | ✅                                             |
| 4    | Gravação de áudio      | 🟡 implementada; 2 checagens manuais pendentes |
| 5    | Transcrição Whisper    | ✅ validada com voz real em PT                 |
| 6    | Geração de prompts     | ✅ 10 modos                                    |
| 7    | Editor e refinamento   | ✅ + biblioteca de modelos                     |
| 8    | Histórico              | ⬜ próxima                                     |
| 9    | Terminal e Claude Code | ⬜                                             |
| 10   | Polimento e instalador | ⬜                                             |

Verificação atual: **183 testes Rust + 1 de integração**, **48 testes de frontend**, lint,
typecheck, formatação e clippy limpos, build de produção gerando MSI e NSIS.

## O que ainda falta

### Fase 8 — Histórico

Tela de histórico com lista paginada (mais recentes primeiro), busca full-text via FTS5 **sem
acentos**, filtros combinados por projeto/modo/favorito, e ações de copiar, editar (abre no
editor), favoritar e excluir com confirmação.

O índice FTS já é populado desde a Fase 6, na mesma transação que grava o prompt — falta a tela.
Critério de aceite: buscar uma palavra do meio de um prompt antigo (inclusive com acento trocado)
encontra, e 200 itens na lista continuam fluidos.

### Fase 9 — Terminal e Claude Code

Detectar o Windows Terminal (`wt.exe`) com fallback para PowerShell, abrir na pasta do projeto,
detectar o `claude` no PATH e oferecer "Abrir no Claude Code".

Regra travada (ADR-005): a colagem no terminal acontece **somente** sob clique explícito, e nunca
com Enter automático. Nada gerado pelo app é executado como comando.

### Fase 10 — Polimento e instalador

Ícone na bandeja com menu, autostart opcional (desligado por padrão), tela de configurações
completa, tratamento global de erros revisado, documentação de uso para o usuário final, e o
instalador NSIS testado numa máquina limpa, sem toolchain de desenvolvimento.

### Pendências abertas

- **Validação manual da Fase 4** (só quem tem a máquina consegue fazer): com o app minimizado na
  bandeja, o atalho global abre o gravador em menos de 300 ms; e `Esc` cancela apagando o WAV, com
  troca de microfone funcionando.
- **Qualidade do refino via `claude` CLI** só é observável num ambiente com sessão logada. O
  contrato está testado, mas o texto que volta nunca foi lido em ambiente de desenvolvimento.
- **Busca na biblioteca de modelos**: 117 itens navegam bem por categoria, mas falta busca por
  texto. Encaixa junto da Fase 8, que já traz FTS.
- **Regenerar cria uma linha nova** em `generated_prompts` e `prompt_history` a cada clique. Faz
  sentido para o histórico, mas pode poluir a lista — reavaliar na Fase 8.
- **O seletor de modo não reflete** o modo herdado ao usar um modelo da biblioteca (cosmético).

### Ideias levantadas, ainda sem decisão

- Detectar o modo pela fala ("tá dando erro" → `bug_fix`), eliminando o seletor.
- Anexar erro/log ao prompt: ninguém dita um stack trace, o fluxo real é falar **e** colar.
- Favoritos e "mais usados" na biblioteca de modelos.
- `OpenAiGenerator` como provedor alternativo — avaliado e adiado (ADR-002b); a trait
  `PromptGenerator` já aceita a extensão sem refatoração.

## Precisa do Claude para funcionar?

Quase nada depende dele. Existe **um único** ponto no código que chama o `claude` CLI, e ele tem
fallback em todos os caminhos:

| Funciona sem o `claude`                                     | Exige o `claude`    |
| ----------------------------------------------------------- | ------------------- |
| Gravação e transcrição (Whisper roda local, sempre)         | Encurtar o prompt   |
| Modo "Transcrição limpa" (nunca usa LLM, por design)        | Detalhar            |
| Geração nos outros 9 modos (cai no template determinístico) | Deixar mais técnico |
| Biblioteca de 117 modelos (100% local, zero IA)             | Dividir em etapas   |
| Editor: editar, desfazer, copiar, salvar como modelo        |                     |

Sem o CLI as quatro ações de refino ficam **desabilitadas**, com o motivo no tooltip — em vez de
parecerem clicáveis e não fazerem nada. Encurtar ou detalhar sem reescrever não tem equivalente
determinístico possível.

## Stack

Tauri 2 · React 19 + TypeScript strict · Tailwind CSS 4 · Rust · SQLite (rusqlite) · whisper.cpp
(whisper-rs) · `claude` CLI headless para geração de prompts.

## Documentação

| Doc                                        | Conteúdo                                          |
| ------------------------------------------ | ------------------------------------------------- |
| [PRODUCT-SPEC](docs/PRODUCT-SPEC.md)       | Escopo, fluxo principal e funcionalidades do MVP  |
| [ARCHITECTURE](docs/ARCHITECTURE.md)       | Camadas, contratos, ADRs, estrutura de diretórios |
| [DATABASE-SCHEMA](docs/DATABASE-SCHEMA.md) | Esquema SQLite, migrations, FTS                   |
| [SECURITY-MODEL](docs/SECURITY-MODEL.md)   | Modelo de ameaças e regras obrigatórias           |
| [MASTER-PLAN](docs/MASTER-PLAN.md)         | 10 fases de implementação com critérios de aceite |
| `docs/PHASE-NN-REPORT.md`                  | Um relatório por fase concluída (00 a 07)         |

A biblioteca de modelos de prompt fica em [templates/](templates/README.md) — 117 modelos em 18
categorias, embutidos no binário e disponíveis na aba **Modelos** do app.

## Desenvolvimento

```bash
npm install
npm run tauri dev            # app desktop em modo desenvolvimento
npm run tauri build          # build de produção + instalador (MSI e NSIS)
```

Checagens de qualidade — todas precisam estar verdes ao fim de cada fase (MASTER-PLAN §2):

```bash
npm run lint && npm run typecheck && npm run format:check && npm run test
npm run lint:rust && npm run format:rust:check && npm run test:rust
```

`npm run format` e `npm run format:rust` corrigem a formatação de TS/MD e de Rust.

> Compilar o `whisper-rs-sys` exige `cmake` no PATH e a variável `LIBCLANG_PATH` apontando para o
> diretório do `libclang.dll`. Só é necessário quando o crate precisa recompilar (mudança em
> `Cargo.toml`/`Cargo.lock`).

IDE recomendada: [VS Code](https://code.visualstudio.com/) + extensões em
[.vscode/extensions.json](.vscode/extensions.json) (Tauri + rust-analyzer).

## Para implementadores (humanos ou modelos)

Antes de escrever qualquer código, leia os docs acima e siga o **protocolo de execução de fase** do
MASTER-PLAN §2. Uma fase por sessão; lint, testes e build verdes ao final; relatório
`PHASE-NN-REPORT.md` obrigatório.

Três regras que não são óbvias lendo o código:

- IDs que cruzam a fronteira IPC são `i32`, nunca `i64` — o specta-typescript recusa exportar
  `i64`/BigInt.
- Testes de repositório usam arquivo temporário (`storage::test_pool()`), nunca `:memory:` — cada
  conexão do pool r2d2 abriria um banco isolado.
- `src/ipc/bindings.ts` é **gerado**; não editar à mão.
