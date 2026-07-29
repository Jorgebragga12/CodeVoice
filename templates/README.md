# Biblioteca de modelos de prompt do CodeVoice

Modelos prontos para gerar prompts **sem depender do Claude CLI**. Cada arquivo é um
esqueleto de prompt: você escolhe o modelo, fala (ou cola) o pedido específico no lugar
do marcador `<<SUA FALA>>`, ajusta os campos entre `[colchetes]` e copia o resultado.

## Convenções

- `<<SUA FALA>>` — onde entra a transcrição da sua voz (o pedido específico).
- `[campo]` — valor que você preenche manualmente (nome do projeto, stack, prazo…).
- **Modo** — um dos 10 modos do CodeVoice (`quick`, `technical`, `new_feature`, `bug_fix`,
  `refactor`, `planning`, `code_review`, `ui_creation`, `db_change`, `clean_transcript`),
  para quando estes modelos forem importados na tabela `prompt_templates` (Fase 7).
- Modelos de tecnologia falam com um **agente de programação** (Claude Code ou similar).
  Modelos das demais áreas servem para qualquer LLM ou até como checklist manual.

## Índice (105 modelos)

### Desenvolvimento (`desenvolvimento/`)
| Modelo | Modo | Quando usar |
|---|---|---|
| [nova-funcionalidade](desenvolvimento/nova-funcionalidade.md) | new_feature | Implementar algo novo de ponta a ponta |
| [correcao-de-bug](desenvolvimento/correcao-de-bug.md) | bug_fix | Investigar e corrigir um erro |
| [refatoracao-segura](desenvolvimento/refatoracao-segura.md) | refactor | Melhorar código sem mudar comportamento |
| [api-rest-endpoint](desenvolvimento/api-rest-endpoint.md) | new_feature | Criar/alterar endpoint de API |
| [integracao-api-externa](desenvolvimento/integracao-api-externa.md) | new_feature | Consumir serviço de terceiros |
| [crud-completo](desenvolvimento/crud-completo.md) | new_feature | Cadastro completo de uma entidade |
| [autenticacao-login](desenvolvimento/autenticacao-login.md) | new_feature | Login, sessão, permissões |
| [ferramenta-cli](desenvolvimento/ferramenta-cli.md) | new_feature | Criar utilitário de linha de comando |

### Frontend e UI (`frontend-ui/`)
| Modelo | Modo | Quando usar |
|---|---|---|
| [tela-nova](frontend-ui/tela-nova.md) | ui_creation | Construir uma tela/página nova |
| [componente-reutilizavel](frontend-ui/componente-reutilizavel.md) | ui_creation | Componente isolado e testável |
| [formulario-validacao](frontend-ui/formulario-validacao.md) | ui_creation | Formulário com validação e erros |
| [dashboard-graficos](frontend-ui/dashboard-graficos.md) | ui_creation | Painel com métricas e gráficos |
| [landing-page](frontend-ui/landing-page.md) | ui_creation | Página de venda/captura |

### Qualidade e segurança (`qualidade-seguranca/`)
| Modelo | Modo | Quando usar |
|---|---|---|
| [suite-de-testes](qualidade-seguranca/suite-de-testes.md) | technical | Criar/reforçar testes |
| [revisao-de-codigo](qualidade-seguranca/revisao-de-codigo.md) | code_review | Revisar mudanças em busca de defeitos |
| [auditoria-seguranca](qualidade-seguranca/auditoria-seguranca.md) | code_review | Varredura de vulnerabilidades |
| [otimizacao-performance](qualidade-seguranca/otimizacao-performance.md) | technical | Deixar algo mais rápido, com medição |
| [acessibilidade](qualidade-seguranca/acessibilidade.md) | code_review | Garantir a11y em telas existentes |

### DevOps e infra (`devops-infra/`)
| Modelo | Modo | Quando usar |
|---|---|---|
| [pipeline-ci-cd](devops-infra/pipeline-ci-cd.md) | technical | Automatizar build/teste/deploy |
| [dockerizacao](devops-infra/dockerizacao.md) | technical | Containerizar a aplicação |
| [deploy-producao](devops-infra/deploy-producao.md) | planning | Planejar e executar um deploy |
| [observabilidade-logs](devops-infra/observabilidade-logs.md) | technical | Logs, métricas e alertas |
| [atualizacao-dependencias](devops-infra/atualizacao-dependencias.md) | technical | Upgrade seguro de libs |

