# CodeVoice — PHASE-00-REPORT (Fundação)

> Executada em 22/07/2026 por Claude (Fable 5). Nenhum código de aplicação foi escrito nesta fase — apenas documentação e estrutura.

## 1. Ambiente verificado

| Item | Resultado |
|---|---|
| SO | Windows 11 Pro 10.0.22631 |
| Node / npm | v24.14.1 / 11.11.0 ✅ (Tauri 2 requer Node ≥18) |
| Rust / cargo | 1.97.0 / 1.97.0 ✅ (Tauri 2 requer ≥1.77) |
| `claude` CLI | presente em `~\.local\bin\claude.exe` ✅ (viabiliza ADR-002) |
| WebView2 | nativo no Windows 11 ✅ |
| Pasta do projeto | `C:\Users\Jorge Braga\Documents\CLAUDETE\CodeVoice` — estava vazia; git inicializado nesta fase |

Atenção para implementadores: o caminho do projeto contém **espaço** ("Jorge Braga") — sempre citar paths em comandos.

## 2. Arquivos criados nesta fase

- `docs/PRODUCT-SPEC.md` — escopo e funcionalidades do MVP
- `docs/ARCHITECTURE.md` — stack validada, camadas, contratos, ADRs 001–005
- `docs/DATABASE-SCHEMA.md` — 8 tabelas + schema_migrations + FTS5 + regras
- `docs/SECURITY-MODEL.md` — ameaças, denylist, filtro de logs, checklist por fase
- `docs/MASTER-PLAN.md` — 10 fases com critérios de aceite verificáveis
- `docs/PHASE-00-REPORT.md` — este arquivo
- `README.md`, `.gitignore`

## 3. Decisões — status em 22/07/2026 (confirmadas pelo Jorge)

1. **ADR-001 — CONFIRMADA.** Transcrição com `whisper-rs` (whisper.cpp in-process), não faster-whisper. Modelo **`large-v3-turbo` travado como padrão definitivo** (não é mais "a decidir por benchmark"). O spike da Fase 5 passa a ter propósito de **validação de desempenho** na máquina real e de decidir se vale oferecer `medium`/`small` como opção para hardware mais fraco — não de escolher o modelo padrão.
2. **ADR-002 — CONFIRMADA com adição.** Geração de prompts via `claude` CLI headless (assinatura existente, sem chave de API) + templates determinísticos como fallback offline, **continua sendo o provedor padrão do MVP**. O Jorge perguntou sobre usar ChatGPT/OpenAI — decisão: a trait `PromptGenerator` já é desenhada para múltiplos provedores (ADR-002 em ARCHITECTURE.md), então um `OpenAiGenerator` fica documentado como **opção B plugável, fora do MVP por padrão** (evita trazer gestão de chave de API da OpenAI para a Fase 6). Se o Jorge quiser ChatGPT ativo já na Fase 6, é só avisar — o encaixe arquitetural já está pronto, é a implementação de mais uma impl do trait.
3. **Idioma dos docs — CONFIRMADA.** Português com termos técnicos em inglês.
4. **Nome do app — CONFIRMADA.** "CodeVoice" é o nome definitivo (não é mais provisório). Bundle identifier proposto para a Fase 1: `com.jorgebraga.codevoice` (ajustável — não precisa resolver como domínio real, só ser único).
5. **Conta GitHub — CONFIRMADA.** Repositório vai na conta pessoal do Jorge, **Jorgebragga12** (não Inconformedia — essa é a conta de trabalho, hoje ativa por padrão no `gh`). Antes de criar/pushar o repo remoto será necessário `gh auth switch --user Jorgebragga12` (ação de rede/GitHub — só será executada mediante pedido explícito, não incluída nesta atualização de docs).

Decisões fechadas sem pendência desde o início: ADR-003 (rusqlite só no Rust), ADR-004 (tauri-specta), ADR-005 (colar via clipboard + ação do usuário).

## 4. Dependências sugeridas (a instalar na Fase 1+, com justificativa)

