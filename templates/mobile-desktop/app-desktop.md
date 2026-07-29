# App desktop (Tauri/Electron)

> Modo: `new_feature` · Área: mobile/desktop
> Uso: criar um app desktop multiplataforma com ponte segura, auto-update e empacotamento por SO.

---

Crie o app desktop do projeto **[nome do projeto]**.

## Objetivo

<<SUA FALA>>

## Contexto a preencher

- Framework: [Tauri / Electron] — justifique: Tauri gera binário menor e usa menos memória (webview do SO); Electron tem ecossistema maior e Chromium consistente entre SOs. Diga qual critério pesa mais NESTE app.
- SOs-alvo: [Windows / macOS / Linux].
- Assinatura de código: [certificados já existem / precisam ser obtidos] — sem ela, o Windows mostra alerta SmartScreen e o macOS bloqueia por padrão (Gatekeeper/notarização).

## Requisitos técnicos

1. **Janelas e tray**: comportamento de fechar definido ([encerra / minimiza para o tray]), posição e tamanho da janela restaurados entre sessões, menu do tray com as ações essenciais.
2. **Ponte frontend↔backend segura**: valide TODO payload que cruza a ponte (tipo, faixa, caminho de arquivo permitido) no lado nativo — o frontend é território hostil. Exponha só os comandos necessários (menor privilégio); nada de liberar acesso irrestrito a filesystem/shell por conveniência.
3. **Auto-update assinado**: atualização baixada só de [URL do servidor de update], com assinatura verificada ANTES de aplicar — update sem verificação é vetor clássico de ataque. Falha de update loga e avisa, não trava o app.
4. **Empacotamento por SO**: instalador nativo de cada SO-alvo ([MSI/NSIS, DMG, AppImage/deb — conforme o alvo]), com script de build reprodutível e documentado.
5. **Atalhos e comportamentos nativos**: atalhos padrão de cada plataforma (Cmd no macOS, Ctrl nos demais), menu de aplicativo no macOS, convenções de cada SO respeitadas — não portar hábitos de um SO para o outro.
6. Erros do processo nativo logados em arquivo local com rotação — nunca engolidos.

## Validações

- Build e execução verificadas em cada SO-alvo (VM serve).
- Teste da ponte: enviar payload malformado do frontend e confirmar rejeição com erro claro, sem crash.
- Teste do update: instalar a versão N, publicar N+1 num canal de teste e confirmar o fluxo completo, incluindo a recusa de pacote sem assinatura válida.

## Critérios de aceitação

- [ ] App roda e empacota em todos os SOs-alvo com comandos documentados.
- [ ] Toda entrada da ponte validada no lado nativo; permissões no mínimo necessário.
- [ ] Auto-update funciona e recusa pacote não assinado (teste prova).
- [ ] Comportamento de janela e tray implementado conforme definido.
- [ ] Binários assinados, ou pendência de certificado registrada explicitamente.

## Formato do relatório final

Framework escolhido e por quê, mapa dos comandos expostos na ponte com suas validações, como buildar e assinar por SO, resultado dos testes de update e o que ficou pendente.
