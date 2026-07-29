# CodeVoice

App desktop para Windows que transforma fala desorganizada em prompts técnicos estruturados para o Claude Code. Fale sobre um bug, uma feature ou uma ideia; o CodeVoice transcreve localmente (Whisper) e gera um prompt de engenharia pronto para colar no terminal.

**Status**: Fases 0–7 concluídas. O fluxo completo já roda: atalho global → gravação → transcrição local (Whisper) → prompt técnico estruturado → editor com refino, biblioteca de 117 modelos e "salvar como modelo". Faltam histórico (Fase 8), integração com o terminal (Fase 9) e polimento/instalador (Fase 10).

## Stack

Tauri 2 · React 19 + TypeScript strict · Tailwind CSS 4 · Rust · SQLite (rusqlite) · whisper.cpp (whisper-rs) · `claude` CLI headless para geração de prompts.

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

IDE recomendada: [VS Code](https://code.visualstudio.com/) + extensões em [.vscode/extensions.json](.vscode/extensions.json) (Tauri + rust-analyzer).

## Para implementadores (humanos ou modelos)

Antes de escrever qualquer código, leia os docs acima e siga o **protocolo de execução de fase** do MASTER-PLAN §2. Uma fase por sessão; lint + testes + build verdes ao final; relatório `PHASE-NN-REPORT.md` obrigatório.
