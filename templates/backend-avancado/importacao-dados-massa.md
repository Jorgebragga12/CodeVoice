# Importação de Dados em Massa (CSV/Planilha)

> Modo: `new_feature` · Área: backend
> Uso: importar planilhas/CSV enviados pelo usuário com validação linha a linha e sem duplicar dados ao reimportar.

---

Implemente importação de dados em massa no projeto **[nome do projeto]**.

## Escopo

<<SUA FALA>>

## Contrato do arquivo (preencha)

- Formato aceito: [CSV / .xlsx], tamanho máximo [N MB / N linhas]
- Colunas esperadas e tipo de cada uma: [coluna → tipo → obrigatória?]
- Chave natural para deduplicação/upsert: [e-mail, CPF, código externo...]
- Política para linha inválida: [importa as válidas e reporta as inválidas | rejeita o arquivo inteiro] — decida explicitamente; o meio-termo silencioso é o pior dos mundos
- Linha duplicada (chave já existe): [atualiza (upsert) | ignora | marca como erro]

## Requisitos técnicos

1. **Parsing tolerante**: aceite [UTF-8 e Latin-1] (detectando encoding), separadores [`,` e `;`], e mapeie colunas **pelo nome do cabeçalho, não pela posição** — planilha de usuário vem com colunas reordenadas, e mapear por posição corrompe dados sem erro nenhum.
2. **Validação linha a linha antes de gravar**: cada erro acumula com [número da linha, coluna, valor recebido, motivo] — parar no primeiro erro obriga o usuário a corrigir às cegas, um erro por tentativa.
3. **Relatório de erros para o usuário**: resumo ([N importadas, N com erro]) + arquivo baixável com as linhas rejeitadas e o motivo em linguagem de gente, para corrigir e reenviar só o que falhou.
4. **Idempotência ao reimportar**: rodar o mesmo arquivo duas vezes ([clique duplo, retry, reenvio após correção]) não duplica registros — o upsert pela chave natural garante isso; sem chave natural definida, a importação não deve nem começar.
5. **Preview antes de gravar** ([primeiras N linhas] interpretadas + contagem de válidas/inválidas), com confirmação explícita do usuário — importação é escrita em massa, e escrita em massa errada é cara de desfazer.
6. **Processamento em lote via job em background** para arquivo grande: chunks de [N linhas], transação por chunk, progresso consultável ([X de Y linhas]) — request HTTP segurando um arquivo de milhares de linhas é timeout garantido.
7. Registre cada importação ([quem, quando, arquivo, resultado]) — quando aparecer dado estranho na base, a pergunta será "de qual importação veio isso".
8. Nenhuma linha descartada em silêncio: toda linha do arquivo termina como importada, atualizada, ignorada por duplicidade ou rejeitada com motivo — e os números têm que fechar com o total do arquivo.

## Critérios de aceitação

- [ ] Arquivo com [encoding Latin-1, separador `;`, colunas fora de ordem] importa corretamente (teste com fixture real)
- [ ] Arquivo com linhas inválidas segue a política definida e gera relatório com linha + motivo
- [ ] Reimportar o mesmo arquivo não cria duplicatas (teste rodando a importação duas vezes)
- [ ] Arquivo grande ([volume de teste]) processa em background com progresso, sem estourar memória nem timeout
- [ ] Soma de importadas + atualizadas + ignoradas + rejeitadas = total de linhas do arquivo (verificado no teste)
- [ ] Preview mostra a interpretação dos dados antes de qualquer escrita no banco

## Formato do relatório final

Contrato final do arquivo (colunas, chave, política de erro), fluxo implementado (preview → job → relatório), como o usuário obtém o relatório de erros, e a saída dos testes de idempotência e de arquivo grande.
