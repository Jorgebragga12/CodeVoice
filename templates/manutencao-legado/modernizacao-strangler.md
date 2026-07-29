# Modernização incremental (strangler)

> Modo: `refactor` · Área: manutenção
> Uso: substituir um módulo legado aos poucos, sem reescrever tudo de uma vez.

---

Modernize o módulo **[módulo legado]** do projeto **[nome do projeto]** usando o padrão strangler fig.

## Objetivo

<<SUA FALA>>

## Regras da migração

1. **Fachada primeiro**: todo acesso ao legado passa a entrar por uma fachada/roteador único ANTES de qualquer código novo existir — sem esse ponto de corte não há como rotear caso a caso.
2. **Migre caso a caso**: escolha o próximo por [critério: mais simples primeiro / mais valioso primeiro], implemente no código novo e roteie só ele. Um caso por vez.
3. **Shadow antes de cortar**: rode novo e legado em paralelo para o caso, compare as saídas com log de divergência por [janela/volume], e só corte o tráfego com divergência zero ou explicada e aceita. **Shadow só é seguro em caminho sem efeito colateral** — se o caso escreve, cobra, envia e-mail ou dispara webhook, rodar os dois em paralelo duplica o efeito no mundo real. Nesse caso: rode o novo com as escritas/integrações desligadas (dry-run) e compare o que ELE TERIA feito com o que o legado fez, em vez de deixar os dois agirem.
4. **O legado continua funcionando durante TODA a migração.** Nenhuma etapa pode quebrá-lo, e o rollback de qualquer caso é reverter o roteamento (flag/uma linha), nunca um deploy de emergência.
5. Métricas de progresso visíveis: % de casos e % de tráfego no código novo, atualizadas a cada corte — sem isso a migração perde tração e vira eterna.
6. **O passo final é deletar o legado** e o código de comparação — planejado desde o início, senão viram dois sistemas mantidos para sempre.
7. Deletar o legado é passo destrutivo: só com confirmação explícita e após [período] com zero tráfego no caminho antigo.

## Critérios de aceitação

- [ ] Fachada única cobrindo 100% dos acessos ao legado.
- [ ] Cada caso migrado passou por shadow com divergências zeradas ou explicadas.
- [ ] Rollback por roteamento testado ao menos uma vez.
- [ ] % de tráfego no novo registrada a cada corte.
- [ ] Plano de deleção do legado escrito, com gatilho definido.

## Formato do relatório final

Casos migrados vs. restantes, % de tráfego no novo, divergências encontradas no shadow (e como foram resolvidas), e o que falta para deletar o legado.
