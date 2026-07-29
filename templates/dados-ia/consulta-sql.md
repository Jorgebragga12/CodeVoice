# Consulta SQL

> Modo: `quick` · Área: dados
> Uso: escrever ou otimizar uma consulta SQL.

---

Escreva/otimize uma consulta SQL em **[dialeto: PostgreSQL/MySQL/SQLite/BigQuery]**.

## O que a consulta precisa responder

<<SUA FALA>>

## Esquema relevante

[tabelas e colunas envolvidas — ou: "descubra pelo esquema do banco em [local]"]

## Regras

- Consulta legível: CTEs nomeadas em vez de subquery aninhada ilegível; alias claros.
- **Nunca** `SELECT *` em consulta final — liste as colunas.
- Atenção aos clássicos: JOIN duplicando linhas (confira a granularidade), NULL em comparação/agregação, timezone em datas, divisão inteira.
- Se for para produção/aplicação: parâmetros vinculados (nunca concatenar valores), e explique o plano com `EXPLAIN` se a tabela for grande.
- Se otimização: mostre o `EXPLAIN` antes/depois e o índice sugerido, se necessário.

## Formato da resposta

A consulta pronta, seguida de: uma frase por CTE/etapa explicando o que faz, as suposições feitas sobre os dados, e um resultado de exemplo (2–3 linhas) do shape esperado.
