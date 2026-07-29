# Observabilidade e logs

> Modo: `technical` · Área: DevOps
> Uso: logging estruturado, métricas e alertas para saber o que acontece em produção.

---

Melhore a observabilidade do projeto **[nome do projeto]**.

## Objetivo

<<SUA FALA>>

## Requisitos técnicos

- **Logs estruturados** (JSON ou formato do stack): timestamp, nível, mensagem, contexto (request id, user id anonimizado).
- Níveis com critério: `error` = precisa de ação, `warn` = anormal mas recuperado, `info` = evento de negócio, `debug` = desligado em produção.
- **Nunca logar**: senha, token, dado de cartão, conteúdo sensível do usuário. Adicione filtro/redação se necessário.
- Todo `catch` loga com contexto suficiente para debugar sem reproduzir — nada de erro engolido.
- Correlação: uma requisição rastreável de ponta a ponta pelo mesmo id.
- [Métricas: latência, taxa de erro, throughput dos endpoints críticos]
- [Alertas: taxa de erro > X% em Y minutos → notificar em [canal]]

## Restrições

- Overhead mínimo: nada de serializar objetos gigantes em log de caminho quente.
- Use a lib de logging já existente no projeto, se houver.

## Critérios de aceitação

- [ ] Fluxo crítico rastreável nos logs de ponta a ponta (demonstre com um exemplo real).
- [ ] Nenhum dado sensível nos logs (mostre o teste/filtro que garante).
- [ ] Erros propositais aparecem com stack e contexto.

## Formato do relatório final

O que foi instrumentado, exemplo real de log de uma requisição, e como consultar/filtrar em produção.
