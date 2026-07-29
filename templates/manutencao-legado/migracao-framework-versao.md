# Migração de versão/framework

> Modo: `planning` · Área: manutenção
> Uso: planejar migração de versão major de framework/linguagem sem quebrar o app.

---

Planeje a migração do projeto **[nome do projeto]**: de [versão/framework atual] para [versão/framework alvo].

## Escopo

<<SUA FALA>>

## Contexto

- Comando de teste e estado atual da suíte: [comando + verde/vermelha]
- Cobertura das áreas críticas: [boa / fraca / desconhecida]
- Janela disponível: [prazo ou "sem prazo fixo"]

## Regras do plano

1. **Leia o guia oficial de migração ANTES de planejar.** O plano cita o guia e segue a ordem que ele recomenda — o time do framework já mapeou as armadilhas.
2. **Inventário primeiro**: liste os breaking changes que afetam ESTE projeto, cada um com o arquivo/uso encontrado por busca no código — não a lista genérica do changelog.
3. **Incremental, nunca big bang.** Cada etapa termina com o app funcionando e deployável; se uma etapa não consegue terminar funcionando, quebre-a em duas.
4. Use os codemods oficiais quando existirem; migração manual só para o que o codemod não cobre.
5. **Suíte verde é o portão entre etapas.** Se a suíte não cobre uma área afetada, a etapa começa escrevendo os testes que faltam.
6. Estimativa honesta por etapa, com folga para o que sempre aparece (dependência transitiva incompatível, plugin abandonado).
7. **Critério de abortar definido antes de começar**: em que ponto (custo real vs. estimado, ex.: estourou [múltiplo]) a migração para e o projeto reverte ao último estado estável.

## Formato da resposta

Plano em etapas numeradas — para cada uma: o que muda, breaking changes cobertos, codemod ou manual, como validar (comando), estimativa. No fim: o critério de abortar e o inventário completo de breaking changes aplicáveis como anexo.
