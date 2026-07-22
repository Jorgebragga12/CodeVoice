# CodeVoice — PRODUCT-SPEC

> Versão 0.1 · Fase 0 · 22/07/2026
> Status: fundação aprovada, aguardando início da Fase 1.

## 1. Visão

CodeVoice é um aplicativo desktop para Windows que transforma fala desorganizada — ideias, bugs, pedidos de funcionalidade ditos em voz alta — em **prompts técnicos estruturados** prontos para uso no Claude Code. É uma ferramenta de ditado especializada para desenvolvedores: fala-se livremente, o app transcreve localmente (Whisper) e reescreve o conteúdo como um prompt de engenharia com objetivo, contexto do projeto, requisitos, restrições e critérios de aceitação.

**Não é**: um assistente de código, um editor, um cliente de chat, nem um wrapper do Claude Code. Ele produz o *prompt*; a execução continua sendo do Claude Code no terminal.

## 2. Usuário-alvo

Desenvolvedor(a) que usa Claude Code diariamente no Windows, alterna entre vários projetos locais e prefere falar a digitar prompts longos. Uso frequente (dezenas de vezes por dia), sessões curtas (30s–3min de fala).

## 3. Fluxo principal (caminho feliz)

1. Usuário seleciona um projeto cadastrado (ou o último usado já vem selecionado).
2. Pressiona o **atalho global** (padrão sugerido: `Ctrl+Shift+Space`, configurável).
3. A **janela compacta de gravação** aparece (sempre no topo, sem borda): indicador visual + contador de duração + nome do projeto ativo.
4. Fala livremente. `Esc` cancela e descarta o áudio.
5. Pressiona o atalho novamente para encerrar.
6. O áudio é transcrito localmente (progresso visível).
7. A transcrição é transformada em prompt no **modo** selecionado (ver §5).
8. A janela principal mostra transcrição original + prompt gerado, editáveis.
9. Usuário revisa, opcionalmente aplica ações (encurtar, detalhar, dividir em etapas…).
10. Copia para a área de transferência e/ou abre o terminal na pasta do projeto para colar no Claude Code.
11. O item é salvo no histórico. O áudio temporário é excluído (a menos que a opção "manter áudio" esteja ativada — desativada por padrão).

Meta de UX: do atalho ao prompt copiado em **menos de 6 interações** e sem tocar no mouse no caminho feliz.

## 4. Plataforma e limites do MVP

| Incluído | Excluído do MVP |
|---|---|
| Windows 10 e 11 (x64) | macOS, Linux |
| App desktop local, offline-first | Versão web, mobile |
| Sem login | Contas, sincronização em nuvem |
| Transcrição local (Whisper) | Transcrição via API (preparada arquiteturalmente, não implementada) |
| Copiar prompt + abrir terminal + colar sob ação do usuário | Integração programática profunda com Claude Code (preparada, não implementada) |
| Instalador Windows (NSIS) | Auto-update (fase futura), assinatura de código (decisão pendente) |

## 5. Funcionalidades do MVP

### 5.1 Aplicativo
- Janela principal; minimização para a bandeja do sistema; opção de iniciar com o Windows (desativada por padrão).
- Janela compacta de gravação (segunda janela, always-on-top, frameless).
- Tema escuro (único tema no MVP).
- Tratamento global de erros com mensagens acionáveis; logs locais com rotação.
- Instância única (segunda execução foca a janela existente).

### 5.2 Gravação
- Seleção de microfone (lista de dispositivos + padrão do sistema).
- Iniciar/parar por botão ou atalho global configurável; cancelar com `Esc`.
- Indicador visual de gravação + contador de duração; limite de segurança de 10 min por gravação.
- Áudio temporário WAV 16 kHz mono, gravado em diretório do app; exclusão automática após processamento; opção "manter áudio" desativada por padrão.

### 5.3 Transcrição
- Local, via whisper.cpp (binding `whisper-rs`); português como idioma principal com reconhecimento de termos técnicos em inglês (nomes de arquivos, comandos, tecnologias devem ser preservados — reforçado por prompt inicial do Whisper e pós-processamento).
- Download do modelo on-demand no primeiro uso, com progresso e verificação de integridade (SHA-256).
- Exibição de progresso da transcrição.
- Erros tratados: silêncio/áudio vazio, modelo ausente/corrompido, áudio inválido, falha de processamento — todos com mensagem clara e opção de repetir.

