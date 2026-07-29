# Revisão de código

> Modo: `code_review` · Área: qualidade
> Uso: revisar mudanças (branch/PR/arquivos) em busca de defeitos reais.

---

Revise o seguinte no projeto **[nome do projeto]**: [branch/PR/arquivos].

## Foco da revisão

<<SUA FALA>>

## O que procurar (em ordem de prioridade)

1. **Bugs reais**: lógica errada, caso de borda não tratado, condição de corrida, off-by-one, null/undefined não verificado.
2. **Segurança**: entrada não validada, injeção (SQL/comando/XSS), segredo no código, dado sensível em log.
3. **Robustez**: erro engolido silenciosamente, falta de timeout, recurso não liberado.
4. **Manutenção**: duplicação real, complexidade desnecessária, nome que mente sobre o que faz.

## Regras da revisão

- Para cada achado: arquivo:linha, o problema, um cenário concreto de falha, e a correção sugerida.
- Classifique por severidade: CRÍTICO / ALTO / MÉDIO / BAIXO.
- **Não** aponte preferência de estilo como defeito.
- Antes de reportar, verifique se o "problema" não é tratado em outro lugar do código.
- Se não encontrar nada grave, diga isso com confiança — não invente problema para parecer útil.

## Formato do relatório final

Achados ordenados por severidade (com arquivo:linha), seguidos de um veredito: pode mergear / mergear após corrigir os críticos / precisa retrabalho.
