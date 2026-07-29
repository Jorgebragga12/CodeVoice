# Exportação e Relatórios (PDF/Excel/CSV)

> Modo: `new_feature` · Área: backend
> Uso: gerar arquivos para download (PDF, Excel, CSV) a partir dos dados do sistema, sem estourar memória.

---

Implemente exportação de relatórios no projeto **[nome do projeto]**.

## Escopo

<<SUA FALA>>

## Definição do relatório (preencha)

- Formato(s): [PDF / Excel (.xlsx) / CSV]
- Conteúdo e layout: [colunas na ordem esperada / estrutura do PDF — cabeçalho, itens, totais]
- Filtros disponíveis: [período, status, cliente...] — a exportação usa **os mesmos filtros aplicados na tela**
- Volume esperado: [até N linhas típico / máximo estimado] — isso decide síncrono vs background

## Requisitos técnicos

1. **Mesma fonte de dados da tela**: exporte reusando o [service/query] que alimenta a listagem, com os filtros recebidos — relatório que diverge da tela destrói a confiança no sistema. Não duplique a regra.
2. **Permissões valem no export**: o usuário só exporta o que pode ver na UI; a rota de exportação passa pelas mesmas checagens de autorização e escopo ([organização/dono]).
3. **Streaming para CSV/Excel**: escreva no response conforme percorre os dados com [paginação por cursor], sem carregar tudo em memória — é o que impede o export de derrubar o processo com volume real.
4. **CSV que abre certo no Excel**: UTF-8 **com BOM** (sem BOM, acento vira lixo no Excel), separador [`;` para pt-BR, onde o Excel espera ponto e vírgula], datas e números no formato [pt-BR/ISO — definir].
5. **Proteção contra CSV injection**: célula começando com `=`, `+`, `-` ou `@` é prefixada com `'` — sem isso, um dado malicioso vira fórmula executável na máquina de quem abre.
6. **Limite síncrono**: até [N linhas], gera na hora; acima disso, **job em background** que notifica ([e-mail/notificação in-app]) com link de download expirável ([período]) — request HTTP não pode ficar minutos aberta.
7. **PDF**: geração via [biblioteca/ferramenta] com template versionado no repositório; valores monetários e totais vêm calculados do backend, nunca recalculados no template.
8. Falha na geração **nunca é silenciosa**: job com erro fica registrado com causa e o usuário é avisado que o arquivo não saiu.
9. Nome do arquivo carrega contexto: [`relatorio-vendas-2025-01.csv`] — dezenas de `export.csv` na pasta de downloads é atrito desnecessário.

## Critérios de aceitação

- [ ] Arquivo exportado bate com a tela: mesmos filtros, mesmos totais (teste comparando os dois caminhos)
- [ ] CSV abre no Excel com acentos e colunas corretos; célula com `=SOMA(...)` chega neutralizada
- [ ] Export de [volume grande de teste] completa sem estourar memória (medir consumo durante)
- [ ] Acima do limite síncrono, vira job em background com notificação e link expirável
- [ ] Usuário sem permissão sobre os dados recebe negado na rota de export (teste)
- [ ] Falha de geração registrada em log e comunicada ao usuário

## Formato do relatório final

Formatos implementados, rota e limites síncrono/background, como o streaming pagina os dados, medição de memória no volume de teste, e amostra de arquivo gerado conferida contra a tela.
