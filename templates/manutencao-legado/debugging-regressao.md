# Caçar regressão (git bisect)

> Modo: `bug_fix` · Área: manutenção
> Uso: encontrar o commit exato que quebrou algo que funcionava.

---

Encontre e corrija a regressão no projeto **[nome do projeto]**.

## Sintoma

<<SUA FALA>>

## Pontos de referência

- Último estado sabidamente BOM: [commit/tag/data ou "descobrir faz parte da tarefa"]
- Primeiro estado sabidamente RUIM: [commit/branch, ex.: HEAD]

## Regras da caçada

1. **Defina o teste objetivo de bom/ruim ANTES de começar**: um comando que sai 0 no estado bom e não-zero no ruim. Sem isso, bisect vira chute.
2. Use `git bisect`; sendo o teste um comando, automatize com `git bisect run` — elimina erro humano na classificação dos commits.
3. Commit que nem compila se pula (`git bisect skip`), não se classifica — classificar errado um único passo aponta o culpado errado.
4. **Confirme o culpado entendendo POR QUE ele quebra.** Leia o diff e explique o mecanismo; se o diff não explica o sintoma, alguma classificação foi errada — refaça o bisect.
5. **Reverter vs. corrigir para frente, com critério**: reverta se o commit é isolado e a correção real demora; corrija para frente se o revert arrasta funcionalidade já em uso ou conflita demais com o que veio depois.
6. Adicione o teste de regressão que teria pegado isso. A caçada só termina quando esse bug não conseguir mais passar despercebido pelo CI.

## Critérios de aceitação

- [ ] Commit culpado identificado, com explicação do mecanismo da quebra.
- [ ] Decisão revert/fix-forward justificada pelo critério acima.
- [ ] Teste de regressão novo: falha no código quebrado, passa após a correção.
- [ ] Suíte completa verde: [comando de teste].

## Formato do relatório final

Commit culpado (hash + trecho relevante do diff), por que quebra, o que foi feito (revert ou correção), e o teste de regressão adicionado.
