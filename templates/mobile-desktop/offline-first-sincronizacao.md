# Offline-first e sincronização

> Modo: `technical` · Área: mobile/desktop
> Uso: implementar dados offline com sincronização confiável e conflito resolvido de propósito, não por acidente.

---

Implemente suporte offline-first com sincronização no projeto **[nome do projeto]**.

## Escopo

<<SUA FALA>>

## Contexto a preencher

- Armazenamento local: [SQLite / Realm / WatermelonDB / outro] — justifique pelo padrão de consulta e volume, não por moda.
- Entidades que precisam funcionar offline: [lista] — o resto fica explicitamente fora do escopo.
- Backend de sync: [API própria / serviço pronto].

## Requisitos técnicos

1. **Fila de mutações pendentes**: toda escrita offline entra numa fila persistida (sobrevive a fechar o app), com ordem preservada e idempotência — reenviar a mesma mutação não pode duplicar dados no servidor.
2. **Resolução de conflito ESCOLHIDA explicitamente, por entidade**: last-write-wins (descarta a edição mais antiga silenciosamente), merge por campo (perde intenção quando dois campos dependem um do outro) ou resolução manual (custa UI e fricção). Cada uma perde dados de um jeito — documente qual perda é aceitável em cada entidade. "Não pensei nisso" é a pior das três.
3. **Indicador de estado na UI**: o usuário sempre sabe se está sincronizado, pendente ou com erro; item com falha de sync mostra o motivo e a ação possível.
4. **Sincronização retomável**: sync interrompida no meio (app morto, rede caiu) continua de onde parou, sem reenviar tudo nem corromper estado — marcador de progresso persistido, não flag em memória.
5. **Erro nunca engolido**: falha de sync registrada e reapresentada; retry com backoff, nunca loop agressivo.
6. Passo destrutivo (limpar base local, descartar fila pendente) só com confirmação explícita do usuário.

## Validações

- Teste automatizado: criar dado offline → derrubar a conexão no MEIO do sync → reconectar → verificar que nada duplicou nem se perdeu.
- Teste do conflito: editar o mesmo registro em duas sessões/dispositivos e verificar que o resultado é o documentado, não um acidente.
- Rode a suíte completa: [comando de teste].

## Critérios de aceitação

- [ ] Mutações offline persistem, sincronizam em ordem e são idempotentes.
- [ ] Estratégia de conflito documentada por entidade, com o tipo de perda aceito.
- [ ] UI mostra o estado de sync (sincronizado/pendente/erro) em tempo real.
- [ ] Sync interrompida no meio retoma sem duplicar nem perder dados (teste prova).
- [ ] Nenhuma operação destrutiva sem confirmação explícita.

## Formato do relatório final

Arquitetura da fila e do sync (diagrama simples), estratégia de conflito por entidade com justificativa, testes criados com resultado, e limitações conhecidas.