### Dados e IA (`dados-ia/`)
| Modelo | Modo | Quando usar |
|---|---|---|
| [analise-de-dados](dados-ia/analise-de-dados.md) | technical | Explorar dados e responder pergunta |
| [consulta-sql](dados-ia/consulta-sql.md) | quick | Escrever/otimizar SQL |
| [migracao-banco](dados-ia/migracao-banco.md) | db_change | Alterar esquema com migration |
| [chatbot-ia](dados-ia/chatbot-ia.md) | new_feature | Integrar LLM num produto |
| [automacao-n8n](dados-ia/automacao-n8n.md) | technical | Workflow de automação |

### Escrita e conteúdo (`escrita-conteudo/`)
| Modelo | Modo | Quando usar |
|---|---|---|
| [email-profissional](escrita-conteudo/email-profissional.md) | quick | E-mail claro e objetivo |
| [post-blog](escrita-conteudo/post-blog.md) | quick | Artigo com SEO básico |
| [post-redes-sociais](escrita-conteudo/post-redes-sociais.md) | quick | Conteúdo para Instagram/LinkedIn/X |
| [roteiro-video](escrita-conteudo/roteiro-video.md) | quick | Roteiro para YouTube/Reels |
| [newsletter](escrita-conteudo/newsletter.md) | quick | Edição de newsletter |

### Negócios e produto (`negocios-produto/`)
| Modelo | Modo | Quando usar |
|---|---|---|
| [proposta-comercial](negocios-produto/proposta-comercial.md) | quick | Orçamento/proposta para cliente |
| [especificacao-prd](negocios-produto/especificacao-prd.md) | planning | Transformar ideia em spec |
| [analise-concorrencia](negocios-produto/analise-concorrencia.md) | quick | Comparar concorrentes |
| [ata-de-reuniao](negocios-produto/ata-de-reuniao.md) | quick | Estruturar reunião gravada/falada |
| [descricao-de-vaga](negocios-produto/descricao-de-vaga.md) | quick | Anúncio de vaga |

### Pessoal e aprendizado (`pessoal-aprendizado/`)
| Modelo | Modo | Quando usar |
|---|---|---|
| [plano-de-estudos](pessoal-aprendizado/plano-de-estudos.md) | planning | Aprender um assunto com prazo |
| [resumo-de-texto](pessoal-aprendizado/resumo-de-texto.md) | quick | Resumir documento/artigo |
| [apresentacao-slides](pessoal-aprendizado/apresentacao-slides.md) | quick | Estruturar uma apresentação |
| [revisao-de-texto](pessoal-aprendizado/revisao-de-texto.md) | quick | Revisar/melhorar um texto seu |

### Arquitetura e design (`arquitetura-design/`)
| Modelo | Modo | Quando usar |
|---|---|---|
| [decisao-arquitetura-adr](arquitetura-design/decisao-arquitetura-adr.md) | planning | Registrar decisão de arquitetura com trade-offs |
| [design-de-sistema](arquitetura-design/design-de-sistema.md) | planning | Desenhar sistema novo antes de codar |
| [modelagem-dominio](arquitetura-design/modelagem-dominio.md) | planning | Modelar entidades e regras antes do código |
| [design-de-api-contrato](arquitetura-design/design-de-api-contrato.md) | planning | Desenhar contrato de API antes de implementar |
| [arquitetura-eventos-mensageria](arquitetura-design/arquitetura-eventos-mensageria.md) | technical | Introduzir fila ou eventos entre serviços |
| [migracao-monolito-servicos](arquitetura-design/migracao-monolito-servicos.md) | planning | Planejar extração de serviço do monolito |
| [multi-tenancy](arquitetura-design/multi-tenancy.md) | planning | Projetar isolamento de dados entre tenants |
| [setup-projeto-do-zero](arquitetura-design/setup-projeto-do-zero.md) | planning | Criar projeto novo com stack e tooling |

