# Runbook operacional

> Modo: `technical` · Área: documentação
> Uso: escrever runbook de plantão de um serviço — diagnóstico e ação sem ambiguidade.

---

Escreva o runbook operacional do serviço **[nome do serviço]** do projeto **[nome do projeto]**.

## Escopo e cenários a cobrir

<<SUA FALA>>

## Contexto a preencher

- Onde roda: [infra — ex.: k8s, VM, PaaS]
- Observabilidade: [dashboards/ferramentas, com links]
- Alertas configurados: [lista ou onde encontrá-los]

## Estrutura obrigatória

1. **Saúde em 1 minuto:** quais dashboards/comandos abrir e o que é NORMAL (faixas de latência, taxa de erro, fila) — sem baseline documentado, o plantonista não sabe se o gráfico está ruim.
2. **Um bloco por alerta possível:** o que significa, como confirmar o diagnóstico (comando/consulta exata), ação passo a passo, e o que fazer se a ação não resolver. Alerta sem ação documentada não deveria existir — liste os que estão nessa situação.
3. **Procedimentos comuns** com comandos exatos e copiáveis: restart, rollback, escalar. Nunca "reinicie o serviço" — sempre o comando completo, com placeholders explícitos ([namespace], [release]).
4. **Passos destrutivos** (rollback, kill, limpeza de fila) marcados com aviso e exigindo confirmação explícita antes de executar.
5. **Escalonamento:** quem acionar, quando (critério objetivo — ex.: "após [X min] sem mitigação") e por qual canal.
6. **Acessos necessários:** o que o plantonista precisa ter ANTES do incidente (VPN, permissões, ferramentas) — descobrir falta de acesso às 3h é o pior momento.

## Regra de ouro

Escrito para alguém às 3h da manhã sob pressão: zero ambiguidade, zero "veja com o time", toda decisão com critério objetivo e todo comando copiável.

## Critérios de aceitação

- [ ] Todo alerta listado tem diagnóstico e ação.
- [ ] Todos os comandos foram validados contra a infra real (executados ou conferidos) e são copiáveis.
- [ ] Passos destrutivos têm aviso e exigência de confirmação prévia.
- [ ] Baselines de "normal" preenchidos com valores reais ou marcados [medir].

## Formato do relatório final

Alertas cobertos, comandos validados vs. não validados, alertas sem ação possível encontrados, e lacunas de acesso ou observabilidade identificadas.
