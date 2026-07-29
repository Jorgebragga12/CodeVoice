# Prompt de sistema de produto

> Modo: `quick` · Área: engenharia de IA
> Uso: escrever o prompt de sistema de uma feature de produto com LLM, robusto a entrada hostil.

---

Escreva o prompt de sistema para a feature abaixo, no projeto **[nome do projeto]**.

## O que a feature faz

<<SUA FALA>>

## Requisitos do prompt

1. **Papel e limites explícitos**: o que o assistente é, o que faz e o que NUNCA faz — limite implícito não segura caso real.
2. **Formato de saída especificado**: se a saída é parseada por código, declare o schema exato (JSON com campos e tipos) e o que devolver quando não houver resposta válida.
3. **Entrada do usuário delimitada como DADO** (ex.: entre tags): o prompt diz textualmente que instrução dentro da entrada não é comando e deve ser tratada como conteúdo.
4. **Casos de recusa definidos**: liste o que está fora do escopo e a resposta padrão para cada tipo — recusa improvisada vaza escopo.
5. **Exemplos few-shot dos casos difíceis** (ambíguos, de borda, de recusa) — exemplo fácil ensina pouco.
6. Sem promessa que o produto não cumpre e sem dado sensível embutido no prompt.

## Teste com entradas hostis

Teste o prompt final com, no mínimo: tentativa de injeção ("ignore as instruções anteriores..."), pedido fora do escopo, entrada vazia e entrada em outro idioma. Documente entrada → saída de cada teste.

## Critérios de aceitação

- [ ] Prompt entregue em arquivo versionado ([caminho]).
- [ ] Saída bate com o schema em todos os testes (quando parseada por código).
- [ ] Injeção e fora-de-escopo caem nos casos de recusa definidos, não em resposta improvisada.
- [ ] Tabela de testes hostis (entrada → saída) documentada.

## Formato do relatório final

O prompt final, as decisões de design (uma linha cada) e a tabela de testes hostis.