### 5.4 Gerador de prompts — modos
1. **Transcrição limpa** — só remove hesitações e pontua (sem LLM se possível).
2. **Prompt rápido** — 1–3 parágrafos diretos.
3. **Prompt técnico** — estrutura completa (ver abaixo).
4. **Nova funcionalidade**
5. **Correção de erro**
6. **Refatoração**
7. **Planejamento**
8. **Revisão de código**
9. **Criação de interface**
10. **Alteração de banco de dados**

O **prompt técnico** (e os modos 4–10, que são especializações dele) pode conter as seções: objetivo; contexto do projeto (injetado do cadastro — stack, arquitetura, regras); requisitos funcionais; requisitos técnicos; regras; restrições; arquivos possivelmente relacionados; plano esperado; validações; critérios de aceitação; formato do relatório final. Seções vazias são omitidas.

Provedores de geração (ver ADR-002 em ARCHITECTURE.md): `claude` CLI headless (primário) e templates determinísticos (fallback offline).

### 5.5 Editor
- Painéis: transcrição original (somente leitura por padrão) e prompt gerado (editável).
- Ações: copiar; regenerar; encurtar; detalhar; deixar mais técnico; dividir em etapas; desfazer (histórico de edição em memória); salvar como modelo (template reutilizável).

### 5.6 Cadastro de projetos
Campos: nome, caminho local, descrição, stack, arquitetura, regras, comandos de desenvolvimento, comandos de teste, tecnologias proibidas, banco de dados, observações, criado em/atualizado em.

**Importação assistida** (somente com autorização explícita do usuário, por projeto): ler `CLAUDE.md`, `README.md`, `package.json`, arquivos de configuração comuns (`tsconfig.json`, `Cargo.toml`, `pyproject.toml`, etc.) e a estrutura principal de diretórios (2 níveis) para pré-preencher os campos.

**Denylist absoluta** (nunca ler nem armazenar): `.env*`, chaves/tokens/credenciais, `node_modules/`, `dist/`, `build/`, `.git/`, binários. Detalhes em SECURITY-MODEL.md.

### 5.7 Histórico
Salvo localmente por item: projeto, data, transcrição, prompt, modo, favorito, duração do áudio, status. Ações: copiar novamente, editar, favoritar, excluir (com confirmação), pesquisar (texto completo), filtrar por projeto e por modo.

### 5.8 Integração inicial com Claude Code
- Copiar prompt para a área de transferência.
- Abrir terminal (Windows Terminal se disponível; senão PowerShell) já na pasta do projeto.
- Detectar se o comando `claude` está no PATH; permitir iniciá-lo.
- Colar o prompt no terminal ativo **somente após ação explícita do usuário**.
- Proibido: executar comandos destrutivos automaticamente; usar flags que pulem confirmações de segurança do Claude Code; executar texto gerado como comando.

## 6. Requisitos não-funcionais
- **Privacidade**: processamento 100% local por padrão; nenhuma telemetria sem opt-in; nenhum secret em logs.
- **Desempenho**: janela de gravação abre em < 300 ms após o atalho; transcrição de 1 min de áudio em tempo aceitável na máquina-alvo (benchmark na Fase 5 define o modelo padrão).
- **Robustez**: nenhuma perda de gravação por crash do pipeline de transcrição (áudio persiste até o fim do processamento).
- **Qualidade**: TypeScript strict, lint, formatação, testes unitários dos serviços críticos, build reproduzível, instalador Windows.

## 7. Fora de escopo declarado
Multi-idioma de UI, temas claros, plugins, transcrição em streaming (tempo real), edição de áudio, atalhos por projeto, estatísticas de uso.

## 8. Documentos relacionados
- [ARCHITECTURE.md](ARCHITECTURE.md) — arquitetura, camadas e ADRs
- [DATABASE-SCHEMA.md](DATABASE-SCHEMA.md) — esquema SQLite e migrations
- [SECURITY-MODEL.md](SECURITY-MODEL.md) — modelo de segurança
- [MASTER-PLAN.md](MASTER-PLAN.md) — fases e critérios de conclusão
- [PHASE-00-REPORT.md](PHASE-00-REPORT.md) — relatório da Fase 0
