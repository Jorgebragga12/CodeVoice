# Componente reutilizável

> Modo: `ui_creation` · Área: frontend/UI
> Uso: criar um componente isolado, testável e documentado.

---

Crie o componente **[nome]** no projeto **[nome do projeto]**.

## Objetivo

<<SUA FALA>>

## Contrato do componente

- Props/entradas: [liste com tipos — ou derive do objetivo e me mostre antes]
- Eventos/callbacks que emite: [liste]
- O componente **não** conhece de onde vêm os dados: recebe tudo por props (sem fetch interno, sem estado global escondido).

## Requisitos técnicos

- Antes de criar: verifique se já existe componente parecido no projeto para estender em vez de duplicar.
- Estados visuais: normal, hover/focus, desabilitado, [carregando], [erro de validação].
- Acessível: papel semântico correto (button é `<button>`, não `<div onClick>`), foco visível, ARIA quando necessário.
- Estilo via [sistema do projeto: tokens/tailwind/CSS modules] — nada de valores mágicos soltos.

## Validações

- Testes: renderização com props mínimas, cada variação relevante, callbacks disparando, estado desabilitado não dispara ação.

## Critérios de aceitação

- [ ] Usável em pelo menos 2 contextos diferentes sem alteração.
- [ ] Testes verdes.

## Formato do relatório final

API final do componente (props/eventos), exemplo de uso, e onde ele já foi aplicado.
