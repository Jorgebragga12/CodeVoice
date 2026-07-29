# Continuar de onde parei

> Modo: `quick` · Área: fala rápida
> Uso: retomar um trabalho de outra sessão sem perder contexto nem refazer o que já foi feito.

---

Quero retomar um trabalho no projeto **[nome do projeto]**.

## O que eu lembro

<<SUA FALA>>

## Antes de propor qualquer coisa, levante o estado real

1. **O que já foi feito**: veja o que mudou de fato — `git status` e `git log --oneline -[10]`, mais os arquivos alterados e não commitados. O que eu lembro pode estar errado ou incompleto; **o repositório é a fonte da verdade, minha memória não**.
2. **Onde parou**: procure os sinais de trabalho interrompido — `TODO`/`FIXME` recentes, teste falhando, função esboçada sem uso, import não utilizado, código comentado. Se houver [documento de plano/relatório de fase], leia e compare com o que existe no código.
3. **Se está saudável**: rode [comando de teste] e [lint] para saber se o ponto de partida está verde. Retomar em cima de base quebrada e não perceber é como se perde meia hora.

## Depois disso, me devolva

- **Onde exatamente eu parei** (arquivo, função, o que estava no meio).
- **O que está feito e funcionando** vs. **feito pela metade** — a distinção importa mais que a lista.
- **O próximo passo concreto**, um só, com o motivo de ser esse.
- **Armadilhas**: decisão que eu tomei antes e que não é óbvia lendo o código agora, e que eu poderia contrariar sem perceber.

## Regras

- **Não comece a implementar ainda.** Primeiro o diagnóstico, eu confirmo, depois seguimos.
- Se o que eu falei contradiz o que está no código, **aponte a contradição** em vez de assumir que um dos dois está certo.
- Não sugira recomeçar do zero por não entender o que está lá — pergunte.

## Formato da resposta

Estado atual em poucas linhas, lista curta do que está pronto vs. pela metade, o próximo passo, e as armadilhas que eu deveria lembrar antes de continuar.
