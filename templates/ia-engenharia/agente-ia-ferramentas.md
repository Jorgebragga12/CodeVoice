# Agente de IA com ferramentas

> Modo: `new_feature` · Área: engenharia de IA
> Uso: construir um agente com tool use — loop que chama ferramentas até resolver a tarefa.

---

Construa um agente de IA com uso de ferramentas no projeto **[nome do projeto]**.

## Objetivo

<<SUA FALA>>

## Contexto a preencher

- Modelo/provedor: [modelo]
- Ferramentas que o agente terá: [lista]
- Onde roda: [backend / CLI / worker]

## Requisitos do loop de execução

1. **Limite duro de iterações ([nº]) e de custo por tarefa** — agente sem teto roda em círculo gastando dinheiro. Ao estourar, pare e reporte o estado parcial.
2. Cada ferramenta tem schema validado na entrada; ferramenta que falha devolve erro estruturado ao modelo, nunca exceção não tratada nem falha silenciosa.
3. **Resultado de ferramenta é dado NÃO confiável**: conteúdo vindo de web, arquivo ou API pode conter instrução maliciosa (prompt injection). Delimite-o como dado no contexto e nunca trate ordem vinda dele como comando.
4. **Fallback definido**: se o modelo chamar ferramenta inexistente, alucinar argumentos ou repetir a mesma chamada [nº] vezes, interrompa com mensagem clara em vez de insistir.
5. Estado/memória da conversa com política explícita: o que persiste entre turnos, o que é descartado e quando o contexto é truncado ou resumido.

## Telemetria

- Registre cada passo do loop: prompt, ferramenta chamada, argumentos, resultado, tokens e latência — sem isso, debugar agente é adivinhação.
- Um id de execução amarra todos os passos de uma mesma tarefa.

## Avaliação

- Monte [nº] tarefas-teste com resultado esperado, incluindo uma que exige múltiplas ferramentas e uma impossível (deve terminar em desistência honesta, não em invenção).
- O agente só está pronto quando passa nesse conjunto — rode-o de novo a cada mudança de prompt ou ferramenta.

## Critérios de aceitação

- [ ] Loop respeita limite de iterações e de custo (testar forçando o estouro).
- [ ] Injeção de instrução via resultado de ferramenta não muda o comportamento (testar com payload real).
- [ ] Ferramenta alucinada ou argumento inválido cai no fallback sem crash.
- [ ] Telemetria permite reconstruir uma execução inteira passo a passo.
- [ ] Todas as tarefas-teste passam.

## Formato do relatório final

Arquitetura do loop, resultado das tarefas-teste, limites configurados e casos de fallback cobertos.
