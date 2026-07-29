# Design de contrato de API

> Modo: `planning` · Área: arquitetura
> Uso: desenhar o contrato de uma API antes de implementar qualquer endpoint.

---

Desenhe o contrato da API de **[recurso/domínio — ex.: pedidos]** do projeto **[nome do projeto]**.

## O que a API precisa oferecer

<<SUA FALA>>

## Contexto a preencher

- Quem consome: [frontend próprio, app mobile, parceiros externos — ou vários]
- Estilo: [REST/JSON, GraphQL, gRPC — ou "sugira e justifique"]
- Convenções já existentes no projeto: [formato de erro, autenticação, versionamento — ou "não há"]

## Regras do contrato

1. **Desenhe do ponto de vista de quem CONSOME**: comece listando as telas/operações do cliente e derive os endpoints delas — não exponha a estrutura interna do banco como API.
2. Recursos são substantivos no plural; o verbo vai no método HTTP. Códigos de status coerentes: 201 com Location ao criar, 400 para entrada inválida, 404 para recurso inexistente, 409 para conflito de estado — nunca 200 com erro no corpo.
3. **Toda listagem nasce com paginação, filtro e ordenação definidos** (ex.: cursor ou page/limit) — adicionar paginação depois quebra todos os clientes.
4. **Um único formato de erro para a API inteira**: código estável para máquina, mensagem para humano, campo indicando o que falhou. Documente uma vez, use em tudo.
5. Defina versionamento [ex.: /v1 na URL] e a regra de compatibilidade retroativa: adicionar campo opcional pode; renomear, remover ou mudar tipo exige versão nova.
6. Marque em cada operação: autenticação exigida e idempotência (PUT/DELETE idempotentes; POST de criação aceita [chave de idempotência] se o cliente faz retry).
7. Se uma decisão depender de como o consumidor usa (tamanho de página, campos da listagem), pergunte em vez de chutar.

## Formato da resposta

1. Especificação OpenAPI (YAML) **ou** tabela: endpoint → método → request → response → códigos de erro.
2. Exemplo de payload de sucesso e de erro para cada operação.
3. Formato de erro padrão documentado.
4. Decisões tomadas e dúvidas abertas para mim.
