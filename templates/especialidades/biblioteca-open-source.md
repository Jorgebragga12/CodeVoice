# Biblioteca open source

> Modo: `new_feature` · Área: especialidades
> Uso: preparar e publicar uma lib/pacote com API estável, docs e CI dignos de uso por terceiros.

---

Prepare a publicação da biblioteca **[nome do pacote]** no **[npm/PyPI/crates.io/outro]**, a partir do projeto **[nome do projeto]**.

## Escopo

<<SUA FALA>>

## Requisitos técnicos

1. **API pública mínima e deliberada**: tudo que for exportado vira contrato que você terá de manter — exporte só o que o usuário precisa; o resto fica interno.
2. **Semver a sério desde o 0.x**: breaking change sobe major (ou minor no 0.x, documentado); nunca quebrar em patch. Cada release tem entrada no CHANGELOG dizendo o que mudou e como migrar.
3. **README que funciona em 30 segundos**: instalação + um exemplo mínimo copiável logo no topo — e o exemplo roda de verdade (teste isso, não presuma).
4. **Tipos e docs gerados** ([d.ts/docstrings/rustdoc]): toda assinatura pública documentada com parâmetros e retorno.
5. **CI testando nas versões suportadas** de [linguagem/runtime] declaradas — suporte não testado é suporte de mentira.
6. **LICENSE explícita** ([licença]) no repositório e nos metadados do pacote — sem licença, ninguém pode usar legalmente.
7. **Tamanho do pacote controlado**: declare o que entra no artefato ([files/include/exports]) e inspecione o conteúdo empacotado antes de publicar — teste, config e fixture não vão junto.
8. **O teste final**: instale o pacote (empacotado local ou publicado) num projeto limpo, fora do repositório, e rode o exemplo do README. Se não funciona ali, não está pronto.

## Critérios de aceitação

- [ ] Superfície pública revisada: só o necessário exportado
- [ ] README com exemplo mínimo que roda (verificado)
- [ ] CI verde em todas as versões suportadas declaradas
- [ ] LICENSE e CHANGELOG presentes; versão e metadados do pacote corretos
- [ ] Conteúdo do pacote inspecionado — sem arquivo que não devia estar lá
- [ ] Pacote instalado em projeto limpo e usado com sucesso (o teste final)

## Formato do relatório final

Superfície pública final (lista de exports), versão escolhida e por quê, conteúdo do artefato publicado, resultado do teste de instalação em projeto limpo, e o que ficou de fora de propósito.
