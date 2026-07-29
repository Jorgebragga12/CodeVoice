# TDD estrito

> Modo: `new_feature` · Área: testes
> Uso: implementar uma funcionalidade nova dirigida por testes, ciclo a ciclo.

---

Implemente a funcionalidade abaixo no projeto **[nome do projeto]** usando TDD estrito.

## Funcionalidade

<<SUA FALA>>

## Antes de codificar

Escreva a lista de casos de teste ANTES do primeiro teste: caminho feliz, bordas ([vazio/limite/duplicado]) e erros esperados. A lista guia a ordem dos ciclos e pode crescer durante o trabalho — mas nada é implementado sem constar nela.

## Ciclo obrigatório (red-green-refactor)

1. **RED:** escreva UM teste pequeno e rode-o. Ele deve falhar pelo motivo certo (asserção, não erro de import/setup). Se o teste nunca falhou, ele não prova nada — descarte e reescreva.
2. **GREEN:** escreva a implementação MÍNIMA que faz o teste passar. Resista a implementar o caso seguinte "já que está aqui".
3. **REFACTOR:** com a suíte verde, limpe duplicação e nomes — em passos pequenos que mantêm tudo verde.
4. Commit ao fim de cada ciclo (ou de poucos ciclos coesos), com mensagem indicando o caso coberto.

## Regras

- Passos pequenos: se um teste exigir implementação grande para passar, quebre o caso em dois.
- Nenhum código de produção sem um teste que o exija; nenhum teste que passe sem a implementação.
- Rode a suíte completa antes de encerrar: [comando de teste].

## Critérios de aceitação

- [ ] Lista de casos escrita antes do primeiro teste (e presente no relatório).
- [ ] Todo teste foi visto falhando antes de passar.
- [ ] Histórico de commits reflete os ciclos.
- [ ] Bordas e erros da lista cobertos, não só o caminho feliz.
- [ ] Suíte completa verde ao final.

## Formato do relatório final

A lista de casos com o status de cada um, os commits por ciclo, decisões de refactor relevantes e o resultado da suíte completa.
