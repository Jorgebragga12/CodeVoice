# README do projeto

> Modo: `quick` · Área: documentação
> Uso: escrever ou reescrever o README para que qualquer pessoa rode o projeto do zero.

---

Escreva o README.md do projeto **[nome do projeto]**.

## O que documentar

<<SUA FALA>>

## Contexto a preencher

- Stack principal: [linguagem/framework]
- Comandos: dev [comando] · teste [comando] · build [comando]

## Estrutura obrigatória do README

1. **O que é** em no máximo 2 frases — sem histórico do projeto, sem marketing.
2. **Pré-requisitos** com versões mínimas (derive de package.json/pyproject/Dockerfile — não chute).
3. **Instalação passo a passo.** EXECUTE cada comando antes de documentar; comando que você não rodou não entra no README.
4. **Variáveis de ambiente:** crie/atualize o `.env.example` com todas as variáveis, um comentário de uma linha por variável e nenhum valor real de segredo.
5. **Comandos do dia a dia** (dev, teste, build) em bloco de código copiável.
6. **Estrutura de pastas** em até 5 linhas — só o que orienta a navegação.
7. **Como contribuir:** branch, padrão de commit, como abrir PR.

## Regras

- Se um passo falharia numa máquina limpa, o README está errado — não documente atalhos que dependem do seu ambiente local.
- Derive tudo do código existente; onde faltar informação, marque `TODO:` em vez de inventar.
- Se um comando falhar durante a verificação, reporte a falha — não omita o passo nem esconda o erro.

## Critérios de aceitação

- [ ] Todos os comandos de instalação foram executados e funcionaram.
- [ ] `.env.example` cobre todas as variáveis lidas pelo código (confira buscando `process.env`/`os.environ`/equivalente).
- [ ] Cada pré-requisito cita versão mínima e o arquivo de onde ela veio.
- [ ] Estrutura de pastas tem no máximo 5 linhas.

## Formato do relatório final

Comandos executados na verificação (com resultado), variáveis encontradas no código vs. documentadas no `.env.example`, e o que ficou marcado como TODO.
