# Avaliação de feature de IA (evals)

> Modo: `technical` · Área: engenharia de IA
> Uso: criar avaliação automatizada (evals) para uma feature com LLM, antes de otimizar prompt ou trocar modelo.

---

Crie um conjunto de evals para a feature de IA descrita abaixo, no projeto **[nome do projeto]**.

## Feature a avaliar

<<SUA FALA>>

## Contexto a preencher

- Onde a feature vive no código: [arquivo/módulo]
- Tipo de tarefa: [extração / classificação / geração de texto / agente]
- Fonte dos casos reais: [logs de produção / exemplos manuais / dados de teste]

## Regras de construção

1. **Casos reais com resultado esperado**: [nº] exemplos representativos, incluindo casos de borda e adversariais (entrada vazia, fora do escopo, tentativa de injeção, ambígua). Caso sem gabarito não é eval.
2. **Métrica por tipo de tarefa**: exatidão/F1 para extração e classificação; rubrica com critérios explícitos para geração; LLM-como-juiz só com validação humana amostral — confira [nº] vereditos do juiz contra julgamento humano antes de confiar nele.
3. **Baseline antes de otimizar**: rode o eval no estado atual e registre o número — sem baseline, "melhorou" é achismo.
4. Eval roda por comando único ([comando]) e reporta resultado por caso + agregado; roda a cada mudança de prompt ou modelo, como teste de regressão.
5. Falha de eval mostra entrada, saída obtida e saída esperada lado a lado — eval que só diz "falhou" não ajuda a corrigir.
6. **Regra da casa: mudança de prompt sem eval é chute com estilo.** Nenhum ajuste entra sem número antes/depois.

## Critérios de aceitação

- [ ] Conjunto de casos versionado no repositório, com gabarito.
- [ ] Casos adversariais e de borda incluídos.
- [ ] Baseline registrado com data e versão do prompt/modelo.
- [ ] Eval roda com um comando e reporta por caso + agregado.
- [ ] Documentado como adicionar um caso novo ao conjunto.

## Formato do relatório final

Nº de casos por categoria, métrica escolhida (e por quê), número do baseline e como rodar o eval.
