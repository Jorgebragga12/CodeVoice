# Suíte de testes

> Modo: `technical` · Área: qualidade
> Uso: criar ou reforçar testes de um módulo/fluxo.

---

Escreva testes para **[módulo/fluxo]** no projeto **[nome do projeto]**.

## Objetivo

<<SUA FALA>>

## Regras de qualidade dos testes

- Teste **comportamento**, não implementação: o teste deve sobreviver a uma refatoração interna.
- Cada teste com nome que descreve o cenário e o resultado esperado.
- Priorize por risco: caminhos com dinheiro/dados do usuário/segurança primeiro.
- Casos de borda obrigatórios: entrada vazia, valor nulo, limite máximo/mínimo, entrada maliciosa, falha de dependência externa (mock).
- **Nenhum teste que passa sempre**: antes de finalizar, quebre de propósito o código testado e confirme que o teste falha.
- Sem dependência entre testes (ordem de execução não pode importar).

## Restrições

- Use o framework de teste já adotado: [ex.: vitest, cargo test, pytest].
- Não altere o código de produção para "facilitar o teste", exceto injeção de dependência justificada.

## Critérios de aceitação

- [ ] Comportamentos principais e casos de borda cobertos.
- [ ] Suíte completa verde e rápida (sem sleep arbitrário/flakiness).
- [ ] Cada teste demonstradamente falha quando o comportamento quebra.

## Formato do relatório final

Lista dos cenários cobertos, o que ficou intencionalmente sem cobrir e por quê, e cobertura antes/depois se disponível.
