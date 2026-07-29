# Condição de corrida / concorrência

> Modo: `bug_fix` · Área: depuração
> Uso: dado inconsistente, travamento ou falha que só aparece às vezes, com coisas acontecendo ao mesmo tempo.

---

Suspeito de problema de concorrência no projeto **[nome do projeto]**.

## Sintoma

<<SUA FALA>>

- Acontece quando: [há vários usuários / há requisições simultâneas / tem thread e callback / tem job em background / não sei]
- Sintoma: [valor errado/perdido / trava e não volta (deadlock) / duplicou / falha aleatória]

## Regras de investigação

1. **Encontre o estado compartilhado e liste quem escreve nele.** Bug de concorrência é sempre sobre estado alcançável por dois caminhos ao mesmo tempo: variável global/singleton, cache em memória, arquivo, linha do banco, contador. Se não houver estado compartilhado, não é corrida — é outra coisa.
2. **Classifique, porque a correção é diferente:**
   - **Corrida de dados** (dois escrevem, um sobrescreve o outro) → precisa de lock, operação atômica, ou transação com o nível de isolamento certo.
   - **Read-modify-write** (lê, calcula, grava — e alguém escreveu no meio) → precisa de atualização atômica no banco (`UPDATE ... SET x = x + 1`) ou trava otimista com versão.
   - **Deadlock** (dois esperam um pelo outro) → ordem de aquisição de locks inconsistente.
   - **Ordem de eventos** (o callback chegou antes do esperado) → falta sincronização/estado explícito, não lock.
3. **Nunca "corrija" com `sleep`.** Dormir só reduz a janela e faz o bug voltar em máquina mais rápida, sob carga, ou no cliente. Se apareceu um `sleep` como solução, o problema não foi entendido.
4. **Reproduza sob concorrência real**: um teste que dispare [N] operações simultâneas contra o mesmo recurso. Bug de corrida não reproduz em teste sequencial — e por isso passou despercebido até agora.
5. **Prefira eliminar o compartilhamento a proteger o compartilhamento.** Passar dado por mensagem/canal, ou deixar o banco resolver com constraint (`UNIQUE`) e transação, costuma ser mais robusto que espalhar locks. Lock é a última opção, não a primeira.
6. **Nunca segure um lock durante I/O** (rede, disco, chamada externa) — é a receita de contenção e deadlock.

## Validações

- Teste com [N] execuções concorrentes que **falha antes** da correção e passa depois.
- Se a linguagem tiver detector ([thread sanitizer, `--race`, etc.]), rode e cole a saída.
- Rodar: [comando de teste].

## Critérios de aceitação

- [ ] O estado compartilhado e os escritores estão nomeados.
- [ ] A classe do problema foi identificada (corrida / read-modify-write / deadlock / ordem).
- [ ] Nenhum `sleep` foi usado como sincronização.
- [ ] Existe teste concorrente que reproduzia o defeito.

## Formato do relatório final

Qual estado era compartilhado, como as duas execuções se cruzavam, a correção escolhida e por que ela (e não as alternativas), e o teste que prova.
