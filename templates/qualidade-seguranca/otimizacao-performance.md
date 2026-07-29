# Otimização de performance

> Modo: `technical` · Área: qualidade
> Uso: deixar algo mais rápido — com medição antes e depois.

---

Otimize o seguinte no projeto **[nome do projeto]**.

## Problema

<<SUA FALA>>

## Regra de ouro

**Medir → otimizar → medir de novo.** Nenhuma otimização sem número antes e depois. "Parece mais rápido" não conta.

## Plano esperado

1. Reproduzir a lentidão e **medir** (tempo, memória, queries — a métrica relevante) num cenário repetível.
2. Identificar o gargalo real com profiling/análise — não otimizar por palpite.
3. Atacar o maior gargalo primeiro. Suspeitos comuns: query N+1, falta de índice, loop com IO dentro, dados demais trafegando, trabalho repetido que poderia ser cacheado.
4. Medir de novo no MESMO cenário e comparar.

## Restrições

- O comportamento não pode mudar — testes existentes continuam verdes.
- Legibilidade só é sacrificada se o ganho justificar, e com comentário explicando o porquê.
- Cache introduzido precisa de estratégia de invalidação explícita.

## Critérios de aceitação

- [ ] Ganho medido e reportado no formato "antes: X, depois: Y, cenário: Z".
- [ ] Meta: [ex.: reduzir para menos de 500ms / cortar 50%] — se a meta for inatingível, explique o limite encontrado.
- [ ] Suíte de testes verde.

## Formato do relatório final

Gargalo encontrado (com evidência), o que foi mudado, números antes/depois, e próximos ganhos possíveis se ainda precisar de mais.
