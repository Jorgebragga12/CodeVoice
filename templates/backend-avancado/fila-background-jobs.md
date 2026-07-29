# Fila e background jobs

> Modo: `technical` · Área: backend
> Uso: mover trabalho pesado ou lento para processamento em background com fila.

---

Implemente processamento em background no projeto **[nome do projeto]**.

## Objetivo

<<SUA FALA>>

## Contexto a preencher

- Infra de fila: [a que o projeto já usa, ou a mais simples que atende — ex.: Redis/BullMQ, SQS, RabbitMQ, tabela no próprio banco]
- Jobs a criar: [lista de jobs e o que cada um faz]
- Volume esperado: [jobs por minuto/hora, aproximado]

## Regras de implementação

1. **Todo job é idempotente.** Assuma que ele VAI executar duas vezes (retry, redeploy, duplicação da fila); a segunda execução não pode duplicar e-mail, cobrança ou registro.
2. **Retry com backoff exponencial e limite de tentativas** ([N] tentativas). Retry infinito é proibido.
3. **Esgotou as tentativas → dead-letter visível, com alarme/notificação.** Job que falha silenciosamente é proibido: toda falha final gera log com payload e erro.
4. **Timeout por job** ([valor]): job travado é encerrado e contado como falha, não fica pendurado para sempre segurando um worker.
5. Status do job (pendente/executando/concluído/falhou) fica registrado e consultável — é isso que alimenta a UI e o reprocessamento.

## Experiência do usuário

- Defina o que o usuário vê enquanto o job processa: [indicador de status, notificação ao concluir, e-mail...]. "Nada acontece na tela" não é resposta aceitável.

## Reprocessamento manual

- Deve existir um caminho documentado para reprocessar job da dead-letter ([comando, endpoint admin ou script]): como listar, inspecionar o payload e reenfileirar.

## Critérios de aceitação

- [ ] Job executado duas vezes com o mesmo payload produz o mesmo resultado (teste provando)
- [ ] Falha transitória gera retry com backoff; falha permanente vai para a dead-letter com alarme
- [ ] Job que estoura o timeout é encerrado e marcado como falha
- [ ] A request principal responde sem esperar o job terminar
- [ ] Reprocessamento manual documentado e testado

## Formato do relatório final

Infra escolhida e por quê, jobs criados, configuração de retry/dead-letter/timeout, e o passo a passo do reprocessamento manual.
