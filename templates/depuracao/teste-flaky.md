# Teste intermitente (flaky)

> Modo: `bug_fix` · Área: depuração
> Uso: teste que passa às vezes e falha às vezes, sem mudar o código.

---

Tenho um teste intermitente no projeto **[nome do projeto]**.

## Qual é

<<SUA FALA>>

Teste: `[nome/arquivo do teste]`
Quando falha: [na CI / local / só quando roda a suíte inteira / aleatório]

## Regras de investigação

1. **Meça antes de teorizar.** Rode o teste [50–100] vezes e me dê a taxa real ("falhou 7 de 50"), não "às vezes". Se falhar 0 de 100 isolado mas falhar na suíte, a causa é interferência entre testes — o que já elimina metade das hipóteses.
2. **Separe as hipóteses com dois experimentos baratos:**
   - **Isolado vs. em suíte** → se só falha em suíte: estado compartilhado (banco, arquivo, singleton, variável global, mock não restaurado).
   - **Ordem aleatória** ([flag de random order do runner]) → se a ordem muda o resultado: um teste depende do que outro deixou para trás.
3. **Causas clássicas, nesta ordem de frequência:** estado compartilhado não limpo entre testes; espera por tempo fixo (`sleep`/timeout) em vez de esperar pela condição; concorrência/ordem de tarefas assíncronas; dado aleatório ou data/hora real sem fixar semente/relógio; dependência de rede ou serviço externo; ordenação não determinística de coleção/query sem `ORDER BY`.
4. **Nunca "resolva" com retry, `sleep` maior ou aumentando o timeout.** Isso não conserta nada: esconde o problema, deixa a suíte mais lenta e converte o defeito num que só aparecerá em produção sob carga.
5. **O flaky pode estar apontando bug real no código**, não no teste. Antes de mexer no teste, responda: se isso acontecesse em produção, seria um bug? Se sim, conserte o código.
6. Se não der para corrigir agora: **quarentena explícita** — marque com o motivo e crie o registro/issue. Nunca desabilite silenciosamente.

## Validações

- Rodar [100]x seguidas **verde** após a correção — a mesma medição que provou o problema agora prova o conserto.
- Rodar a suíte completa em ordem aleatória, verde.

## Critérios de aceitação

- [ ] A taxa de falha foi medida antes e depois.
- [ ] A causa está nomeada (qual das classes acima).
- [ ] Nenhum `sleep`/retry/timeout foi usado como solução.
- [ ] Se era bug real de código, o código foi corrigido — não o teste.

## Formato do relatório final

Taxa antes/depois, a causa concreta, o que mudou, e se algum outro teste da suíte tem o mesmo padrão de fragilidade.
