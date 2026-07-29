# Tela nova

> Modo: `ui_creation` · Área: frontend/UI
> Uso: construir uma tela ou página nova.

---

Crie a tela **[nome da tela]** no projeto **[nome do projeto]**.

## Objetivo

<<SUA FALA>>

## Estados obrigatórios

Toda tela precisa tratar os 4 estados — não só o caso feliz:

1. **Carregando** — indicador visível, layout não "pula" quando os dados chegam.
2. **Vazio** — mensagem útil + ação sugerida (não uma tela em branco).
3. **Erro** — o que deu errado + botão de tentar de novo.
4. **Com dados** — o caso principal.

## Requisitos técnicos

- Use os componentes e o padrão visual já existentes no projeto; não invente um estilo novo.
- Responsivo: funcional em [mobile 375px / desktop] no mínimo.
- Acessibilidade básica: navegável por teclado, labels em inputs, contraste adequado, `alt` em imagens.
- Nenhuma chamada de dados dentro de loop de render.

## Restrições

- Sem biblioteca de UI nova sem autorização.
- Textos em [idioma do produto].

## Critérios de aceitação

- [ ] Os 4 estados implementados e verificáveis.
- [ ] Navegação até a tela funcionando (rota/menu).
- [ ] Lint/typecheck e testes verdes.

## Formato do relatório final

Componentes criados, rota adicionada, como forçar cada um dos 4 estados para eu conferir visualmente.
