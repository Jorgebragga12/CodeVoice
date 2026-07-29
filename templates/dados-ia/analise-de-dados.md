# Análise de dados

> Modo: `technical` · Área: dados
> Uso: explorar um conjunto de dados e responder uma pergunta com evidência.

---

Analise os dados em **[fonte: tabela/arquivo/planilha]** e responda:

## Pergunta

<<SUA FALA>>

## Plano esperado

1. **Perfil dos dados primeiro**: quantas linhas, colunas, tipos, nulos por coluna, duplicatas, valores fora do esperado. Reporte problemas de qualidade ANTES de concluir qualquer coisa.
2. Definir a métrica que responde a pergunta (e dizer explicitamente qual definição usou — ex.: "usuário ativo = login nos últimos 30 dias").
3. Calcular, com o código/consulta visível para eu auditar.
4. Verificar o resultado por um segundo caminho quando possível (ex.: total por soma das partes).

## Regras

- **Conclusão separada de especulação**: o que os dados mostram vs. o que você supõe.
- Números com contexto: "cresceu 20%" precisa de base ("de 100 para 120 no período X").
- Se os dados não sustentam uma resposta confiável, diga isso — não force conclusão.
- Cuidado com médias escondendo distribuição: reporte mediana/percentis quando fizer diferença.

## Formato do relatório final

Resposta direta à pergunta em 1–2 frases primeiro, depois: método, números de suporte, ressalvas de qualidade dos dados, e o código/SQL usado.
