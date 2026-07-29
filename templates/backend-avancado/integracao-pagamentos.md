# Integração de pagamentos

> Modo: `new_feature` · Área: backend
> Uso: integrar gateway de pagamento sem tocar em dado de cartão nem perder cobrança.

---

Integre o gateway de pagamento **[gateway]** no projeto **[nome do projeto]**.

## Escopo

<<SUA FALA>>

## Regras inegociáveis

1. **Nenhum dado de cartão passa pelo nosso servidor ou banco.** Use a tokenização do gateway (checkout hospedado ou SDK no cliente); armazenar número/CVV coloca o sistema no escopo pesado do PCI DSS. Guarde apenas token, últimos 4 dígitos e bandeira.
2. **Webhook é a fonte da verdade do status do pagamento — nunca o redirect de retorno.** O usuário pode fechar a aba e o redirect pode nunca chegar. Valide a assinatura do webhook antes de processar qualquer evento.
3. **Webhook idempotente**: o gateway reenvia eventos; processar o mesmo evento duas vezes não pode dar baixa dupla no pedido.
4. **Chave de idempotência na criação de cobrança**: retry de rede não pode cobrar o cliente duas vezes.
5. **Valores sempre em centavos inteiros**, nunca float — float perde centavos em soma e comparação. Vale para código, banco e chamadas ao gateway.

## Estados e conciliação

- Modele os estados do pagamento ([pendente/aprovado/recusado/estornado/em disputa]) e as transições válidas entre eles; transição inválida gera erro logado.
- Rotina de **conciliação** ([diária]): comparar registros locais com o gateway e alertar divergência — pagamento aprovado lá e pendente aqui é dinheiro perdido em silêncio.
- Trate **estorno e disputa (chargeback)** recebidos por webhook: defina o que acontece com o pedido/acesso do cliente em cada caso.

## Ambiente de teste

- Toda a implementação validada no sandbox do gateway com cartões de teste ([aprovado, recusado, 3DS se houver]). Credenciais de produção nunca em código ou repositório — só em [gerenciador de segredos/variável de ambiente].

## Critérios de aceitação

- [ ] Nenhum campo de cartão trafega pelo backend (verificado nos payloads das requests)
- [ ] Status só muda via webhook com assinatura validada; assinatura inválida é rejeitada (teste)
- [ ] Mesmo webhook entregue duas vezes produz um único efeito (teste)
- [ ] Criação de cobrança com retry não duplica a cobrança (teste com chave de idempotência)
- [ ] Valores em centavos inteiros em todo o fluxo, inclusive no banco
- [ ] Fluxos de estorno e disputa implementados e exercitados no sandbox

## Formato do relatório final

Fluxo completo (checkout → webhook → estado), eventos de webhook tratados, como rodar a conciliação, e o que foi testado no sandbox.
