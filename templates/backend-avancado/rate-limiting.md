# Rate limiting

> Modo: `technical` · Área: backend
> Uso: limitar taxa de requisições para proteger a API de abuso e sobrecarga.

---

Implemente rate limiting no projeto **[nome do projeto]**.

## Objetivo

<<SUA FALA>>

## Decisões a registrar

1. **Algoritmo**: [token bucket ou sliding window] — janela fixa pura deixa passar rajada dupla na virada da janela; justifique a escolha.
2. **Onde aplicar**: [gateway/proxy ou middleware da aplicação] — quanto mais na borda, mais barato rejeitar.
3. **Armazenamento dos contadores**: [Redis/memória] — com múltiplas instâncias, contador em memória local não limita nada de verdade.

## Chave de limite

- Ordem de preferência: [API key > usuário autenticado > IP]. IP é só fallback para tráfego anônimo.
- **Cuidado com IP**: NAT/CGNAT e proxies corporativos colocam milhares de usuários atrás do mesmo endereço. Leia o IP real de [X-Forwarded-For] apenas se vier de proxy confiável — senão o cliente falsifica o header e escapa do limite.

## Comportamento no limite

1. Resposta **429** com header **Retry-After** dizendo quando tentar de novo.
2. Headers informativos nas respostas ([RateLimit-Limit, RateLimit-Remaining, RateLimit-Reset]) para o cliente se auto-regular antes de bater no teto.
3. **Limites por rota e por plano**: [login e endpoints caros mais restritos; planos pagos com teto maior]. Um número global único quase nunca serve.
4. **Allowlist** para tráfego interno ([health checks, serviços próprios]) — para não se auto-derrubar.
5. Toda rejeição gera **métrica/log com a chave limitada** — é o que separa ataque de cliente legítimo mal configurado, e permite ajustar limites com dado em vez de chute.
6. Defina o comportamento com o storage de contadores fora do ar: [fail-open ou fail-closed] e por quê — para rate limiting, fail-open costuma ser o padrão certo, já que indisponibilidade total é pior que abuso temporário.

## Critérios de aceitação

- [ ] Teste automatizado: [N] requests passam e a seguinte recebe 429 com Retry-After
- [ ] Headers informativos presentes nas respostas
- [ ] Limites diferentes por rota/plano funcionando (teste em pelo menos duas rotas)
- [ ] Tráfego da allowlist não é limitado
- [ ] Métricas expostas mostrando quem está sendo limitado e em qual rota
- [ ] Comportamento com o storage fora do ar definido, implementado e testado

## Formato do relatório final

Algoritmo e ponto de aplicação, tabela rota → limite → chave, decisão fail-open/fail-closed, e a saída do teste que prova o 429.
