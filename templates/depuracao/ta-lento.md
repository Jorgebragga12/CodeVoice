# Tá lento (investigar)

> Modo: `bug_fix` · Área: depuração
> Uso: alguma coisa ficou lenta e eu ainda não sei o quê — investigar antes de otimizar.

---

Algo está lento no projeto **[nome do projeto]**.

## O que está lento

<<SUA FALA>>

- Onde eu percebo: [tela/rota/comando]
- Quanto demora hoje: [tempo percebido, ou "não medi"]
- Quanto seria aceitável: [alvo]
- Ficou lento de repente ou sempre foi? [de repente após [mudança] / sempre foi / piorou aos poucos]

## Regras de investigação

1. **Meça primeiro, e me mostre o número.** Sem baseline não há como saber se a mudança ajudou — e "parece mais rápido" não conta. Se não houver instrumentação, adicione a medição mínima antes de mexer em qualquer coisa.
2. **Localize a camada antes de otimizar qualquer linha.** Divida o tempo total entre: banco (query), rede (chamada externa), CPU (processamento), I/O (disco/arquivo), render (frontend). Otimizar a camada errada é trabalho perdido — a maior parte do tempo costuma estar em um lugar só.
3. **Suspeite primeiro do óbvio, nesta ordem:** consulta N+1 (loop que consulta o banco a cada item); falta de índice em coluna usada em `WHERE`/`JOIN`/`ORDER BY`; trazer dados demais (`SELECT *`, sem paginação); chamada externa síncrona dentro de loop; trabalho refeito a cada request que poderia ser feito uma vez; re-render em cascata no frontend.
4. **"Ficou lento de repente" muda a investigação**: compare com o antes — mudança de código recente, crescimento de volume de dados (query que era rápida com 100 linhas e morreu com 100 mil), ou mudança de infra. Comece pelo diff, não pelo profiler.
5. **Não adicione cache como primeira solução.** Cache sobre consulta ineficiente esconde o defeito e cria problema novo (invalidação). Só depois de ter o número e a causa.

## Validações

- Medição **antes e depois**, mesmo cenário e mesma massa de dados, com o comando/método usado para medir.
- Confirmar que o comportamento não mudou: [comando de teste] verde.

## Critérios de aceitação

- [ ] Existe número antes e número depois — não impressão.
- [ ] A causa da lentidão está nomeada (qual camada, qual operação).
- [ ] O ganho é proporcional ao esforço; nada de otimização especulativa em código que não era o gargalo.
- [ ] Nenhum resultado mudou por causa da otimização.

## Formato do relatório final

Tempo antes → depois, onde estava o gargalo, o que foi feito, e o que **não** vale otimizar agora (com o motivo) para eu não perder tempo depois.
