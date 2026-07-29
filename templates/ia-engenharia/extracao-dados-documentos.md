# Extração de Dados de Documentos (OCR + LLM)

> Modo: `new_feature` · Área: engenharia de IA
> Uso: extrair dados estruturados de documentos não estruturados (notas, contratos, currículos, PDFs).

---

Implemente extração de dados de documentos no projeto **[nome do projeto]**.

## Objetivo

<<SUA FALA>>

## Contexto a preencher

- Tipos de documento: [nota fiscal / contrato / currículo / outro — e se são PDFs digitais ou escaneados]
- Campos alvo: [lista dos campos a extrair, com tipo esperado de cada um]
- Volume esperado: [documentos por dia/mês — define quanto o custo por documento importa]
- Modelo/provedor: [modelo LLM] · OCR: [ferramenta, se necessário]

## Requisitos técnicos

1. **Tente extração de texto nativa antes de OCR.** PDF digital tem texto embutido; OCR nele só adiciona custo e erro. OCR entra apenas para documento escaneado ou foto.
2. **Extração com schema tipado (JSON estruturado), nunca texto livre parseado depois.** Use o recurso de saída estruturada do provedor; defina tipo, formato e descrição de cada campo.
3. **Todo campo é anulável.** Se o dado não está no documento, o valor é `null` — proibido o modelo inventar para preencher. Campo errado com cara de certo é pior que campo vazio.
4. Valide os campos extraídos por código, fora do LLM: formato (datas, CNPJ/CPF, valores monetários) e consistência interna (ex.: soma dos itens bate com o total).
5. **Baixa confiança vai para revisão humana, não para o banco.** Defina o gatilho ([campo obrigatório nulo / validação falhou / documento ilegível]) e uma fila de revisão com o documento original ao lado dos campos extraídos.
6. Guarde o documento original e o JSON extraído juntos, com versão do prompt/modelo — sem isso não existe auditoria nem debug de regressão.
7. Texto vindo do documento é DADO, delimitado no prompt — instrução embutida num PDF malicioso não vira comando.
8. Falha de OCR, parsing ou chamada ao modelo é registrada com o documento identificado; nunca engolida silenciosamente.

## Custo e avaliação

- Monte um gabarito com [nº] documentos reais rotulados ANTES de ajustar prompt ou modelo; meça acurácia por campo, não só por documento.
- Meça tokens e custo por documento no gabarito e projete para o volume esperado — se estourar o orçamento, teste modelo menor ou corte de páginas irrelevantes antes de escalar.
- Toda mudança de prompt/modelo roda no gabarito de novo; compare os números antes/depois.

## Critérios de aceitação

- [ ] Pipeline roda de ponta a ponta com documentos reais dos tipos listados.
- [ ] Acurácia por campo no gabarito medida e reportada.
- [ ] Documento sem um campo obrigatório resulta em `null` + fila de revisão, nunca em valor inventado (testar com pelo menos 3 casos).
- [ ] Validações de formato e consistência rejeitam extração inválida antes de persistir.
- [ ] Custo por documento medido e projetado para o volume esperado.

## Formato do relatório final

Estratégia de extração por tipo de documento (nativa vs OCR), acurácia por campo no gabarito, custo por documento projetado no volume, e quais campos mais caem em revisão humana com hipótese do porquê.
