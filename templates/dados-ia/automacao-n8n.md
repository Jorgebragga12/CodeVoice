# Automação de workflow (n8n/Zapier/Make)

> Modo: `technical` · Área: automação
> Uso: automatizar um processo repetitivo conectando serviços.

---

Crie uma automação em **[plataforma: n8n/Zapier/Make]**.

## Objetivo

<<SUA FALA>>

## Descreva o fluxo em etapas

- **Gatilho**: [o que inicia — webhook, agendamento, e-mail novo, linha em planilha…]
- **Etapas**: [o que acontece com o dado em cada passo]
- **Saída**: [onde o resultado termina — mensagem, planilha, API, e-mail]

## Requisitos técnicos

- Credenciais dos serviços pelo cofre da plataforma, nunca em campo de texto de um nó.
- **Tratamento de erro em cada etapa crítica**: o que acontece se a API do meio do fluxo falhar? (retry, caminho de erro, notificação — não falha silenciosa).
- Idempotência: se o gatilho disparar duas vezes com o mesmo dado, o resultado não pode duplicar [pedido/mensagem/linha].
- Dados sensíveis: não trafegar mais campos do que o necessário entre serviços.
- Volume esperado: [X execuções/dia] — confirme que cabe nos limites do plano/rate limits das APIs.

## Validações

- Executar com dado de teste real e mostrar o resultado em cada nó.
- Testar o caminho de erro de propósito (ex.: serviço de destino indisponível) e mostrar o comportamento.

## Critérios de aceitação

- [ ] Fluxo completo funcionando com dado real de teste.
- [ ] Falha em etapa intermediária não perde o dado nem passa despercebida.

## Formato do relatório final

Diagrama/lista do fluxo final, o que preciso conectar/autorizar, e como monitorar se a automação parar de funcionar.
