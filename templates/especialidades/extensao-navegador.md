# Extensão de navegador

> Modo: `new_feature` · Área: especialidades
> Uso: criar extensão Chrome/Firefox (Manifest V3) pronta para passar na revisão da loja.

---

Implemente a seguinte extensão de navegador no projeto **[nome do projeto]**, com Manifest V3.

## Escopo

<<SUA FALA>>

## Arquitetura (o que vive onde)

- **Content script**: só o que precisa tocar o DOM da página; roda isolado e não tem acesso direto às APIs privilegiadas.
- **Service worker** (background): lógica central, chamadas de rede, coordenação — lembrando que ele é encerrado quando ocioso, então nada de estado em variável global; persista em `chrome.storage`.
- **Popup/options**: UI de configuração; morre ao fechar, também não guarda estado.
- **Comunicação por mensagens tipadas** (`chrome.runtime.sendMessage`/`onMessage`), com um tipo/ação por mensagem e tratamento para mensagem desconhecida — nunca acesso direto entre contextos.

## Regras inegociáveis

1. **Permissões MÍNIMAS no manifest**: cada permissão a mais assusta o usuário na instalação e atrasa a revisão da loja. Prefira `activeTab` a `<all_urls>`; host permissions só para [domínios realmente necessários]. Justifique cada uma no relatório.
2. **Zero código remoto**: nenhum script carregado de URL externa — proibido pela Chrome Web Store no Manifest V3; todo código vai empacotado na extensão.
3. **Armazenamento via `chrome.storage`** ([sync/local] conforme o dado) — não localStorage, que não funciona de forma confiável no service worker.
4. Dado do usuário não sai da extensão sem necessidade real, declarada na política de privacidade.

## Checklist de publicação (Chrome Web Store)

- Manifest com nome, descrição, versão e ícones nos tamanhos exigidos
- Justificativa de cada permissão preenchida no painel da loja
- Política de privacidade se coleta qualquer dado
- Screenshots reais e descrição honesta do que a extensão faz
- Testar o pacote zipado final instalado do zero, não só a pasta de trabalho

## Critérios de aceitação

- [ ] Manifest V3 com o conjunto mínimo de permissões, cada uma justificada
- [ ] Nenhum script carregado de origem remota (verificado no código e no manifest)
- [ ] Estado sobrevive ao service worker ser encerrado e religado (verificação documentada)
- [ ] Mensagens entre contextos tipadas, com tratamento para mensagem desconhecida
- [ ] Extensão funciona instalada a partir do zip final

## Formato do relatório final

Lista de permissões com justificativa, mapa do que roda em cada contexto, como o estado é persistido, e o que falta para submeter à loja.
