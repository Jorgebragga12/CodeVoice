# Notificações push

> Modo: `new_feature` · Área: mobile/desktop
> Uso: implementar push notifications completas — da permissão ao deep link — nas duas plataformas.

---

Implemente notificações push no projeto **[nome do projeto]**.

## Objetivo

<<SUA FALA>>

## Contexto a preencher

- Provedor de push: [FCM / APNs direto / serviço gerenciado].
- Tipos de notificação previstos: [lista — ex.: transacional, lembrete, marketing].
- Backend que dispara: [serviço/endpoint existente ou "criar nesta tarefa"].

## Requisitos técnicos

1. **Permissão com contexto, nunca no primeiro boot**: tela própria explicando o valor ("avise-me quando X") ANTES do diálogo do sistema, disparada por uma ação do usuário — no iOS, o diálogo nativo negado não reaparece sem ir a Ajustes.
2. **Tokens**: registre o token no backend associado a usuário+dispositivo, trate renovação (tokens mudam sem aviso) e remova o token no logout — senão o próximo usuário do aparelho recebe notificação alheia.
3. **Envio segmentado**: por usuário, por tópico/segmento e por tipo; "broadcast para todos" não pode ser o único caminho.
4. **Deep link nos 3 estados do app**: tocar na notificação abre a tela certa com o app fechado (cold start), em background e em foreground — são três fluxos de código diferentes e os três precisam de teste.
5. **Notificações silenciosas** [se aplicável]: só para acordar sync em background; não dependa de entrega garantida — o SO pode descartá-las para economizar bateria.
6. **Opt-out por tipo**: tela de preferências onde o usuário desliga cada tipo separadamente, respeitada no BACKEND antes do envio, não só escondendo no cliente.
7. Falha de registro ou entrega logada com o motivo — nunca engolida.

## Validações

- Teste em DISPOSITIVO REAL nas duas plataformas (simulador não é confiável para push): app fechado, em background e aberto, verificando o deep link em cada caso.
- Teste de opt-out: desligar um tipo e confirmar que o backend deixa de enviar.

## Critérios de aceitação

- [ ] Permissão pedida com tela de contexto, disparada por ação do usuário — nada no primeiro boot.
- [ ] Token registrado, renovado quando muda e removido no logout.
- [ ] Deep link funciona nos 3 estados do app, nas duas plataformas, testado em aparelho real.
- [ ] Opt-out por tipo funcionando de ponta a ponta (cliente + backend).
- [ ] Nenhum envio para quem optou por não receber aquele tipo.

## Formato do relatório final

Fluxo implementado (permissão → token → envio → deep link), matriz de testes executados (plataforma × estado do app × resultado) e pendências conhecidas.
