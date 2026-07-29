# E-mails Transacionais

> Modo: `new_feature` · Área: backend
> Uso: enviar e-mails disparados por eventos do sistema (boas-vindas, reset de senha, confirmação) de forma confiável.

---

Implemente envio de e-mails transacionais no projeto **[nome do projeto]**.

## Escopo

<<SUA FALA>>

## Inventário de e-mails (preencha um por linha)

| E-mail | Gatilho (evento) | Dados dinâmicos |
|---|---|---|
| [boas-vindas] | [cadastro concluído] | [nome, link de ativação] |
| [reset de senha] | [solicitação de reset] | [nome, link com token e expiração] |

## Requisitos técnicos

1. **Provedor** [Resend/SendGrid/SES] atrás de uma **interface própria** (`enviarEmail(template, destinatario, dados)`) — trocar de provedor depois vira troca de adapter, não caça por todo o código.
2. **Envio assíncrono via fila**: o evento de negócio enfileira e segue; a chamada ao provedor roda em background. Cadastro nunca pode falhar porque o provedor de e-mail está fora.
3. **Retry com backoff exponencial** ([N] tentativas) para falha transitória; esgotou → falha registrada em log com motivo, visível para operação. E-mail perdido em silêncio é bug invisível.
4. **Idempotência**: o mesmo evento processado duas vezes ([retry da fila, webhook duplicado]) não dispara e-mail duplicado — use chave de deduplicação por [evento + destinatário].
5. **Templates HTML responsivos** com variáveis + **versão texto puro** de cada um; variáveis escapadas ao renderizar (dado vindo do usuário dentro do HTML é vetor de injeção). Layout base único com [logo/rodapé] para não duplicar.
6. **Log de envios**: destinatário, template, evento de origem, status e id da mensagem no provedor — sem isso, "não recebi o e-mail" é indebugável.
7. **Ambiente de dev/teste não envia e-mail real**: use [sandbox do provedor/catch-all/Mailpit] — disparo acidental para base real é incidente.

## Entregabilidade

- Configure **SPF, DKIM e DMARC** no domínio remetente antes de ir a produção — sem isso, provedores grandes jogam direto no spam.
- Envie de [subdomínio dedicado, ex.: mail.dominio.com] para isolar a reputação do domínio principal.
- Inclua **link de unsubscribe** nos e-mails não críticos ([resumos, novidades]); transacional puro (reset de senha, confirmação de pedido) não leva.

## Critérios de aceitação

- [ ] Cada e-mail do inventário dispara no gatilho certo com os dados certos (teste por e-mail)
- [ ] Provedor fora do ar: fluxo de negócio conclui, e-mail entra em retry, falha final fica logada
- [ ] Evento duplicado não gera e-mail duplicado (teste de idempotência)
- [ ] Templates renderizam em [cliente desktop e mobile] com versão texto presente
- [ ] SPF/DKIM/DMARC validados no domínio de envio ([ferramenta de verificação])
- [ ] Ambiente de dev comprovadamente não alcança destinatários reais

## Formato do relatório final

Tabela e-mail → gatilho → template, provedor escolhido e onde fica o adapter, política de retry, como consultar o log de envios, e o status da configuração DNS (SPF/DKIM/DMARC).
