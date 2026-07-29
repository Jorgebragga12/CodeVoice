# Descrição de pull request

> Modo: `quick` · Área: documentação
> Uso: escrever descrição de PR que o revisor entende sem abrir o diff.

---

Escreva a descrição do pull request da branch **[branch]** do projeto **[nome do projeto]**.

## Contexto da mudança

<<SUA FALA>>

## Fonte

- Derive do diff real (`git diff [base]...[branch]`) e das mensagens de commit — não descreva intenção que o código não confirma.
- Issue/tarefa relacionada: [link ou "nenhuma"]

## Estrutura obrigatória

1. **O problema e o porquê ANTES do que foi feito.** Comece com 2 a 4 frases sobre o problema e por que resolver agora — o revisor precisa do contexto antes da solução.
2. **Resumo das mudanças por área** (backend, UI, banco, config...), não arquivo por arquivo.
3. **Como testar:** passos numerados e executáveis, do setup ao resultado esperado — comando que o revisor cola e roda.
4. **Mudança visual? Screenshot ou gravação é obrigatória** — insira o marcador [anexar screenshot/gravação] no ponto certo.
5. **Riscos e o que NÃO foi feito de propósito:** limitações conhecidas, follow-ups planejados, decisões adiadas — evita comentário de revisão sobre o que já foi decidido.
6. **Guia de leitura:** "comece por [arquivo]" — aponte o arquivo que carrega a decisão central e a ordem de leitura sugerida.

## Regra de ouro

O revisor não deve precisar abrir o diff para entender a intenção. Se a descrição só faz sentido olhando o código, reescreva.

## Formato da resposta

A descrição pronta em Markdown na estrutura acima, seguida de:

- O que você não conseguiu confirmar no diff (se houver), marcado [confirmar].
- Sugestão de título do PR em uma linha (imperativo, sem jargão de commit).
