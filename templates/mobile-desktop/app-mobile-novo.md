# App mobile novo

> Modo: `new_feature` · Área: mobile/desktop
> Uso: iniciar um app mobile do zero com estrutura, navegação e build nas duas plataformas.

---

Crie a estrutura inicial do app mobile do projeto **[nome do projeto]**.

## Objetivo do app

<<SUA FALA>>

## Contexto a preencher

- Stack: [React Native / Flutter / nativo (Swift+Kotlin)] — justifique a escolha para ESTE caso (equipe, reuso de código web, necessidade de APIs nativas). Se a justificativa não se sustentar, aponte antes de começar.
- Plataformas-alvo: [iOS / Android / ambas] · versão mínima de SO: [versão].
- Telas principais previstas: [lista de telas].
- Backend/API existente: [URL ou "não existe ainda"].

## Requisitos técnicos

1. **Navegação e estrutura de telas**: defina a árvore de navegação (stack/tab/modal) antes de codar telas; cada tela em seu módulo, sem lógica de negócio no componente de UI.
2. **Gestão de estado**: escolha UMA solução [ex.: a padrão do ecossistema escolhido] e justifique; estado de servidor separado de estado de UI.
3. **Os 4 estados de toda tela**: carregando, vazio, erro e com dados. Tela que só trata o caminho feliz está incompleta.
4. **Offline básico**: falha de rede mostra mensagem acionável com opção de tentar de novo — nunca tela branca nem spinner infinito.
5. **Permissões no momento do uso**: peça câmera/localização/notificação só quando a funcionalidade for usada, com uma frase de contexto antes do diálogo do sistema — pedir tudo no boot derruba a taxa de aceite e é motivo comum de rejeição nas lojas.
6. **Erro nunca engolido**: toda falha logada e visível em dev; em produção, reportada para [ferramenta de crash reporting ou "definir"].

## Validações

- Build de debug rodando nas DUAS plataformas (simulador/emulador serve nesta fase); registre os comandos de build no README.
- Navegue por todas as telas criadas e force os 4 estados de pelo menos uma tela (ex.: desligando a rede).

## Critérios de aceitação

- [ ] Projeto compila e roda em iOS e Android com os comandos documentados.
- [ ] Árvore de navegação implementada conforme a lista de telas.
- [ ] Pelo menos uma tela demonstra os 4 estados (carregando/vazio/erro/dados).
- [ ] Nenhuma permissão pedida no primeiro boot.
- [ ] Estrutura de pastas e escolha de gestão de estado documentadas em meia página.

## Formato do relatório final

Stack escolhida e por quê, estrutura de pastas criada, telas implementadas, como rodar em cada plataforma e o que ficou de fora desta primeira entrega.
