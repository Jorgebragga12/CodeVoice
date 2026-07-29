# Eventos e mensageria

> Modo: `technical` · Área: arquitetura
> Uso: introduzir fila ou eventos entre serviços/módulos com consumo confiável.

---

Implemente comunicação por eventos no projeto **[nome do projeto]**, usando **[broker — ex.: RabbitMQ, Kafka, SQS, ou "avalie e proponha"]**.

## O que precisa ser comunicado

<<SUA FALA>>

## Requisitos técnicos

1. **Schema de evento explícito e versionado**: nome, versão, id único, timestamp e payload documentados em [formato — JSON Schema, Avro, etc.]. Mudança incompatível gera versão nova do evento — nunca mutação silenciosa do schema, porque consumidor antigo quebra sem aviso.
2. **Consumidor idempotente — a mensagem VAI chegar duplicada** (retry do broker, reconexão, reprocessamento manual). Deduplique pelo id do evento ou torne a operação naturalmente idempotente: processar duas vezes não pode cobrar duas vezes.
3. **Não dependa de ordem entre eventos**, salvo garantia explícita configurada no broker (ex.: partição por chave). Se a ordem importa em algum fluxo, aponte onde e como será garantida.
4. **Retry com backoff exponencial e limite de tentativas.** Esgotado o limite, a mensagem vai para a **dead-letter queue** — nunca é descartada em silêncio; erro engolido é bug invisível.
5. **DLQ com alarme**: mensagem chegando na DLQ dispara alerta em [canal de alerta]. Documente o procedimento de análise e reprocessamento.
6. **Observabilidade do fluxo**: log estruturado com o id do evento no produtor e no consumidor (rastreio ponta a ponta), métricas de lag/profundidade de fila e taxa de erro.
7. Publicação junto com escrita no banco usa [outbox transacional — ou justifique por que não precisa]: publicar fora da transação perde evento quando o processo cai no meio.

## Critérios de aceitação

- [ ] Schema dos eventos documentado e versionado no repositório.
- [ ] Teste provando idempotência: consumir o mesmo evento duas vezes produz o mesmo estado final.
- [ ] Teste do caminho de falha: após [N] tentativas com backoff, a mensagem chega à DLQ e o alarme dispara.
- [ ] Nenhum caminho de código descarta mensagem sem log e métrica.
- [ ] Suíte completa passando: [comando de teste].

## Formato do relatório final

Eventos criados (nome, versão, schema), broker e configuração usados, como a idempotência foi garantida, fluxo de retry/DLQ implementado, e o que ficou pendente de decisão ou de infra.
