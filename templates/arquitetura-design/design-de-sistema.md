# Design de sistema

> Modo: `planning` · Área: arquitetura
> Uso: desenhar um sistema ou serviço novo antes de escrever qualquer código.

---

Desenhe a arquitetura de um novo sistema para o projeto **[nome do projeto]**.

## O que o sistema precisa fazer

<<SUA FALA>>

## Números que definem o design

- Usuários/requisições esperados: [ex.: pico de X req/s, Y usuários ativos/dia]
- Volume de dados: [ex.: X GB/mês, Y milhões de linhas no primeiro ano]
- Latência aceitável: [ex.: p95 abaixo de X ms]
- Disponibilidade exigida: [pode ficar fora 1h por mês? ou não pode cair nunca?]

Se eu não passei números, estime a partir do contexto e declare a estimativa — "muitos usuários" e "alta performance" são adjetivos, não requisitos.

## Regras do design

1. **Comece pelo desenho mais simples que atende os números acima.** Só adicione fila, cache, réplica ou serviço separado se um número exigir — cada peça extra é custo de operação para sempre.
2. Para cada componente: uma responsabilidade clara e o que acontece quando ele cai.
3. Mostre o **fluxo de dados de ponta a ponta** no caso de uso principal: quem chama quem, o que persiste onde, o que é síncrono e o que é assíncrono.
4. Identifique os **pontos de falha** e o comportamento degradado esperado em cada um (falha total, fila acumulando, resposta parcial).
5. Declare explicitamente **o que fica de fora da v1** e qual decisão do design permite evoluir depois sem reescrever.
6. Se uma escolha depender de informação que eu não dei (orçamento, time, infra existente), pergunte em vez de assumir.

## Formato da resposta

1. Diagrama de componentes (ASCII ou mermaid).
2. Tabela: componente → responsabilidade → o que acontece se cair.
3. Fluxo do caso de uso principal, passo a passo.
4. Justificativa das escolhas amarrada aos números (por que isso basta para X req/s).
5. O que ficou fora da v1 e por quê.
6. Riscos abertos e perguntas para mim.
