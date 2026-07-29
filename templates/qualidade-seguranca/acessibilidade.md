# Acessibilidade (a11y)

> Modo: `code_review` · Área: qualidade
> Uso: auditar e corrigir acessibilidade de telas existentes.

---

Audite e corrija a acessibilidade de **[telas/componentes]** no projeto **[nome do projeto]**.

## Escopo

<<SUA FALA>>

## Checklist (WCAG básico)

1. **Teclado**: tudo alcançável e acionável por Tab/Enter/Esc; foco sempre visível; sem armadilha de foco; modais devolvem o foco ao fechar.
2. **Semântica**: botão é `<button>`, link é `<a>`, headings em hierarquia (um h1 por página); landmarks (`main`, `nav`).
3. **Formulários**: todo input com `<label>` associado; erros ligados ao campo via `aria-describedby`; obrigatoriedade indicada não só por cor.
4. **Imagens/ícones**: `alt` descritivo em imagens informativas, `alt=""` em decorativas; botões só-ícone com `aria-label`.
5. **Contraste**: texto normal ≥ 4.5:1, texto grande ≥ 3:1.
6. **Dinâmico**: mudanças importantes anunciadas (`aria-live` para toasts/erros); estados de carregamento perceptíveis sem visão.

## Regras

- Primeiro o relatório de problemas (tela, elemento, critério violado, correção); depois aplique as correções.
- Não mude o visual perceptível além do necessário (ex.: ajustar um tom de cor para contraste é ok).
- Rode [axe/lighthouse] antes e depois e reporte os números.

## Critérios de aceitação

- [ ] Fluxo principal completável só com teclado.
- [ ] Zero erros críticos no axe/lighthouse nas telas do escopo.

## Formato do relatório final

Problemas encontrados vs. corrigidos, score antes/depois, e o que ficou pendente com justificativa.
