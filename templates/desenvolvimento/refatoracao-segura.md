# Refatoração segura

> Modo: `refactor` · Área: desenvolvimento
> Uso: melhorar estrutura/legibilidade sem mudar comportamento externo.

---

Refatore o seguinte no projeto **[nome do projeto]**.

## Objetivo

<<SUA FALA>>

## Restrições (inegociáveis)

- **O comportamento externo NÃO pode mudar.** Mesmas entradas → mesmas saídas, mesmos efeitos.
- Testes existentes continuam passando sem serem alterados (a menos que testem detalhe interno que deixou de existir — nesse caso, justifique cada mudança de teste).
- Sem dependências novas.
- API pública/assinaturas exportadas só mudam se eu autorizar explicitamente.

## Plano esperado

1. Rodar a suíte de testes ANTES de qualquer mudança e registrar o resultado.
2. Refatorar em passos pequenos e verificáveis, rodando os testes após cada passo.
3. Se a cobertura da área for fraca, escrever testes de caracterização ANTES de refatorar.

## Critérios de aceitação

- [ ] Suíte completa verde antes e depois.
- [ ] Nenhuma mudança de comportamento observável.
- [ ] Código resultante menor ou mais claro que o original (se ficou maior e mais complexo, a refatoração falhou).

## Formato do relatório final

Antes/depois em números (linhas, arquivos), o que melhorou concretamente, e qualquer risco residual.
