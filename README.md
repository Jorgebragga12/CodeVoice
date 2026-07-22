# CodeVoice

App desktop para Windows que transforma fala desorganizada em prompts técnicos estruturados para o Claude Code. Fale sobre um bug, uma feature ou uma ideia; o CodeVoice transcreve localmente (Whisper) e gera um prompt de engenharia pronto para colar no terminal.

**Status**: Fase 0 (fundação) concluída — apenas documentação; nenhum código de aplicação ainda.

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
| [PHASE-00-REPORT](docs/PHASE-00-REPORT.md) | Relatório da fundação: ambiente, decisões, riscos |

## Desenvolvimento

```bash
npm install
npm run tauri dev      # app desktop em modo desenvolvimento
npm run lint            # ESLint
npm run test            # Vitest
npm run tauri build     # build de produção + instalador
```

IDE recomendada: [VS Code](https://code.visualstudio.com/) + extensões em [.vscode/extensions.json](.vscode/extensions.json) (Tauri + rust-analyzer).

## Para implementadores (humanos ou modelos)

Antes de escrever qualquer código, leia os docs acima e siga o **protocolo de execução de fase** do MASTER-PLAN §2. Uma fase por sessão; lint + testes + build verdes ao final; relatório `PHASE-NN-REPORT.md` obrigatório.
