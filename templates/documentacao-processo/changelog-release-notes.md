# Changelog e release notes

> Modo: `quick` · Área: documentação
> Uso: gerar changelog/release notes a partir dos commits e PRs de um período.

---

Gere o changelog/release notes do projeto **[nome do projeto]** para o período **[tag anterior..HEAD | intervalo de datas]**.

## Contexto e público

<<SUA FALA>>

## Fonte dos dados

- Derive tudo do `git log` e dos PRs do período — não inclua mudança que não está no histórico.
- Formato dos links: [URL de PR/issue do repositório].

## Regras de escrita

1. **Escreva para o usuário, não para o autor do commit.** "Corrigido travamento ao exportar PDF", não "fix: null check no exporter" — traduza cada item para o efeito que o leitor percebe.
2. **Agrupe por tipo:** Breaking changes, Novidades, Correções (e Outros só se necessário).
3. **Breaking changes no topo**, cada uma com instrução de migração concreta: o que o usuário precisa mudar e como.
4. **Link para o PR/issue** em cada item relevante.
5. **Ignore commits internos** sem efeito para o usuário (refactor interno, ajuste de CI, bump de dependência sem impacto) — changelog poluído não é lido.
6. Um item por mudança perceptível: agrupe os commits que compõem a mesma feature em vez de listar um por um.

## Versão sugerida

Sugira a próxima versão semver com justificativa em uma frase: major se houver breaking change, minor se houver feature nova, patch se só correções.

## Formato da resposta

O changelog pronto em Markdown (breaking → novidades → correções, com links), seguido de:

- Versão sugerida + justificativa.
- Lista dos commits ignorados, com o motivo em uma palavra.
- Itens em que a intenção do commit ficou ambígua, marcados [confirmar].
