# Autenticação e login

> Modo: `new_feature` · Área: desenvolvimento
> Uso: login, sessão, cadastro, recuperação de senha, permissões.

---

Implemente autenticação no projeto **[nome do projeto]**.

## Objetivo

<<SUA FALA>>

## Requisitos técnicos (segurança primeiro)

- **Nunca** armazene senha em texto puro: use [bcrypt/argon2] com salt.
- Prefira biblioteca/provedor consolidado ([ex.: NextAuth, Supabase Auth, Auth0]) a implementação caseira — só implemente do zero se eu pedir explicitamente.
- Sessão: [JWT curto + refresh / cookie httpOnly + SameSite] — cookie httpOnly por padrão em app web.
- Mensagem de erro de login genérica ("credenciais inválidas") — não revele se o e-mail existe.
- Rate limit no login e na recuperação de senha.
- Rotas protegidas verificadas **no servidor**; esconder botão no cliente não é proteção.

## Requisitos funcionais

- Login, logout, [cadastro], [recuperação de senha por e-mail], [níveis de permissão: admin/usuário].
- Sessão sobrevive a refresh da página; expira em [tempo].

## Validações

- Testes: login correto, senha errada, usuário inexistente, acesso a rota protegida sem sessão, sessão expirada.

## Critérios de aceitação

- [ ] Fluxo completo funcional de ponta a ponta.
- [ ] Nenhuma credencial ou token em log.
- [ ] Testes e lint verdes.

## Formato do relatório final

Fluxo implementado, decisões de segurança tomadas, variáveis de ambiente novas, e o que eu preciso configurar (ex.: SMTP, provedor OAuth).
