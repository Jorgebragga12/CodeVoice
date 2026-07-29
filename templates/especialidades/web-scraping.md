# Web scraping

> Modo: `technical` · Área: especialidades
> Uso: coletar dados de sites de forma legal, educada e resiliente a mudança de layout.

---

Implemente a seguinte coleta de dados no projeto **[nome do projeto]**.

## Objetivo da coleta

<<SUA FALA>>

## Antes de escrever qualquer código

1. **Verifique se existe API oficial** para esses dados — se existir, use a API e pare aqui; scraping é o último recurso, não o primeiro.
2. **Leia os ToS e o robots.txt de [site alvo]** e respeite o que dizem; registre no relatório o que foi verificado. Se o site proíbe a coleta, reporte e pare — não contorne.

## Requisitos técnicos

1. **Rate limiting educado**: no máximo [N] requisições por segundo, com pausa entre páginas e User-Agent honesto identificando o projeto ([nome/contato]) — não finja ser um navegador comum.
2. **Parsing que falha visivelmente**: se o seletor não encontra o elemento esperado, a coleta ABORTA com erro claro — nunca colete lixo silenciosamente achando que está tudo bem.
3. **Validação contra schema**: todo registro extraído é validado ([campos obrigatórios, tipos, formatos]) antes de salvar; registro inválido vai para uma fila de rejeitados com o motivo, não para o dataset.
4. **Retry só para falha de rede**: timeout e 5xx tentam de novo com backoff ([N] tentativas); mudança de estrutura da página (seletor sumiu, campo vazio onde nunca era) aborta o job inteiro — são problemas diferentes e a resposta é diferente.
5. **Armazenamento com proveniência**: cada registro salvo carrega a URL de origem e o timestamp da coleta — sem isso o dado é inauditável.
6. **Cache local durante o desenvolvimento**: páginas já baixadas são reaproveitadas nos ajustes de parser, para não martelar o site a cada iteração.

## Critérios de aceitação

- [ ] Verificação de API oficial/ToS/robots.txt documentada no relatório
- [ ] Rate limit e User-Agent identificado configurados
- [ ] Mudança simulada de layout (HTML alterado) → coleta aborta com erro claro (teste)
- [ ] Registro que viola o schema é rejeitado com motivo, não salvo (teste)
- [ ] Falha de rede simulada → retry com backoff; após esgotar, erro reportado (teste)
- [ ] Registros salvos têm URL de origem e timestamp da coleta

## Formato do relatório final

O que foi verificado sobre API/ToS/robots.txt, taxa de coleta configurada, schema de validação usado, e como o parser se comporta quando o site muda.
