# Erro agora (colar stack trace)

> Modo: `bug_fix` · Área: depuração
> Uso: quebrou agora, tenho a mensagem de erro na tela e quero entender e corrigir.

---

Estou com um erro no projeto **[nome do projeto]**.

## O que eu estava fazendo

<<SUA FALA>>

## Erro completo

```
[COLE AQUI o stack trace / mensagem inteira, sem cortar]
```

## Regras de investigação

1. **Leia o erro inteiro antes de tocar em qualquer código.** Em stack de exceção encadeada, a causa real costuma estar no fim (`Caused by:`, `originally thrown at`), não na primeira linha — a primeira é onde estourou, não onde nasceu.
2. **Localize a linha do MEU código** no stack. Frames de biblioteca/framework raramente são o bug; eles são o mensageiro.
3. **Diga a causa antes de propor a correção.** Se não deu para determinar com certeza, liste as hipóteses em ordem de probabilidade e diga que informação falta para decidir — não escolha uma e siga como se fosse fato.
4. **Nunca "corrija" engolindo o erro**: `try/catch` vazio, `?.` para calar um `undefined`, `unwrap_or_default()` para mascarar `None` — isso troca uma falha visível por uma silenciosa, que é pior.
5. **Reproduza antes de corrigir.** Se não conseguir reproduzir, diga isso explicitamente em vez de aplicar uma correção especulativa.

## Validações

- Um teste que **falha antes** da correção e **passa depois** — é o que prova que você corrigiu a causa e não o sintoma.
- Rodar: [comando de teste].

## Critérios de aceitação

- [ ] A causa raiz está explicada em uma frase que eu entendo.
- [ ] A correção ataca a causa, não a mensagem.
- [ ] Existe teste cobrindo o caso que quebrou.
- [ ] Nenhum comportamento não relacionado foi alterado de brinde.

## Formato do relatório final

O que causou o erro (uma frase), o que foi mudado e por quê, o teste que prova a correção, e — se aplicável — outros pontos do código com o mesmo padrão de defeito.
