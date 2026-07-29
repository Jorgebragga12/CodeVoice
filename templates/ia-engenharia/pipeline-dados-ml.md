# Pipeline de dados

> Modo: `technical` · Área: engenharia de IA
> Uso: construir pipeline de dados para analytics/ML com validação na entrada e reprocessamento seguro.

---

Construa um pipeline de dados no projeto **[nome do projeto]**.

## Escopo

<<SUA FALA>>

## Contexto a preencher

- Fonte(s) dos dados: [API / banco / arquivos / eventos]
- Destino: [warehouse / lake / feature store / tabela]
- Frequência: [batch diário / horário / streaming]
- Orquestração: [ferramenta — ou derivar do que o projeto já usa]

## Requisitos técnicos

1. **Validação de schema NA ENTRADA**: dado ruim para na porta — registro inválido vai para quarentena com o motivo registrado, nunca é descartado silenciosamente nem contamina o destino.
2. **Transformações idempotentes e re-executáveis**: rodar duas vezes o mesmo período produz o mesmo resultado, sem duplicar linhas; cada passo pode ser reprocessado isoladamente.
3. **Versionamento de dados e de schema**: mudança de schema é registrada e a compatibilidade verificada antes do deploy; dá para saber com qual versão cada lote foi processado.
4. **Backfill planejado desde o início**: reprocessar histórico é operação prevista, com comando próprio — e backfill que sobrescreve dado existente exige confirmação explícita.
5. **Monitoramento de frescor e volume**: alerta quando a fonte para de mandar dado ou o volume foge do esperado — pipeline que falha em silêncio é pior que pipeline quebrado.
6. **Linhagem documentada**: de cada tabela/coluna do destino dá para rastrear a fonte e as transformações aplicadas.

## Validações

- Teste as transformações com dados de amostra versionados (casos válidos, inválidos e de borda), rodando com [comando de teste].
- Teste de idempotência: rode o mesmo lote duas vezes e compare o destino.
- Simule fonte vazia ou atrasada e confirme que o alerta dispara.

## Critérios de aceitação

- [ ] Registro inválido cai na quarentena com o motivo registrado.
- [ ] Reprocessar um período não duplica nem corrompe dado no destino.
- [ ] Alerta de frescor/volume dispara na simulação.
- [ ] Backfill documentado e testado em um período pequeno.
- [ ] Testes de transformação passam com [comando de teste].

## Formato do relatório final

Desenho do pipeline (fonte → transformações → destino), decisões de schema/versionamento, e resultado dos testes de idempotência e de alerta.
