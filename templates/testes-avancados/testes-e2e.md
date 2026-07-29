# Testes E2E

> Modo: `technical` · Área: testes
> Uso: criar ou reforçar a suíte E2E cobrindo apenas os fluxos críticos de negócio.

---

Implemente a suíte de testes E2E com **[Playwright/Cypress]** no projeto **[nome do projeto]**.

## Objetivo

<<SUA FALA>>

## Escopo — o que merece E2E

- Cubra SOMENTE os fluxos críticos de negócio: [ex.: login, checkout, fluxo principal]. E2E é caro e lento — tudo que puder ser provado por teste de unidade ou integração fica fora da suíte E2E.
- Liste os fluxos escolhidos antes de começar e justifique cada um em uma linha.

## Regras técnicas

1. **Seletores estáveis.** Use `data-testid` (ou role/label acessível); nunca classe CSS ou hierarquia de DOM — quebram a cada refactor de estilo.
2. **Dados isolados por teste.** Cada teste cria (e destrói) seus próprios dados via API/seed; nenhum teste depende de estado deixado por outro nem de dado "que já existe" no ambiente.
3. **Espera por condição, nunca sleep fixo.** Aguarde elemento visível, requisição concluída ou estado da UI; `sleep(3000)` é proibido — flake garantido em máquina lenta.
4. **Evidência na falha.** Screenshot + trace/vídeo anexados automaticamente quando o teste falha.
5. **CI.** A suíte roda no pipeline [CI do projeto] em modo headless; documente o comando local equivalente.

## Política anti-flake

- Teste instável é corrigido ou deletado. Nunca configure retry até passar nem marque "skip temporário" sem issue aberta — teste flaky treina o time a ignorar vermelho.

## Critérios de aceitação

- [ ] Só fluxos críticos têm E2E; a lista de fluxos está justificada.
- [ ] Zero seletores por CSS frágil; tudo via `data-testid`/role.
- [ ] Nenhum `sleep`/espera fixa no código de teste.
- [ ] Cada teste passa isolado e em qualquer ordem (rodar com ordem aleatória prova).
- [ ] Suíte verde no CI com screenshot/trace configurados para falha.

## Formato do relatório final

Fluxos cobertos e por quê, comando para rodar local e no CI, tempo total da suíte, e o que ficou de fora do E2E com a justificativa.
