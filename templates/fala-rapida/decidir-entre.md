# Decidir entre opções

> Modo: `quick` · Área: fala rápida
> Uso: escolher entre duas ou três alternativas técnicas sem abrir um ADR formal.

---

Preciso decidir algo no projeto **[nome do projeto]**.

## A decisão

<<SUA FALA>>

Opções que estou considerando: [A, B, C — ou "sugira você"]

## Regras

1. **Decida para ESTE projeto, não em abstrato.** Leve em conta a stack que já existe, o tamanho do time ([1 pessoa]), o que já está instalado e o que eu já sei manter. A resposta "depende" sem aterrissar é inútil aqui.
2. **Recomende UMA opção** e assuma a recomendação. Comparação equilibrada sem escolha me devolve o problema.
3. Para cada opção, no máximo: **o que ganho, o que perco, e o que me obriga a fazer depois**. Sem tabela gigante de features.
4. **Diga o custo de errar**: se eu escolher e me arrepender em [3 meses], quanto custa trocar? Decisão fácil de reverter merece menos análise — e você deve dizer isso quando for o caso, em vez de me fazer deliberar à toa.
5. **Aponte a alternativa que eu não listei** se houver uma melhor, incluindo "não fazer nada agora" — adiar decisão é uma opção legítima quando o custo de esperar é baixo.
6. Se faltar informação para decidir com responsabilidade, **faça a pergunta** em vez de escolher no escuro.

## Critérios de decisão (nesta ordem)

- Funciona para o caso real e o volume real de hoje.
- Simplicidade de manter sozinho — menos peças móveis ganha de mais poderoso.
- Reversibilidade: prefira o que dá para desfazer.
- Só então: desempenho, elegância, popularidade.

## Formato da resposta

A recomendação em uma frase, o porquê em três, o principal risco de tê-la escolhido, e o sinal concreto que indicaria que foi a escolha errada (para eu perceber cedo).
