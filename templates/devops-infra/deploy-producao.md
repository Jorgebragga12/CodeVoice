# Deploy em produção

> Modo: `planning` · Área: DevOps
> Uso: planejar (e depois executar) um deploy com segurança e rollback.

---

Planeje o deploy de **[o que]** do projeto **[nome do projeto]** para **[ambiente/plataforma]**.

## Contexto

<<SUA FALA>>

## O que o plano precisa cobrir

1. **Pré-deploy**: CI verde? Migrations pendentes? Variáveis de ambiente novas configuradas no destino? Backup do banco feito?
2. **Ordem de execução**: migrations compatíveis com a versão antiga rodando (deploy sem downtime) ou janela de manutenção necessária?
3. **Execução**: passos exatos, um por um, com o comando de cada.
4. **Verificação pós-deploy**: como confirmar que está no ar e saudável (URL de healthcheck, fluxo crítico manual, logs sem erro novo).
5. **Rollback**: gatilho objetivo ("se X falhar") e passos para voltar — incluindo o plano para desfazer migration, que é sempre a parte perigosa.

## Regras

- **Primeiro o plano, para minha aprovação. Não execute nada ainda.**
- Qualquer passo destrutivo ou irreversível deve estar destacado em negrito no plano.
- Se faltar informação (acesso, credencial, estado do ambiente), liste como pendência em vez de assumir.

## Formato do relatório final

Plano numerado com comandos, checklist pré-deploy, critérios de verificação, plano de rollback, e lista do que você precisa de mim para executar.
