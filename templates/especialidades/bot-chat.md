# Bot de chat (Discord/Telegram/WhatsApp)

> Modo: `new_feature` · Área: especialidades
> Uso: criar bot de chat pela API oficial da plataforma, seguro e resiliente.

---

Implemente o seguinte bot para **[plataforma: Discord/Telegram/WhatsApp]** no projeto **[nome do projeto]**.

## Escopo

<<SUA FALA>>

## Requisitos técnicos

1. **API oficial da plataforma** para registro do bot e dos comandos ([lista de comandos]) — nada de automação de conta de usuário, que viola os termos e derruba a conta.
2. **Webhook vs polling com justificativa**: webhook se há URL pública estável (menor latência, exige validação); polling se roda atrás de NAT ou em desenvolvimento. Registre a escolha e o porquê.
3. **Validar que a mensagem vem mesmo da plataforma**: verifique a assinatura do webhook ([mecanismo de assinatura da plataforma]); requisição com assinatura inválida recebe 401 e é logada — sem isso, qualquer um envia comandos ao seu bot.
4. **Estado de conversa por usuário com expiração** ([tempo]): conversa abandonada não pode ocupar memória para sempre nem vazar contexto para a interação seguinte.
5. **Rate limits da plataforma respeitados**: trate a resposta de limite (429/retry_after) com espera, não com retry cego — bot que martela a API é banido.
6. **Mídia** [se aplicável]: valide tipo e tamanho antes de processar; arquivo inesperado gera resposta educada ao usuário, não crash.
7. **Comandos de ajuda decentes**: `/help` lista os comandos com exemplos; comando desconhecido responde apontando o help, nunca silêncio.
8. **Token em variável de ambiente** ([NOME_DA_VAR]) — token no código ou no repositório é incidente de segurança; o deploy documenta onde configurá-lo.

## Critérios de aceitação

- [ ] Comandos registrados pela API oficial e funcionando
- [ ] Requisição de webhook com assinatura inválida → 401 (teste) [se webhook]
- [ ] Estado de conversa expira após [tempo] (teste)
- [ ] Resposta de rate limit da plataforma tratada com espera (teste ou simulação)
- [ ] Comando desconhecido → resposta apontando o /help (teste)
- [ ] Nenhum token no código ou no histórico do repositório (verificado)

## Formato do relatório final

Escolha webhook/polling com justificativa, como a assinatura é validada, política de expiração de estado, e instruções de deploy com as variáveis de ambiente necessárias.
