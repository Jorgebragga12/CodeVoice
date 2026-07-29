# Testes de integração

> Modo: `technical` · Área: testes
> Uso: testar o código contra dependências reais (banco, fila) em vez de mocks.

---

Implemente testes de integração com dependências reais no projeto **[nome do projeto]**.

## Objetivo

<<SUA FALA>>

## Contexto a preencher

- Dependências reais envolvidas: [banco/fila/cache e versão]
- Ferramenta de infra efêmera: [testcontainers/docker compose/outra]
- Comando de migrations: [comando]

## Regras técnicas

1. **Banco real efêmero, não mock do repositório.** Suba [banco e versão] via [testcontainers/docker] para a suíte; mock de repositório não prova constraint, transação nem o SQL real.
2. **Migrations no setup.** O schema de teste nasce das mesmas migrations de produção — nunca de um `CREATE TABLE` paralelo que diverge em silêncio.
3. **Isolamento total.** Cada teste cria seus próprios dados e limpa depois (transação com rollback ou truncate); a suíte passa em qualquer ordem e sem depender de execução anterior.
4. **Teste o contrato real:** constraints de unicidade/FK violadas, comportamento em transação (commit/rollback) e pelo menos um caso de concorrência se houver escrita concorrente no código.
5. **Tempo controlado.** Reuse o container entre testes (setup uma vez por suíte); se a suíte estourar [limite de tempo], reporte antes de continuar adicionando testes.

## Fronteira unidade vs integração

Defina e documente o critério deste projeto: [ex.: "toca I/O real = integração; lógica pura = unidade"]. Não duplique o mesmo cenário nas duas camadas.

## Critérios de aceitação

- [ ] Nenhum mock de repositório onde há banco efêmero disponível.
- [ ] Migrations de produção rodam no setup dos testes.
- [ ] Suíte passa com ordem aleatória de execução.
- [ ] Constraints e transações têm testes explícitos.
- [ ] Tempo total da suíte dentro do limite declarado.

## Formato do relatório final

O que foi coberto, tempo da suíte, o critério unidade vs integração adotado, e lacunas conhecidas (o que ficou sem teste e por quê).
