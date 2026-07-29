# Correção de bug

> Modo: `bug_fix` · Área: desenvolvimento
> Uso: investigar e corrigir um erro pela causa raiz, não pelo sintoma.

---

Corrija o seguinte problema no projeto **[nome do projeto]**.

## Sintoma observado

<<SUA FALA>>

## Como reproduzir

[passos para reproduzir — se não souber, escreva "descobrir a reprodução faz parte da tarefa"]

## Comportamento esperado

[o que deveria acontecer]

## Regras da investigação

1. **Reproduza o problema antes de mexer em qualquer coisa.** Se não conseguir reproduzir, reporte o que tentou e pare.
2. **Encontre a causa raiz.** Não aplique remendo que só esconde o sintoma; explique POR QUE o bug acontece antes de corrigir.
3. Verifique se o mesmo padrão de erro existe em outros pontos do código.
4. Não altere comportamento não relacionado ao bug.

## Validações

- Escreva um teste que falha ANTES da correção e passa DEPOIS.
- Rode a suíte completa: [comando de teste].

## Formato do relatório final

Causa raiz (com o arquivo/linha), correção aplicada, teste que prova a correção, e se o padrão se repetia em outro lugar.
