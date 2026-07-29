# Endpoint de API REST

> Modo: `new_feature` · Área: desenvolvimento
> Uso: criar ou alterar um endpoint HTTP com contrato claro.

---

Crie/altere um endpoint no projeto **[nome do projeto]**.

## Objetivo

<<SUA FALA>>

## Contrato

- Método e rota: [ex.: POST /api/v1/pedidos]
- Corpo da requisição: [campos e tipos, ou "derivar do objetivo"]
- Resposta de sucesso: [status + shape do JSON]
- Erros: 400 para entrada inválida (com mensagem por campo), 401/403 quando aplicável, 404 para recurso inexistente. Nunca 500 para erro previsível.

## Requisitos técnicos

- Valide TODA entrada no servidor (tipo, tamanho, formato) — não confie no cliente.
- Siga o padrão de rotas/handlers já existente no projeto.
- Autenticação/autorização: [regra — ex.: só o dono do recurso pode alterar].
- Paginação em listagens: [padrão do projeto ou limit/offset com máximo].

## Validações

- Testes cobrindo: caso feliz, entrada inválida, não autorizado, recurso inexistente.
- Rodar: [comando de teste].

## Critérios de aceitação

- [ ] Contrato respeitado exatamente como descrito acima.
- [ ] Nenhum dado sensível vaza em mensagens de erro ou logs.
- [ ] Testes novos + existentes verdes.

## Formato do relatório final

Contrato final (rota, request, responses), arquivos alterados e um exemplo de chamada (curl) para eu testar.
