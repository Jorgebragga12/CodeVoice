# Onboarding de dev

> Modo: `quick` · Área: documentação
> Uso: criar guia de onboarding para dev novo chegar rápido ao primeiro PR.

---

Escreva o guia de onboarding do projeto **[nome do projeto]** para um dev novo no time. Declare no topo do guia a meta: **primeiro PR em [X dias]**.

## Contexto do time e do projeto

<<SUA FALA>>

## Seções obrigatórias do guia

1. **Setup do ambiente** com tempo estimado por etapa. Derive do que o projeto realmente exige (lockfiles, Docker, scripts) e EXECUTE os passos num ambiente limpo antes de documentar — passo não testado é passo quebrado.
2. **Arquitetura em uma página:** os 3 a 6 blocos principais e como conversam, com um diagrama simples (Mermaid serve). O objetivo é orientar, não esgotar — detalhe de implementação fica fora.
3. **Glossário do domínio:** termos que o código usa e um dev de fora não conhece ([termos do domínio]).
4. **Fluxo de trabalho do time:** como nasce uma branch, padrão de commit, como funciona o review, como o código chega em produção.
5. **Onde pedir ajuda:** canal ou pessoa por assunto ([canais/pessoas]).
6. **Primeira tarefa sugerida:** uma tarefa real, pequena e de baixo risco que exercita o fluxo completo (branch → PR → merge). Escolha do backlog ou proponha uma (ex.: melhorar uma mensagem de erro).

## Regras

- Escreva para quem nunca viu o projeto: nenhuma sigla sem expansão na primeira ocorrência.
- Onde não houver informação (ex.: canal de ajuda), marque [preencher] em vez de inventar.
- Se um passo do setup falhar no teste, corrija o guia (ou o script) — não documente a versão que "deveria funcionar".

## Critérios de aceitação

- [ ] Passos de setup executados do zero, com o tempo real anotado por etapa.
- [ ] Diagrama de arquitetura presente e coerente com o código atual.
- [ ] Primeira tarefa sugerida existe, é pequena e cabe na meta de [X dias].
- [ ] Nenhuma sigla ou termo de domínio sem explicação.

## Formato do relatório final

O que foi testado no setup (e quanto tempo levou cada etapa), divergências entre a documentação anterior e o projeto real, e os campos deixados como [preencher].