### Backend avançado (`backend-avancado/`)
| Modelo | Modo | Quando usar |
|---|---|---|
| [fila-background-jobs](backend-avancado/fila-background-jobs.md) | technical | Mover trabalho lento para fila em background |
| [cache-estrategia](backend-avancado/cache-estrategia.md) | technical | Introduzir cache sem bugs de dado velho |
| [websockets-tempo-real](backend-avancado/websockets-tempo-real.md) | new_feature | Atualização em tempo real no produto |
| [upload-de-arquivos](backend-avancado/upload-de-arquivos.md) | new_feature | Upload de arquivos com segurança |
| [integracao-pagamentos](backend-avancado/integracao-pagamentos.md) | new_feature | Integrar gateway de pagamento sem risco |
| [webhooks-emissao](backend-avancado/webhooks-emissao.md) | new_feature | Emitir webhooks confiáveis para terceiros |
| [busca-full-text](backend-avancado/busca-full-text.md) | new_feature | Busca de texto no produto |
| [rate-limiting](backend-avancado/rate-limiting.md) | technical | Limitar taxa de requisições da API |
| [autorizacao-permissoes](backend-avancado/autorizacao-permissoes.md) | new_feature | Papéis e permissões (RBAC) com enforcement real |
| [emails-transacionais](backend-avancado/emails-transacionais.md) | new_feature | E-mails disparados por eventos do sistema |
| [exportacao-relatorios](backend-avancado/exportacao-relatorios.md) | new_feature | Gerar PDF/Excel/CSV sem estourar memória |
| [importacao-dados-massa](backend-avancado/importacao-dados-massa.md) | new_feature | Importar planilhas com validação e sem duplicar |

### Mobile e desktop (`mobile-desktop/`)
| Modelo | Modo | Quando usar |
|---|---|---|
| [app-mobile-novo](mobile-desktop/app-mobile-novo.md) | new_feature | Iniciar app mobile do zero |
| [publicacao-loja-apps](mobile-desktop/publicacao-loja-apps.md) | planning | Planejar envio para App Store e Play |
| [offline-first-sincronizacao](mobile-desktop/offline-first-sincronizacao.md) | technical | Dados offline com sincronização |
| [push-notifications](mobile-desktop/push-notifications.md) | new_feature | Push notifications com deep link |
| [app-desktop](mobile-desktop/app-desktop.md) | new_feature | App desktop (Tauri/Electron) |
| [pwa](mobile-desktop/pwa.md) | technical | Transformar web app em PWA instalável |

### Testes avançados (`testes-avancados/`)
| Modelo | Modo | Quando usar |
|---|---|---|
| [testes-e2e](testes-avancados/testes-e2e.md) | technical | Suíte E2E dos fluxos críticos |
| [testes-integracao](testes-avancados/testes-integracao.md) | technical | Testar contra banco e dependências reais |
| [testes-carga](testes-avancados/testes-carga.md) | technical | Medir desempenho sob carga realista |
| [tdd-nova-funcionalidade](testes-avancados/tdd-nova-funcionalidade.md) | new_feature | Implementar funcionalidade nova com TDD |
| [property-based-testing](testes-avancados/property-based-testing.md) | technical | Testar invariantes com entradas geradas |
| [qualidade-da-suite-mutation](testes-avancados/qualidade-da-suite-mutation.md) | technical | Medir se a suíte detecta defeitos |

### Manutenção e legado (`manutencao-legado/`)
| Modelo | Modo | Quando usar |
|---|---|---|
| [entender-codebase-legado](manutencao-legado/entender-codebase-legado.md) | planning | Mapear código desconhecido sem alterar nada |
| [migracao-framework-versao](manutencao-legado/migracao-framework-versao.md) | planning | Migração major de framework ou linguagem |
| [auditoria-divida-tecnica](manutencao-legado/auditoria-divida-tecnica.md) | code_review | Inventariar e priorizar dívida técnica |
| [remover-codigo-morto](manutencao-legado/remover-codigo-morto.md) | refactor | Detectar e remover código sem uso |
| [debugging-regressao](manutencao-legado/debugging-regressao.md) | bug_fix | Achar o commit que quebrou (git bisect) |
| [modernizacao-strangler](manutencao-legado/modernizacao-strangler.md) | refactor | Substituir módulo legado aos poucos |
| [monorepo-organizacao](manutencao-legado/monorepo-organizacao.md) | technical | Estruturar monorepo com CI seletivo |

