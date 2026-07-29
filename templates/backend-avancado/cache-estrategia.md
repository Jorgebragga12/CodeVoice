# Estratégia de cache

> Modo: `technical` · Área: backend
> Uso: introduzir cache em ponto quente do sistema sem criar bugs de dado velho.

---

Introduza cache no projeto **[nome do projeto]**.

## Problema

<<SUA FALA>>

## Antes de escrever qualquer cache

1. **Meça o custo real do que quer cachear** (latência ou nº de queries por request, ANTES). Sem medição não há justificativa para o cache — e não haverá como provar o ganho depois.
2. **Defina a invalidação ANTES de escrever o cache.** Para cada chave: o que a escreve, o que a invalida e em qual evento. Cache sem plano de invalidação explícito é bug agendado.

## Regras de implementação

1. Invalidação explícita nos pontos de escrita do dado; **TTL é rede de segurança, não a estratégia** ([valor] como teto para dado que escapou da invalidação).
2. Proteja contra **cache stampede**: quando a chave expira sob carga, só uma execução recalcula (lock/single-flight); as demais esperam ou recebem o valor anterior.
3. Exponha **métricas de hit/miss** por chave ou grupo — hit rate baixo é complexidade sem retorno, e deve levar à remoção do cache.
4. Chaves com prefixo versionado ([prefixo]), para invalidar tudo num deploy que muda o formato do valor guardado.
5. Falha na infra de cache ([Redis fora, etc.]) degrada para a fonte original com log — nunca derruba a request nem falha em silêncio.

## O teste que decide

- **Com o cache desligado, o sistema continua correto** (mais lento, mas correto). Se desligar o cache quebra funcionalidade, o cache virou fonte da verdade — erro de design que precisa ser corrigido antes de seguir.

## Critérios de aceitação

- [ ] Medição antes/depois registrada no relatório (latência ou nº de queries)
- [ ] Cada chave tem invalidação explícita mapeada para os pontos de escrita
- [ ] Teste: escrever o dado → ler em seguida → recebe o valor novo, não o cacheado
- [ ] Stampede tratado (teste, ou justificativa de por que não se aplica ao caso)
- [ ] Sistema funciona correto com o cache desabilitado
- [ ] Métricas de hit rate expostas

## Formato do relatório final

O que foi cacheado e por quê (com números antes/depois), tabela chave → evento de invalidação, TTLs escolhidos, e como desligar o cache em emergência.
