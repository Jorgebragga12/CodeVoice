# CRUD completo de uma entidade

> Modo: `new_feature` · Área: desenvolvimento
> Uso: cadastro completo (criar, listar, editar, excluir) de uma entidade nova.

---

Implemente o CRUD da entidade **[nome da entidade]** no projeto **[nome do projeto]**.

## Objetivo

<<SUA FALA>>

## Campos da entidade

[liste os campos com tipo e obrigatoriedade — ou: "derive do objetivo e me apresente para aprovação antes de implementar"]

## Requisitos funcionais

- Criar, listar (com paginação e busca por [campo]), ver detalhe, editar e excluir.
- Exclusão pede confirmação e é [soft delete com campo deleted_at / exclusão real — escolha e justifique].
- Validação nos dois lados: cliente (feedback imediato) e servidor (fonte da verdade).

## Requisitos técnicos

- Siga o padrão das entidades já existentes no projeto (rotas, camadas, nomes).
- Migration versionada para a tabela nova, com rollback.
- Datas em UTC; IDs conforme padrão do projeto.

## Validações

- Testes de cada operação incluindo: validação rejeitando dados inválidos, edição de registro inexistente, exclusão dupla.

## Critérios de aceitação

- [ ] As 5 operações funcionam de ponta a ponta (UI ou API, conforme o projeto).
- [ ] Dados inválidos são rejeitados com mensagem clara por campo.
- [ ] Testes e lint verdes.

## Formato do relatório final

Esquema final da entidade, rotas/telas criadas, e o que ficou de fora de propósito.
