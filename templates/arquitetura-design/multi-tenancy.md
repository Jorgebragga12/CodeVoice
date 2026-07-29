# Multi-tenancy

> Modo: `planning` · Área: arquitetura
> Uso: projetar isolamento de dados entre clientes (tenants) no mesmo sistema.

---

Projete a estratégia de multi-tenancy do projeto **[nome do projeto]**.

## Requisitos e situação

<<SUA FALA>>

## Contexto a preencher

- Tenants esperados: [hoje e em 2 anos — 10 grandes é um design, 10 mil pequenos é outro]
- Exigência de isolamento dos clientes: [contratual/compliance ou apenas lógico]
- Banco e stack: [ex.: PostgreSQL + ORM usado]
- Distribuição de tamanho: [tenants uniformes ou um deles 100x maior que os outros]

## Regras do design

1. **Compare os três modelos com trade-offs honestos** para os números acima: linha com `tenant_id` (barato, escala a milhares, maior risco de vazamento), schema por tenant (isolamento médio, migração multiplicada por N), banco por tenant (isolamento máximo, custo e operação por tenant). Recomende um e diga o que faria mudar de ideia.
2. **Regra inegociável: TODA query filtra por tenant — garantida por estrutura, não por disciplina.** Desenvolvedor esquece WHERE; o design não pode depender de alguém lembrar. Proponha o mecanismo: [RLS no banco, repositório/base query central que injeta o filtro, middleware que resolve o tenant da requisição] — e proíba caminho de acesso ao banco que o contorne.
3. **Vazamento entre tenants é o pior bug possível do produto** — trate como incidente de segurança. Exija testes automatizados de vazamento: criar dados no tenant A, autenticar como tenant B e provar que nada do A aparece — em listagem, busca, acesso direto por ID e relatórios/agregações.
4. Defina como **migração de schema** roda em todos os tenants (ordem, falha no meio do lote, tenant indisponível) e como fazer **backup e restore de UM tenant** sem afetar os demais — restore individual é pedido certo de acontecer.
5. Cache, filas, busca e storage de arquivos também são particionados por tenant — o vazamento não acontece só no banco.
6. Restore ou exclusão de dados de um tenant é passo destrutivo: o plano marca onde **exige confirmação explícita** antes de executar.

## Formato da resposta

1. Tabela comparativa dos três modelos aplicada aos números dados (custo, isolamento, operação).
2. Recomendação com justificativa e critérios que fariam trocar de modelo.
3. Mecanismo estrutural do filtro por tenant (com onde ele mora no código/banco).
4. Plano de testes de vazamento entre tenants.
5. Estratégia de migração de schema e de backup/restore por tenant.
6. Riscos e perguntas abertas.
