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
- **Antes de repetir qualquer chamada que ESCREVE, resolva a idempotência**: em timeout ou 5xx você não sabe se o outro lado processou — repetir pode cobrar, enviar e-mail ou criar registro duas vezes. Use a chave de idempotência da API (se existir) ou um identificador seu por operação; se a API não oferecer nenhum mecanismo, **repita apenas leituras** e trate a escrita como incerta (consulte o estado antes de tentar de novo).
- Defina timeout explícito em toda chamada; nada de esperar para sempre.

## Restrições

- A aplicação deve continuar funcional (degradar com aviso) se o serviço externo estiver fora do ar — nada de quebrar tudo por dependência externa.
- Nenhum dado sensível do usuário enviado ao serviço além do estritamente necessário.

## Validações

- Testes com o client mockado: sucesso, timeout, 429, 5xx, resposta malformada.
- Um teste de contrato ou script manual documentado para validar contra a API real.

## Formato do relatório final

Módulo criado, variáveis de ambiente necessárias, como testar com credenciais reais, e limites conhecidos (rate limits, custos).
