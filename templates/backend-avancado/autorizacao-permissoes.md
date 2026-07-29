# Autorização e Permissões (RBAC)

> Modo: `new_feature` · Área: backend
> Uso: definir quem pode fazer o quê no sistema e garantir isso no backend, não só na tela.

---

Implemente autorização por papéis e permissões no projeto **[nome do projeto]**.

## Escopo

<<SUA FALA>>

## Matriz de acesso (preencha antes de codar)

| Papel | Recurso | Ações permitidas |
|---|---|---|
| [admin] | [recurso] | [criar/ler/editar/excluir] |
| [membro] | [recurso] | [ler/editar próprios] |

- Escopo dos dados: acesso é limitado por [organização/time/dono do recurso]? Papel certo + recurso de outra org = negado.
- Se houver regra por atributo além do papel ([status do recurso, horário, plano]), descreva — isso é ABAC e muda o desenho da checagem.

## Requisitos técnicos

1. **Negar por padrão**: rota sem regra explícita de acesso é rota bloqueada. O erro seguro é negar acesso legítimo (alguém reclama), não liberar acesso indevido (ninguém avisa).
2. **Enforcement centralizado no backend** ([middleware/guard/policy]), nunca `if` espalhado por controller — regra espalhada é regra que alguém esquece de aplicar na próxima rota.
3. **Permissões nomeadas por ação** ([`recurso:acao`], ex.: `fatura:excluir`); papéis são agrupamentos de permissões. Código checa permissão, não papel — assim criar papel novo não exige mexer no código.
4. **Checagem no nível do recurso**, não só da rota: carregou o objeto, valide dono/organização antes de agir — é isso que impede IDOR (trocar o ID na URL e acessar dado alheio).
5. **Acesso negado**: 403 com mensagem genérica; recurso de outra organização responde [404] para não confirmar que o ID existe.
6. Mudança de papel do usuário vale [imediatamente ou no próximo login] — defina e implemente a invalidação de [sessão/cache/token] correspondente.

## Reflexo na UI

- Esconder/desabilitar ações que o usuário não pode executar, derivando das mesmas permissões do backend (endpoint tipo [`/me/permissions`]) — nunca uma lista duplicada no frontend.
- UI é conveniência, não segurança: toda ação escondida na tela continua bloqueada na API.

## Critérios de aceitação

- [ ] Matriz papel × ação implementada e coberta por teste: cada célula tem um teste de permitido E um de negado
- [ ] Requisição autenticada a recurso de outra organização/dono é negada (teste automatizado)
- [ ] Rota nova sem regra declarada nasce bloqueada (teste provando o default)
- [ ] Acesso negado gera log com usuário, rota e recurso — sem vazar detalhe na resposta
- [ ] UI esconde/desabilita ações não permitidas usando as permissões vindas do backend

## Formato do relatório final

Matriz final papel × recurso × ação, onde fica o enforcement (arquivo do middleware/policy), decisão 403/404, como a UI consome as permissões, e a lista de testes de autorização com resultado.
