# Emissão de webhooks

> Modo: `new_feature` · Área: backend
> Uso: emitir webhooks para sistemas de terceiros de forma confiável e verificável.

---

Implemente emissão de webhooks no projeto **[nome do projeto]**.

## Escopo

<<SUA FALA>>

## Requisitos técnicos

1. **Assinatura HMAC** ([SHA-256]) do payload com **segredo único por assinante**, enviada em header ([X-Signature]) — sem isso o consumidor não tem como saber que o webhook é legítimo. Inclua timestamp na base assinada para bloquear replay.
2. **Entrega via fila própria, fora do fluxo principal**: o evento de negócio enfileira e a request responde; a chamada HTTP ao terceiro acontece em background. Endpoint lento de terceiro nunca pode travar o seu fluxo.
3. **Timeout curto na entrega** ([5–10s]); só resposta 2xx conta como sucesso.
4. **Retry com backoff exponencial e limite** ([N] tentativas ao longo de [período]); esgotou → falha definitiva registrada e visível, com opção de desativar endpoint que só falha.
5. **Log/painel de entregas** por assinante: evento, payload, tentativas, resposta recebida e status — com **reenvio manual** de uma entrega específica.
6. **Versionamento do payload** ([campo `version` ou versão no endpoint]): mudar o formato sem versionar quebra todos os consumidores de uma vez.

## Documentação para o consumidor

- Documente: eventos disponíveis, formato do payload por versão, headers enviados, e o **passo a passo de verificação da assinatura com exemplo de código** — assinatura que ninguém consegue verificar não protege ninguém.
- Recomende ao consumidor responder 2xx rápido e processar de forma assíncrona.

## Critérios de aceitação

- [ ] Assinatura HMAC com segredo por assinante, verificável seguindo apenas a documentação
- [ ] Emissão não bloqueia o fluxo principal (teste com endpoint lento/fora do ar)
- [ ] Falha na entrega gera retries com backoff até o limite; depois, falha definitiva registrada
- [ ] Painel/log lista as entregas com status e permite reenvio manual
- [ ] Payload carrega versão explícita
- [ ] Nenhuma entrega falha sem deixar rastro em log

## Formato do relatório final

Eventos emitidos, formato do payload, política de timeout/retry, como consultar o log de entregas e reenviar, e o trecho da documentação do consumidor.
