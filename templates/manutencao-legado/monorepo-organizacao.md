# Organizar monorepo

> Modo: `technical` · Área: manutenção
> Uso: estruturar (ou arrumar) monorepo/workspaces com fronteiras claras e CI seletivo.

---

Organize o monorepo do projeto **[nome do projeto]** usando **[pnpm workspaces / turborepo / nx / cargo workspace]**.

## Escopo

<<SUA FALA>>

## Contexto

- Pacotes/apps atuais ou desejados: [lista, ou "derivar da estrutura atual"]
- Publica pacotes em registry? [sim, quais / não]

## Requisitos técnicos

1. **Fronteiras pelo domínio, não pelo tipo de arquivo**: cada pacote tem uma responsabilidade nomeável em uma frase; se dois pacotes só mudam juntos, provavelmente são um só.
2. **Dependências internas explícitas e sem ciclos**, com verificação automática no CI ([ferramenta da stack, ex.: regra de lint de fronteiras]) — ciclo detectado quebra o build, não vira warning.
3. **Monorepo não é desculpa para acoplamento**: importar de outro pacote passa pela API pública dele (entrypoint exportado); import profundo de caminho interno é bloqueado por lint.
4. Build e teste seletivos por afetados no CI — mudança em um pacote não roda a pipeline inteira. Meça o tempo de CI antes e depois para provar o ganho.
5. Tooling compartilhado (lint, formatter, tsconfig/config base) definido uma vez e estendido pelos pacotes — zero config duplicada.
6. [Se publica]: versionamento e publicação automatizados por [changesets ou equivalente da stack], com changelog por pacote.
7. Migração em passos com o repositório funcionando em cada um; mova arquivos preservando o histórico do git (`git mv`, não recriar).

## Critérios de aceitação

- [ ] Grafo de dependências internas sem ciclos, verificado no CI.
- [ ] Import profundo entre pacotes bloqueado por lint (comprovado com um import proibido que falha).
- [ ] CI roda só o afetado, comprovado com uma mudança de teste em um único pacote.
- [ ] Configs compartilhadas sem duplicação entre pacotes.
- [ ] Build e testes verdes em todos os passos da migração.

## Formato do relatório final

Estrutura final de pacotes (árvore + responsabilidade de cada), grafo de dependências internas, tempo de CI antes/depois, e as regras de fronteira ativadas.
