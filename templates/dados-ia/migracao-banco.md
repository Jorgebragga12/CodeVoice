# Migração de banco de dados

> Modo: `db_change` · Área: dados
> Uso: alterar esquema ou dados com migration versionada e rollback.

---

Altere o banco do projeto **[nome do projeto]**.

## Objetivo

<<SUA FALA>>

## Regras (inegociáveis)

1. **Migration versionada** no padrão do projeto, nunca ALTER manual direto no banco.
2. **Rollback definido**: toda migration com o caminho de volta. Se a operação for irreversível (DROP de coluna com dados), o plano precisa dizer isso em negrito e esperar minha confirmação.
3. **Dados existentes preservados**: renomear = criar nova + copiar + validar + só então remover a antiga. Nada de perder dados no meio.
4. Compatibilidade: a versão atual da aplicação continua funcionando com o esquema novo? Se não, descreva a ordem correta de deploy.
5. Migration testada num banco com **dados de exemplo**, não só vazio.

## Plano esperado

1. Estado atual do esquema (mostre o relevante).
2. A mudança proposta e a migration.
3. Simulação: rodar `up`, validar os dados, rodar `down`, validar que voltou.

## Critérios de aceitação

- [ ] `up` e `down` rodam limpos num banco com dados.
- [ ] Nenhuma perda de dados não autorizada.
- [ ] Testes do projeto verdes com o esquema novo.

## Formato do relatório final

SQL final da migration (up/down), o que acontece com os dados existentes, e ordem de deploy se houver dependência com o código.
