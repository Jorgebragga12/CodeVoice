# Mutation testing

> Modo: `technical` · Área: testes
> Uso: medir se a suíte de testes realmente detecta defeitos, não só executa linhas.

---

Meça a força da suíte de testes do projeto **[nome do projeto]** com **[Stryker/mutmut/cargo-mutants]**.

## Objetivo

<<SUA FALA>>

## Contexto a preencher

- Módulos críticos priorizados: [lista — regra de negócio, dinheiro, segurança]
- Meta de mutation score por módulo crítico: [ex.: percentual]
- Comando da suíte: [comando]

## Regras da execução

1. **Rode nos módulos críticos primeiro.** Mutation testing é caro (re-executa a suíte por mutante); rodar no projeto inteiro de cara desperdiça horas e afoga o resultado em ruído.
2. **Meta por módulo crítico, não score global.** Score global mistura código trivial com código de risco e vira número de vaidade.
3. **Cada mutante sobrevivente é uma lacuna real:** ou vira um teste novo que o mata, ou uma justificativa escrita (mutante equivalente, código morto). Nunca ignore em bloco.
4. Não escreva teste que só "mata o mutante" sem afirmar comportamento — o teste novo deve falhar por uma razão de negócio compreensível.
5. **CI (opcional):** se integrar, rode apenas sobre o código alterado no PR — execução completa a cada build é inviável em projeto real.

## O aviso que justifica a tarefa

Cobertura de linha alta com mutation score baixo = suíte que executa o código mas não verifica nada: ela finge testar. Se encontrar esse quadro, aponte exatamente onde.

## Critérios de aceitação

- [ ] Relatório de mutantes gerado para todos os módulos críticos listados.
- [ ] Cada sobrevivente tratado: teste novo ou justificativa escrita.
- [ ] Meta de score atingida por módulo crítico (ou plano para as pendências).
- [ ] Nenhum teste novo sem asserção de comportamento.
- [ ] Comparativo cobertura de linha × mutation score incluído no relatório.

## Formato do relatório final

Score por módulo (antes/depois dos testes novos), lista de sobreviventes com o destino de cada um, e o veredito: a suíte detecta defeitos ou só executa linhas?
