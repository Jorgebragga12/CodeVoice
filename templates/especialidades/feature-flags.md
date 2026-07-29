# Feature flags

> Modo: `technical` · Área: especialidades
> Uso: introduzir feature flags com rollout gradual, kill switch e processo de limpeza.

---

Implemente feature flags no projeto **[nome do projeto]**.

## Escopo

<<SUA FALA>>

## Requisitos técnicos

1. **Avaliação server-side como fonte da verdade**: o cliente pode cachear a decisão, mas quem decide é o servidor — flag avaliada só no cliente é burlável e desincroniza.
2. **Rollout gradual**: por porcentagem ([%], com bucketing estável por usuário — o mesmo usuário sempre cai do mesmo lado) e por segmento ([interno/beta/região]).
3. **Kill switch instantâneo**: desligar a flag desativa o caminho novo em no máximo [tempo], sem deploy — isso é metade do valor da flag; documente como acionar.
4. **Default seguro na falha**: se o serviço de flags está fora, cada flag cai para seu default declarado ([geralmente o caminho antigo]) — indisponibilidade do serviço de flags nunca derruba o app.
5. **Toda flag nasce com dono e data de expiração** registrados em [arquivo/ferramenta de registro]: flag esquecida é código morto disfarçado. Processo de limpeza obrigatório: passada a data, o dono remove a flag e o caminho perdedor; revisão periódica ([cadência]) cobra isso.
6. **Teste dos DOIS caminhos enquanto a flag existir**: a suíte roda os fluxos afetados com a flag ligada e desligada — o caminho desligado de hoje é o que estará ligado amanhã.
7. **Log da variante por request**: cada request registra qual variante viu ([campo no log/evento]) — sem isso é impossível correlacionar erro ou métrica com o rollout.

## Critérios de aceitação

- [ ] Decisão de flag vem do servidor; bucketing estável por usuário (teste)
- [ ] Kill switch desativa o caminho novo sem deploy (acionado de verdade, com tempo medido)
- [ ] Serviço de flags derrubado → app segue no default declarado (teste)
- [ ] Flags registradas com dono, data de expiração e default
- [ ] Suíte roda os fluxos afetados com flag ligada E desligada
- [ ] Variante visível no log de cada request afetado

## Formato do relatório final

Flags criadas (dono, expiração, default), como acionar o kill switch, comportamento medido na falha do serviço de flags, e onde ver a variante por request nos logs.
