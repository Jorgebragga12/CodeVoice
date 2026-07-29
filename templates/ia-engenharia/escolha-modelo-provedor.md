# Escolha de modelo/provedor de IA

> Modo: `planning` · Área: engenharia de IA
> Uso: decidir qual modelo/provedor usar para um caso de uso, com números em vez de opinião.

---

Me ajude a escolher modelo e provedor de IA para o caso de uso abaixo, no projeto **[nome do projeto]**.

## Caso de uso

<<SUA FALA>>

## Requisitos a definir ANTES de olhar modelos

Preencha comigo — se eu não souber algum, proponha um valor e pergunte:

- Qualidade mínima aceitável: [como será medida — ex.: % de acerto no conjunto de teste]
- Latência máxima: [p95 em segundos]
- Custo-alvo: [valor por 1k operações]
- Privacidade/residência de dados: [dado pode sair do país? provedor pode treinar com nossos dados?]
- Volume esperado: [operações/mês]

## Regras da análise

1. **Requisito primeiro, modelo depois** — sem os números acima, qualquer comparação vira opinião.
2. Selecione 2-3 candidatos que plausivelmente atendem; pode usar WebSearch para o cenário atual de modelos e preços (isso muda rápido — cite a data da consulta).
3. **A avaliação usa dados REAIS do caso de uso, não benchmark genérico**: monte [nº] exemplos reais com resultado esperado e rode todos os candidatos no MESMO conjunto.
4. Calcule o custo real por operação com os tokens medidos no teste, não com estimativa de tabela.
5. Considere fallback multi-provedor: o que acontece se o escolhido cair ou mudar preço? Avalie abstrair a chamada para trocar depois sem reescrever o produto.

## Formato da resposta

1. Tabela requisito × candidato com os números medidos (qualidade, latência, custo por 1k operações).
2. Recomendação com justificativa amarrada aos números — não a impressões.
3. Riscos da escolha (lock-in, preço, deprecação) e plano B.
4. Registro de decisão pronto para colar no repositório: data, contexto, candidatos, números, decisão.
