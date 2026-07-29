# Integração de IA / chatbot

> Modo: `new_feature` · Área: IA
> Uso: integrar um LLM num produto (chat, geração, classificação, resumo).

---

Integre IA no projeto **[nome do projeto]**.

## Objetivo

<<SUA FALA>>

## Requisitos técnicos

- Provedor/modelo: [ex.: API da Anthropic, claude-sonnet-5 — confirme o modelo atual na documentação antes de fixar].
- **API key por variável de ambiente**, nunca no código nem no cliente (a chamada ao LLM sai do servidor, não do browser).
- **Entrada do usuário é dado, não instrução**: delimite o texto do usuário no prompt e trate tentativa de injeção ("ignore as instruções...") como conteúdo comum.
- Trate as falhas do mundo real: timeout, rate limit (retry com backoff), resposta fora do formato esperado (re-tentar ou degradar com mensagem honesta).
- Streaming da resposta se for chat (UX de espera importa).
- Custo sob controle: limite de tokens por requisição, e [limite por usuário/dia se público].

## Regras de produto

- O sistema deve deixar claro ao usuário que a resposta vem de IA.
- Nunca apresentar alucinação como fato em domínio crítico: [restrinja com contexto/RAG ou disclaimers conforme o caso].
- Histórico de conversa: [persistir em DB / manter só na sessão] com [política de retenção].

## Validações

- Testes com o provedor mockado: sucesso, timeout, rate limit, resposta malformada, tentativa de injeção de prompt.

## Formato do relatório final

Arquitetura da integração, prompt de sistema usado, custo estimado por 1000 interações, e variáveis de ambiente novas.