**Rust (crates)**
| Crate | Fase | Justificativa |
|---|---|---|
| `tauri` 2.x + plugins: `global-shortcut`, `autostart`, `clipboard-manager`, `single-instance`, `store`, `log`, `opener`, `shell` | 1 | shell do app e recursos nativos; cada plugin cobre um requisito do MVP |
| `rusqlite` (bundled) | 2 | SQLite embutido sem dependência de sistema (ADR-003) |
| `specta` + `tauri-specta` | 1–2 | tipos IPC gerados, elimina drift Rust↔TS (ADR-004) |
| `cpal` + `hound` | 4 | captura de áudio multiplataforma + escrita WAV |
| `whisper-rs` | 5 | binding whisper.cpp (ADR-001) |
| `reqwest` (rustls) + `sha2` | 5 | download e verificação de modelos |
| `keyring` | 10 | Credential Manager p/ settings sensíveis futuras |
| `thiserror`, `tracing`, `serde`, `serde_json`, `dunce` | 1+ | erros, logs, serialização, canonicalização de paths Windows |

**JS/TS (npm)**
| Pacote | Fase | Justificativa |
|---|---|---|
| `react` 19, `react-dom`, `typescript` (strict), `vite` | 1 | base da UI |
| `tailwindcss` 4 + `@tailwindcss/vite` | 1 | estilo, tema escuro |
| `zustand` | 1 | estado leve sem boilerplate |
| `@tauri-apps/api` + plugins JS correspondentes | 1+ | ponte com plugins |
| `eslint`, `prettier`, `vitest`, `@testing-library/react` | 1 | qualidade exigida pelo escopo |

Regra: qualquer dependência fora desta lista precisa de justificativa no relatório da fase.

## 5. Riscos técnicos (ordenados por impacto)

1. **Qualidade/velocidade do Whisper em PT na máquina real** — large-v3-turbo em CPU pode ficar lento (>1× tempo real) dependendo do hardware. Mitigação: spike de benchmark no início da Fase 5 decide o modelo; aceleração Vulkan como plano B; trait permite trocar de engine.
2. **Latência do `claude -p`** — cold start do CLI pode levar vários segundos por geração. Mitigação: indicador de progresso honesto; fallback template; avaliar reuso de sessão (`--continue`) na Fase 6.
3. **Flags do `claude` CLI mudam entre versões** — validar na Fase 6 as flags exatas (output JSON, desabilitar tools) contra a versão instalada; encapsular em `claude_cli.rs`.
4. **Atalho global vs. apps elevados** — apps rodando como admin não recebem o atalho de um app não-elevado. Mitigação: documentar limitação; não rodar CodeVoice como admin.
5. **SmartScreen/antivírus em instalador não assinado** — aviso ao instalar. Mitigação: documentar; assinatura de código é decisão futura (custo).
6. **Conflito de atalho global** com outros apps — captura de erro no registro do atalho + UI para reconfigurar (Fase 4).
7. **Dispositivos de áudio instáveis** (troca de mic durante gravação, mic desconectado) — tratar erro de stream do cpal com mensagem clara (Fase 4).
8. **Tamanho do modelo (~1.6 GB)** — download on-demand com progresso e retomada; opção `small` (~466 MB) para máquinas modestas.

## 6. Pontos que ainda precisam de decisão do Jorge

Todas as pendências originais foram resolvidas em 22/07/2026 (ver §3). Resta em aberto apenas:

- Assinatura de código do instalador (custo anual) — pode ficar para depois do MVP.
- Confirmar se quer ChatGPT/OpenAI como provedor de geração já na Fase 6 (padrão atual: não, fica documentado como opção B plugável — ver ADR-002 em §3).

## 7. Critérios para iniciar a Fase 1

Ver MASTER-PLAN.md §4. Resumo: validar as decisões pendentes e autorizar o scaffold. Comandos previstos na Fase 1:

```
npm create tauri-app@latest   # template react-ts, gerenciador npm
npm i -D tailwindcss @tailwindcss/vite eslint prettier vitest @testing-library/react
cargo add tauri-plugin-single-instance tauri-plugin-log ... (no src-tauri)
npm run tauri dev             # validação
```