### Documentação e processo (`documentacao-processo/`)
| Modelo | Modo | Quando usar |
|---|---|---|
| [readme-projeto](documentacao-processo/readme-projeto.md) | quick | README que roda do zero |
| [documentacao-de-api](documentacao-processo/documentacao-de-api.md) | technical | Documentar API para consumidores externos |
| [changelog-release-notes](documentacao-processo/changelog-release-notes.md) | quick | Changelog a partir de commits |
| [descricao-pull-request](documentacao-processo/descricao-pull-request.md) | quick | Descrição de PR que acelera a revisão |
| [guia-onboarding-dev](documentacao-processo/guia-onboarding-dev.md) | quick | Guia para dev novo no projeto |
| [runbook-operacional](documentacao-processo/runbook-operacional.md) | technical | Runbook de plantão sem ambiguidade |
| [postmortem-incidente](documentacao-processo/postmortem-incidente.md) | quick | Postmortem sem culpados |
| [plano-de-implementacao](documentacao-processo/plano-de-implementacao.md) | planning | Quebrar pedido grande em plano aprovável |

### Engenharia de IA (`ia-engenharia/`)
| Modelo | Modo | Quando usar |
|---|---|---|
| [rag-busca-semantica](ia-engenharia/rag-busca-semantica.md) | new_feature | RAG: busca com recuperação sobre documentos |
| [servidor-mcp](ia-engenharia/servidor-mcp.md) | new_feature | Expor ferramentas do projeto para agentes |
| [agente-ia-ferramentas](ia-engenharia/agente-ia-ferramentas.md) | new_feature | Agente com loop de tool use |
| [escolha-modelo-provedor](ia-engenharia/escolha-modelo-provedor.md) | planning | Decidir modelo e provedor com números |
| [evals-qualidade-ia](ia-engenharia/evals-qualidade-ia.md) | technical | Criar evals antes de otimizar prompt |
| [prompt-de-sistema](ia-engenharia/prompt-de-sistema.md) | quick | Prompt de sistema robusto a injeção |
| [pipeline-dados-ml](ia-engenharia/pipeline-dados-ml.md) | technical | Pipeline de dados confiável e reprocessável |
| [extracao-dados-documentos](ia-engenharia/extracao-dados-documentos.md) | new_feature | Extrair dados estruturados de documentos (OCR + LLM) |

### Especialidades (`especialidades/`)
| Modelo | Modo | Quando usar |
|---|---|---|
| [web-scraping](especialidades/web-scraping.md) | technical | Coletar dados de sites sem API oficial |
| [extensao-navegador](especialidades/extensao-navegador.md) | new_feature | Extensão Chrome/Firefox (Manifest V3) |
| [bot-chat](especialidades/bot-chat.md) | new_feature | Bot para Discord, Telegram ou WhatsApp |
| [biblioteca-open-source](especialidades/biblioteca-open-source.md) | new_feature | Publicar lib no npm, PyPI ou crates |
| [graphql-api](especialidades/graphql-api.md) | new_feature | API GraphQL segura e sem N+1 |
| [internacionalizacao-i18n](especialidades/internacionalizacao-i18n.md) | technical | Preparar o app para múltiplos idiomas |
| [feature-flags](especialidades/feature-flags.md) | technical | Rollout gradual com kill switch |
| [seo-tecnico](especialidades/seo-tecnico.md) | technical | Indexação e Core Web Vitals |

## Como isso corta o uso do Claude

1. **Hoje (manual)**: abra o modelo, cole sua transcrição no lugar de `<<SUA FALA>>`,
   preencha os `[campos]` e copie — zero chamadas de LLM.
2. **Fase 7 (integrado)**: importar estes arquivos para `prompt_templates` e usar
   "modelo salvo como base de geração"; o `TemplateGenerator` determinístico injeta a
   transcrição e o contexto do projeto sem chamar o CLI.
