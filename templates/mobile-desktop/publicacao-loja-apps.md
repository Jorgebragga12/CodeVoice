# Publicação nas lojas

> Modo: `planning` · Área: mobile/desktop
> Uso: planejar o primeiro envio de um app para App Store e Play Store sem rejeição por descuido.

---

Monte o plano de publicação do app **[nome do projeto]** na App Store e na Play Store.

## Situação atual

<<SUA FALA>>

## Contexto a preencher

- Estágio do app: [em desenvolvimento / pronto para beta / pronto para produção].
- Contas de desenvolvedor: [já existem / precisam ser criadas].
- O app coleta dados de usuário? [sim/não — quais, incluindo via SDKs de terceiros].
- Público-alvo inclui menores? [sim/não] — muda exigências das duas lojas.

## O que o plano deve cobrir

1. **Contas e taxas**: o que criar em cada loja, custo (Apple cobra taxa anual, Google taxa única) e prazo de verificação da conta — pode levar dias, não deixe para a véspera. Confirme valores atuais na fonte oficial (pode usar WebSearch, máx. 3 buscas).
2. **Assets exigidos**: ícone em todos os tamanhos, screenshots por tamanho de tela exigido, descrição curta e longa, palavras-chave. Confirme dimensões atuais nas docs oficiais em vez de chutar.
3. **Privacidade**: URL de política de privacidade acessível + formulários de dados coletados (App Privacy na Apple, Data safety no Google). Precisam bater com o que o app REALMENTE coleta, inclusive SDKs de analytics/ads — divergência é motivo de rejeição.
4. **Motivos comuns de rejeição a prevenir**: crash durante a revisão, links quebrados, funcionalidade atrás de login sem conta de teste fornecida ao revisor, permissão sem texto de justificativa, metadados prometendo o que o app não faz.
5. **Versionamento**: esquema de versão + build number que incrementa a cada envio; como gerar o build de release assinado de cada plataforma, documentado passo a passo.
6. **Faixas de teste antes de produção**: TestFlight no iOS e faixas internal/closed/open no Android — pelo menos uma rodada com testadores externos antes do envio final.

## Restrições

- Não invente valores de taxa nem prazos de revisão: confirme na fonte oficial ou marque como "[confirmar]".
- O plano deve ser executável por alguém sem experiência prévia de publicação.

## Formato da resposta

Checklist ordenado do que fazer ANTES do primeiro envio (com dependências entre itens), tabela de assets exigidos por loja, lista de riscos de rejeição específicos deste app e o que fazer se a revisão rejeitar.
