# Internacionalização (i18n)

> Modo: `technical` · Área: especialidades
> Uso: preparar o app para múltiplos idiomas sem string esquecida nem frase quebrada.

---

Prepare o projeto **[nome do projeto]** para múltiplos idiomas ([idiomas alvo]).

## Escopo

<<SUA FALA>>

## Requisitos técnicos

1. **TODAS as strings visíveis vão para o catálogo** — inclusive as escondidas: mensagens de validação, erros de API, e-mails, títulos de página, textos de acessibilidade (aria-label, alt). Varra o código atrás de string literal exibida ao usuário; a lista do que foi encontrado vai no relatório.
2. **Plural e interpolação pelo sistema de i18n** ([biblioteca/formato]), nunca concatenando pedaços de frase — ordem de palavras e regras de plural mudam por idioma, e frase montada em código quebra em qualquer idioma que não o original.
3. **Datas, números e moedas formatados pelo locale** (Intl ou equivalente), nunca formatação manual com ponto/vírgula fixos.
4. **Layout que aguenta o texto crescer ~30%** (alemão e português são mais longos que inglês): nada de largura fixa apertada nem truncamento silencioso em botão e menu. **RTL** [se aplicável]: layout espelhado testado com [idioma RTL].
5. **Detecção e troca de idioma**: idioma inicial vem de [preferência salva/navegador], é trocável na UI e a escolha persiste entre sessões.
6. **Processo contra regressão**: chave faltante em qualquer idioma quebra o build ou acusa em CI — string nova que escapa do catálogo hoje é bug de tradução daqui a seis meses. Fallback para [idioma padrão] é logado, nunca silencioso.

## Critérios de aceitação

- [ ] Nenhuma string literal visível ao usuário fora do catálogo (varredura documentada)
- [ ] Plurais corretos em [idiomas alvo], incluindo casos zero/um/muitos (teste)
- [ ] Datas/números/moedas mudam com o locale (teste)
- [ ] UI íntegra com pseudo-tradução ~30% mais longa (verificação visual documentada)
- [ ] Troca de idioma na UI aplica na hora e persiste (teste)
- [ ] CI acusa chave de tradução faltante (teste removendo uma chave de propósito)

## Formato do relatório final

Quantas strings foram extraídas e de onde, sistema de i18n adotado, como plural/interpolação funcionam, o que a verificação de CI cobre, e pendências de tradução por idioma.
