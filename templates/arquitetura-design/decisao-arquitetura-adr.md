# Decisão de arquitetura (ADR)

> Modo: `planning` · Área: arquitetura
> Uso: registrar uma decisão de arquitetura como ADR, com trade-offs honestos e critérios de reversão.

---

Escreva um ADR (Architecture Decision Record) para o projeto **[nome do projeto]**.

## Decisão a registrar

<<SUA FALA>>

## Contexto a considerar

- Restrições atuais: [equipe, prazo, stack, orçamento — o que limita a escolha]
- Requisitos que motivam a decisão: [números de escala, SLA, compliance — se houver]
- Decisões anteriores relacionadas: [ADRs ou escolhas já feitas que esta afeta — ou "nenhuma"]

## Regras do ADR

1. **Liste no mínimo 2 alternativas além da escolhida**, cada uma com prós e contras reais. Alternativa rejeitada sem motivo concreto é espantalho — se o motivo for "ninguém no time conhece a ferramenta", escreva isso; é um motivo válido.
2. **Consequências incluem as ruins.** Toda decisão de arquitetura compra um problema para resolver outro; ADR que só lista benefícios está mentindo por omissão.
3. **Defina critérios de reversão**: qual sinal observável (métrica, custo, incidente recorrente) indicaria que a decisão foi errada, e qual seria o caminho de volta.
4. **Derive do contexto real do projeto**, não de artigo genérico. Se faltar informação para comparar as opções de verdade, pergunte antes de escrever.
5. A decisão é escrita no presente ("Adotamos X"), com data e status ([proposto / aceito / substituído]).
6. Registre quem participou da decisão: [pessoas/times] — ADR sem dono não tem autoridade depois.

## Formato da resposta

ADR em Markdown com as seções, nesta ordem:

1. **Título e status** (com data)
2. **Contexto** — o problema e as restrições
3. **Opções consideradas** — cada uma com trade-offs
4. **Decisão** — o que foi escolhido e por quê
5. **Consequências** — positivas E negativas
6. **Critérios de reversão**

Máximo de 1 página — ADR longo não é lido.
