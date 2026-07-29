# Dashboard com gráficos

> Modo: `ui_creation` · Área: frontend/UI
> Uso: painel com métricas, cards e gráficos.

---

Crie um dashboard no projeto **[nome do projeto]**.

## Objetivo

<<SUA FALA>>

## Conteúdo

- Métricas principais (cards no topo): [liste — ou derive do objetivo]
- Gráficos: [tipo e dado de cada um — ex.: linha para evolução no tempo, barras para comparação entre categorias]
- Filtros: [período, categoria…]

## Regras de visualização

- Escolha o tipo de gráfico pelo dado: evolução no tempo → linha; comparação entre categorias → barras; parte de um todo → evite pizza com mais de 4 fatias.
- Eixo Y começando em zero para barras; formatação de números no padrão [pt-BR: 1.234,56].
- Cores com contraste suficiente e distinguíveis (não dependa só de cor para diferenciar séries — use rótulos).
- Tooltip com o valor exato ao passar o mouse.

## Estados

- Carregando (skeleton nos cards/gráficos), vazio ("sem dados no período"), erro com retry.
- Filtro aplicado atualiza todos os componentes de forma consistente.

## Requisitos técnicos

- Use a lib de gráficos já presente no projeto; se não houver, proponha uma leve e espere aprovação.
- Consultas agregadas no servidor — não traga dados brutos para agregar no cliente.

## Critérios de aceitação

- [ ] Métricas batem com os dados reais (mostre a consulta que gera cada uma).
- [ ] Filtros funcionam em todos os componentes.
- [ ] Estados de carregando/vazio/erro implementados.

## Formato do relatório final

Cada métrica com sua consulta de origem, decisões de visualização e como validar os números.
