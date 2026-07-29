# Atualização de dependências

> Modo: `technical` · Área: DevOps
> Uso: upgrade seguro de bibliotecas, sem quebrar o projeto.

---

Atualize as dependências do projeto **[nome do projeto]**.

## Escopo

<<SUA FALA>>

## Plano esperado

1. Listar dependências desatualizadas ([npm outdated / cargo outdated]) separando: patch, minor, major.
2. Rodar auditoria de segurança ([npm audit / cargo audit]) e priorizar vulnerabilidades.
3. Atualizar em lotes, do menor risco para o maior: patches → minors → majors (um major por vez).
4. **Para cada major**: ler o changelog/guia de migração ANTES de atualizar; listar breaking changes que afetam este projeto.
5. Rodar a suíte completa + build após **cada lote**, não só no final.

## Regras

- Se um major exigir refatoração grande, **pare e me reporte** o custo antes de fazer — pode não valer a pena agora.
- Lockfile atualizado e commitado junto.
- Nenhuma dependência nova adicionada "de carona".
- Se algo quebrar e a causa não for óbvia em [15] minutos de investigação, reverta aquele item e registre como pendência.

## Critérios de aceitação

- [ ] Zero vulnerabilidades de severidade alta/crítica restantes (ou justificadas uma a uma).
- [ ] Testes, lint e build verdes.
- [ ] Aplicação testada manualmente no fluxo principal.

## Formato do relatório final

Tabela: dependência, versão antes → depois, motivo (segurança/rotina), e o que ficou pendente com o porquê.
