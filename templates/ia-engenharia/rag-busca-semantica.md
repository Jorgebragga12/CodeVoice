# RAG e busca semântica

> Modo: `new_feature` · Área: engenharia de IA
> Uso: implementar busca com recuperação (RAG) que responde só com base nos documentos indexados.

---

Implemente RAG (retrieval-augmented generation) no projeto **[nome do projeto]**.

## Objetivo

<<SUA FALA>>

## Contexto a preencher

- Fonte dos documentos: [PDFs / markdown / base de conhecimento / site]
- Modelo de embeddings: [modelo/provedor]
- Armazenamento vetorial: [pgvector / SQLite-vec / serviço gerenciado — comece pelo mais simples que atende]
- Volume estimado: [nº de documentos]

## Requisitos técnicos

1. **Chunking pelo formato do conteúdo, não por tamanho fixo cego.** Respeite seções, títulos e parágrafos; chunk cortado no meio de uma tabela ou lista vira lixo na recuperação.
2. Guarde metadados por chunk (fonte, título, posição) — sem isso não existe citação nem debug.
3. Comece com busca semântica pura; adicione **recuperação híbrida (semântica + keyword)** só se a avaliação mostrar que termos exatos (nomes, códigos, siglas) estão escapando.
4. No prompt final, o contexto recuperado entra **delimitado como DADO** (ex.: entre tags), separado da instrução — instrução embutida em documento não é comando.
5. Toda resposta cita a(s) fonte(s) usada(s), de forma conferível.
6. **Se a resposta não está nos documentos recuperados, o sistema diz que não sabe.** Proibido completar com conhecimento geral do modelo.

## Avaliação antes de otimizar

- Monte um conjunto de [nº] perguntas com resposta-gabarito e documento-fonte esperado ANTES de ajustar qualquer parâmetro.
- Meça: o chunk certo foi recuperado? A resposta bate com o gabarito? A fonte citada é a correta?
- Só depois mexa em chunking, top-k ou híbrido — e compare os números antes/depois de cada mudança.

## Critérios de aceitação

- [ ] Pipeline de indexação roda de ponta a ponta com os documentos reais.
- [ ] Perguntas do gabarito respondidas com fonte citada correta.
- [ ] Pergunta cuja resposta NÃO está nos documentos recebe "não sei" (testar com pelo menos 3 casos).
- [ ] Instrução maliciosa embutida num documento indexado não altera o comportamento da resposta.

## Formato do relatório final

Estratégia de chunking adotada (e por quê), números da avaliação no gabarito, e o que ficou de fora com sugestão de próximo passo.
