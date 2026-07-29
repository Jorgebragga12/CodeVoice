# API GraphQL

> Modo: `new_feature` · Área: especialidades
> Uso: criar API GraphQL com schema pensado, sem N+1 e protegida contra query abusiva.

---

Implemente a seguinte API GraphQL no projeto **[nome do projeto]**.

## Escopo

<<SUA FALA>>

## Requisitos técnicos

1. **Schema desenhado pelo consumo, não pelo banco**: modele os tipos pelo que o cliente precisa ver, não espelhando tabelas — schema que expõe o banco vira contrato impossível de mudar depois.
2. **Dataloader em todo resolver de relação**, agrupando buscas (batch) por request — o N+1 é o problema clássico de GraphQL e aparece assim que alguém pede uma lista com relação aninhada.
3. **Autorização por campo/recurso, não só na entrada**: usuário autenticado ainda não pode ver [campos/recursos restritos]; a checagem vive no resolver ou numa camada de autorização, porque GraphQL permite chegar ao mesmo dado por vários caminhos do grafo.
4. **Limite de profundidade e de complexidade de query** ([profundidade máx / custo máx]): sem isso, uma query aninhada maliciosa derruba o servidor com uma requisição só.
5. **Paginação por cursor no padrão connections** (edges/pageInfo) em toda lista — offset quebra quando os dados mudam entre uma página e outra.
6. **Erros tipados**: erro de negócio sai com `code` estável em `extensions` (ou union de resultado); mensagem crua de exceção interna nunca chega ao cliente, mas é logada com o contexto do request.
7. **Persisted queries** [se aplicável]: em produção, aceitar só queries registradas reduz superfície de ataque e tamanho de payload.
8. **N+1 provado com medição**: log de queries do banco ligado, lista de [N] itens com relação aninhada → número constante de queries, não proporcional a N (medido, não presumido).

## Critérios de aceitação

- [ ] Lista com relação aninhada dispara número constante de queries no banco (teste com contagem)
- [ ] Usuário sem permissão não lê [campo restrito] por nenhum caminho do grafo (teste)
- [ ] Query acima do limite de profundidade/complexidade é rejeitada com erro claro (teste)
- [ ] Paginação por cursor estável com inserção de itens entre páginas (teste)
- [ ] Erro de negócio com código estável; exceção interna não vaza mensagem (teste)
- [ ] Só queries registradas aceitas em produção [se persisted queries]

## Formato do relatório final

Schema resultante (tipos e conexões), onde a autorização é aplicada, limites de query configurados, e a medição de queries do banco provando a ausência de N+1.
