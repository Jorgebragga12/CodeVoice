# PWA

> Modo: `technical` · Área: mobile/desktop
> Uso: transformar um web app existente em PWA instalável, com cache deliberado e offline honesto.

---

Transforme o web app do projeto **[nome do projeto]** em PWA.

## Escopo

<<SUA FALA>>

## Contexto a preencher

- Stack do web app: [framework/bundler].
- O que DEVE funcionar offline: [lista] — o resto avisa que precisa de rede.
- Já existe service worker? [sim/não — se sim, qual e o que faz].

## Requisitos técnicos

1. **Manifest completo**: name, short_name, ícones nos tamanhos exigidos (incluindo maskable), theme_color, background_color, display [standalone/outro] e start_url com escopo correto.
2. **Estratégia de cache POR TIPO de recurso** — nomeie e implemente cada uma:
   - App shell (HTML/CSS/JS versionados): cache-first com precache no install — muda junto com o deploy.
   - Chamadas de API: network-first com fallback ao cache — dado velho identificado como velho é melhor que erro.
   - Imagens/assets estáticos: stale-while-revalidate com limite de entradas — cache de imagem sem teto come o storage do usuário.
3. **Atualização do SW sem quebrar sessão aberta**: o novo SW em waiting não toma controle no meio do uso; mostre "nova versão disponível" e aplique (skipWaiting + reload) só com consentimento — trocar assets sob uma sessão aberta causa erro de chunk e estado inconsistente.
4. **Offline honesto**: o que a lista do escopo diz que funciona offline funciona de verdade; o que não funciona avisa claramente no momento da ação — nunca spinner infinito nem falha silenciosa. Página offline própria para navegação sem cache.
5. Nenhuma resposta de erro (4xx/5xx) entra no cache como se fosse válida.

## Validações

- Auditoria de instalabilidade (ex.: Lighthouse) passando; corrija o que ela apontar.
- Teste offline: carregar o app, derrubar a rede e navegar — comparar o comportamento com a lista do escopo.
- Teste de atualização: publicar uma mudança, manter uma aba aberta e confirmar que a sessão não quebra e o aviso aparece.
- Instalação testada em Android e iOS, com as diferenças documentadas (no iOS a instalação é via Compartilhar → Tela de Início e há limites próprios de storage e push).

## Critérios de aceitação

- [ ] App instalável em Android e iOS, com as diferenças entre plataformas documentadas.
- [ ] Cada tipo de recurso tem estratégia de cache nomeada e implementada.
- [ ] Atualização do SW não quebra sessão aberta (teste prova).
- [ ] Modo offline se comporta exatamente como a lista do escopo define.
- [ ] Nenhum erro HTTP cacheado como resposta válida.

## Formato do relatório final

Estratégia de cache por tipo (tabela), fluxo de atualização do SW, resultado dos testes offline e de instalação por plataforma, e limitações conhecidas.
