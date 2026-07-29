# Auditoria de segurança

> Modo: `code_review` · Área: segurança
> Uso: varredura defensiva de vulnerabilidades no próprio projeto.

---

Faça uma auditoria de segurança do projeto **[nome do projeto]** (código próprio, com autorização).

## Escopo

<<SUA FALA>>

## Checklist mínimo

1. **Segredos**: chaves/senhas/tokens no código, no histórico do git, em logs ou em mensagens de erro.
2. **Entrada de usuário**: toda entrada validada no servidor? Injeção SQL (queries concatenadas), XSS (render sem escape), path traversal (caminhos vindos do usuário), comando de shell com input.
3. **Autenticação/autorização**: rotas protegidas verificadas no servidor; usuário A consegue acessar dados do usuário B trocando um ID na URL? (IDOR)
4. **Dependências**: rodar [npm audit / cargo audit / pip-audit] e listar vulnerabilidades conhecidas com severidade.
5. **Dados sensíveis**: senha com hash **lento e com salt, próprio para senha** — bcrypt, scrypt ou argon2id, com custo configurado? (SHA-256/SHA-512 e MD5 **reprovam** aqui: são rápidos por design, o que é exatamente o que o atacante quer ao testar bilhões de tentativas por segundo em GPU.) Dados pessoais em log? HTTPS forçado?
6. **Configuração**: CORS aberto demais, cookies sem httpOnly/Secure, headers de segurança ausentes.

## Regras

- Cada achado: local exato, cenário de exploração concreto, severidade (CRÍTICO/ALTO/MÉDIO/BAIXO) e correção.
- **Não corrija nada ainda** — primeiro o relatório; eu decido a ordem.
- Não invente vulnerabilidade teórica sem caminho de exploração real neste código.

## Formato do relatório final

Tabela por severidade, os 3 itens que eu deveria corrigir primeiro, e o comando de auditoria de dependências com sua saída.
