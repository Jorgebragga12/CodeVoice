# Teste de carga

> Modo: `technical` · Área: testes
> Uso: montar e rodar teste de carga com cenário realista e critérios definidos antes.

---

Monte e execute um teste de carga com **[k6/vegeta/locust]** no projeto **[nome do projeto]**.

## Objetivo

<<SUA FALA>>

## Contexto a preencher

- Tráfego real de referência: [mix de rotas e proporção — extraia de logs/analytics, não chute]
- Ambiente do teste: [staging/réplica/produção controlada]
- Carga alvo: [RPS ou usuários simultâneos esperados + margem]

## Regras do cenário

1. **Cenário baseado em tráfego REAL.** Reproduza o mix de rotas observado (ex.: leitura pesada + escrita rara), não só a home ou um endpoint isolado — carga irreal gera conclusão irreal.
2. **Critérios de aprovação definidos ANTES de rodar:** p95 ≤ [valor], p99 ≤ [valor], taxa de erro ≤ [valor]. Sem critério prévio, qualquer resultado "parece ok".
3. **Métricas que importam:** p95/p99, taxa de erro e sinais de saturação (CPU, conexões, fila). Média é proibida como métrica de decisão — ela esconde a cauda.
4. **Rampa gradual** em degraus de carga para localizar o ponto de degradação, não "liga tudo de uma vez".
5. **Warm-up descartado.** Os primeiros [N segundos/requisições] ficam fora da análise (cache frio, JIT, pool de conexão vazio).
6. **Ambiente representativo** — ou declare explicitamente as diferenças para produção (hardware, volume de dados, rede) e o que elas invalidam na conclusão.

## Critérios de aceitação

- [ ] Critérios de aprovação registrados antes da primeira execução.
- [ ] Cenário reproduz o mix de tráfego real documentado.
- [ ] Relatório traz p95/p99 e taxa de erro por degrau de carga.
- [ ] Warm-up excluído da análise.
- [ ] Gargalo identificado (ou ausência dele até a carga alvo, com evidência).

## Formato do relatório final

Aprovado/reprovado contra os critérios, tabela de p95/p99/erros por degrau, o gargalo encontrado (componente + evidência) e a recomendação — números sem diagnóstico não encerram a tarefa.
