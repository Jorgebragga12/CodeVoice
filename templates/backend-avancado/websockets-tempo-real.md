# Tempo real (WebSockets/SSE)

> Modo: `new_feature` · Área: backend
> Uso: adicionar atualização em tempo real (notificações, chat, dashboard ao vivo).

---

Implemente o seguinte recurso em tempo real no projeto **[nome do projeto]**.

## Escopo

<<SUA FALA>>

## Escolha do transporte

Escolha pelo caso real, não pela moda, e registre a justificativa:

- **SSE** se o fluxo é só servidor→cliente (notificações, progresso) — mais simples e passa melhor por proxies HTTP.
- **WebSocket** se precisa de bidirecional de verdade (chat, colaboração, jogo).
- **Polling** se a atualização tolera [intervalo] de atraso — às vezes é a resposta certa e a mais barata de operar.

## Requisitos técnicos

1. **Autenticação no handshake da conexão** ([token/cookie/sessão]); conexão sem auth é recusada, e a autorização é revalidada quando a sessão expira.
2. **Reconexão automática com backoff** no cliente + **recuperação do que se perdeu**: ao reconectar, o cliente informa o último evento recebido ([id/timestamp]) e o servidor reenvia o que faltou. Reconectar sem recuperar estado é dado perdido em silêncio.
3. **Heartbeat/ping-pong** para detectar conexão morta dos dois lados; conexão que não responde é fechada e o recurso liberado.
4. **Fan-out explícito**: documente quem recebe o quê ([canal por usuário, por sala, broadcast]) e o que impede um usuário de assinar canal que não é dele.
5. **Degradação visível**: com a conexão caída, a UI mostra "reconectando" e o app continua utilizável — nada de tela congelada fingindo estar ao vivo.
6. **Limite de conexões simultâneas por usuário** ([N]) para conter vazamento de conexões e abuso.

## Critérios de aceitação

- [ ] Transporte escolhido com justificativa por escrito no relatório
- [ ] Conexão sem autenticação é recusada (teste)
- [ ] Derrubar a conexão → cliente reconecta sozinho e recebe os eventos perdidos (teste)
- [ ] Usuário não recebe eventos de canal que não assina ou não pode ver (teste)
- [ ] UI indica o estado desconectado/reconectando
- [ ] Limite de conexões por usuário aplicado

## Formato do relatório final

Transporte escolhido e por quê, mapa de canais (quem recebe o quê), como funciona a recuperação pós-reconexão, e os limites configurados.
