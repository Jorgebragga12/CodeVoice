# Integração com API externa

> Modo: `new_feature` · Área: desenvolvimento
> Uso: consumir um serviço de terceiros (pagamento, e-mail, mapas, IA…).

---

Integre o serviço **[nome do serviço]** ao projeto **[nome do projeto]**.

## Objetivo

<<SUA FALA>>

## Requisitos técnicos

- Leia a documentação oficial da API antes de escrever código; não invente endpoints nem parâmetros.
- **Credenciais NUNCA no código**: use variável de ambiente [ex.: SERVICO_API_KEY] e documente no `.env.example`.
- Isole a integração num módulo próprio (um "client") — o resto do código não deve conhecer detalhes da API externa.
- Trate as falhas reais: timeout, 429 (rate limit, com retry e backoff), 5xx, resposta com shape inesperado.
- Defina timeout explícito em toda chamada; nada de esperar para sempre.

## Restrições

- A aplicação deve continuar funcional (degradar com aviso) se o serviço externo estiver fora do ar — nada de quebrar tudo por dependência externa.
- Nenhum dado sensível do usuário enviado ao serviço além do estritamente necessário.

## Validações

- Testes com o client mockado: sucesso, timeout, 429, 5xx, resposta malformada.
- Um teste de contrato ou script manual documentado para validar contra a API real.

## Formato do relatório final

Módulo criado, variáveis de ambiente necessárias, como testar com credenciais reais, e limites conhecidos (rate limits, custos).
