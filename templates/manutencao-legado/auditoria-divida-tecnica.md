# Auditoria de dívida técnica

> Modo: `code_review` · Área: manutenção
> Uso: inventariar dívida técnica com critério e sair com prioridades acionáveis.

---

Audite a dívida técnica do projeto **[nome do projeto]** sem alterar código.

## Escopo

<<SUA FALA>>

## Regras da auditoria

1. Cada item registra: **localização** (arquivo/módulo), **impacto concreto** (bugs que já causou ou vai causar, velocidade que rouba do time, risco de segurança/dados), **custo estimado** de resolver (P/M/G) e **prioridade = impacto ÷ custo**.
2. **Impacto exige evidência, não opinião**: bug rastreável ao item, mudança recente que demorou por causa dele, área que todo mundo evita tocar. Sem evidência, o item entra como "suspeita" e não disputa o top 10.
3. **Separe dívida real de preferência estética.** Código feio que funciona, tem teste e ninguém precisa mexer NÃO é prioridade — vai para a lista de "ignorar por enquanto", com o motivo.
4. Use as fontes objetivas disponíveis: histórico do git (arquivos que mais mudam juntos e sozinhos), cobertura de testes, avisos de deprecação, dependências desatualizadas com CVE conhecido.
5. Não proponha reescrita geral. Cada item do top 10 vem com um primeiro passo pequeno e executável, não com "refatorar o módulo".

## Critérios de aceitação

- [ ] Todo item do top 10 tem localização, impacto com evidência, custo e primeiro passo.
- [ ] Existe lista explícita do que ignorar e por quê.
- [ ] Nenhum item sustentado apenas por "o código está feio".
- [ ] Nenhum arquivo do projeto foi modificado.

## Formato do relatório final

Top 10 priorizado em tabela (item, localização, impacto, custo, prioridade, primeiro passo), lista de suspeitas sem evidência para investigar depois, e a lista do que explicitamente ignorar.
