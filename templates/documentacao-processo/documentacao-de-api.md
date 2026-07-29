# Documentação de API

> Modo: `technical` · Área: documentação
> Uso: documentar uma API para consumidores externos, com exemplos reais e guia de primeiros passos.

---

Documente a API do projeto **[nome do projeto]** para consumidores externos.

## Escopo

<<SUA FALA>>

## Contexto a preencher

- Endpoints/áreas a cobrir: [todos | lista]
- Onde a doc vive: [pasta/ferramenta — ex.: docs/, Swagger UI, portal]
- Ambiente para gerar exemplos: [URL base + como obter credencial de teste]

## Requisitos da documentação

1. **Exemplos reais, não de memória.** Gere cada exemplo de request/response CHAMANDO a API no ambiente de teste — exemplo escrito de cabeça diverge do contrato real.
2. Para cada endpoint: método + rota, parâmetros com tipo e obrigatoriedade, request e response de exemplo, e **erros possíveis com corpo de exemplo** (no mínimo: validação, não autorizado, não encontrado).
3. **Autenticação passo a passo:** como obter a credencial, como enviá-la no request, o que acontece quando expira.
4. **Rate limits:** limites, headers de resposta e comportamento ao estourar. Se não existir, escreva "sem limite definido" — silêncio vira suposição errada do consumidor.
5. **Guia de primeiros passos** que leva do zero à primeira chamada bem-sucedida em minutos: um fluxo só, comandos copiáveis (curl ou similar).
6. **OpenAPI como fonte da verdade** quando houver spec (ou for viável gerá-la do código): derive a doc dela em vez de manter dois textos que divergem com o tempo.
7. **Changelog de contrato:** seção com mudanças que afetam consumidores (campo removido, tipo alterado, rota deprecada), com data e versão.

## Regras

- Mascare qualquer token ou segredo real nos exemplos ([token]).
- Se um endpoint responder diferente do que o código sugere, reporte a divergência — não documente o comportamento "esperado".

## Critérios de aceitação

- [ ] Todo exemplo de response foi gerado por chamada real (registre as chamadas feitas).
- [ ] Cada endpoint documenta seus erros possíveis com corpo de exemplo.
- [ ] O guia de primeiros passos foi executado de ponta a ponta e termina em resposta de sucesso.
- [ ] Nenhum segredo real aparece na documentação.

## Formato do relatório final

Endpoints documentados, chamadas feitas para gerar os exemplos, divergências encontradas entre código e comportamento real, e o que ficou pendente.
