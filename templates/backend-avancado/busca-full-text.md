# Busca no produto

> Modo: `new_feature` · Área: backend
> Uso: implementar busca de texto para os usuários do produto.

---

Implemente busca no projeto **[nome do projeto]**.

## Escopo

<<SUA FALA>>

## Contexto a preencher

- O que é buscável: [entidades e campos] · Idioma do conteúdo: [idioma]
- Volume atual e projetado: [nº de registros]

## Decisão de infraestrutura

1. **Comece pela busca full-text do próprio banco** ([FTS do banco em uso]). Motor dedicado ([motor de busca]) só entra com justificativa concreta de escala ou relevância que o FTS não atende — é mais uma peça de infra para operar e manter sincronizada.
2. Se propor motor dedicado, registre no relatório a justificativa com os números que a sustentam.

## Requisitos de qualidade da busca

1. **Acentos e caixa não afetam o resultado**: "São Paulo", "sao paulo" e "SAO PAULO" encontram a mesma coisa (unaccent/normalização).
2. **Stemming configurado no idioma [idioma]**: buscar "pagamento" encontra "pagamentos".
3. **Ranking explicável**: defina o que pesa mais ([título > corpo, recência, popularidade]) e seja capaz de explicar por que um resultado veio antes do outro.
4. **Índice atualizado quando o dado muda** (criar/editar/excluir), via [trigger/evento/job], com atraso máximo aceitável de [valor]. Busca que mostra registro excluído é bug.
5. Permissões respeitadas: o usuário só encontra o que pode ver.

## Estados de UI

- **Busca vazia** (antes de digitar): o que aparece [recentes/sugestões/nada].
- **Sem resultados**: mensagem clara com próximo passo, nunca tela em branco.

## Validação de performance

- Meça a latência com volume realista ([N] registros — gere dados se preciso) e registre p50/p95 no relatório. Busca testada com 10 linhas não prova nada.

## Critérios de aceitação

- [ ] Busca ignora acentos e caixa (testes com variações)
- [ ] Stemming funciona no idioma do conteúdo (teste singular/plural)
- [ ] Criar/editar/excluir registro reflete na busca dentro do atraso definido (teste)
- [ ] Usuário não encontra registros que não tem permissão de ver (teste)
- [ ] Estados de busca vazia e sem resultados implementados
- [ ] Latência p50/p95 medida com volume realista e registrada

## Formato do relatório final

Infra escolhida (e por que não a alternativa), regras de ranking, mecanismo de atualização do índice, e os números de latência com o volume testado.
