# Pipeline CI/CD

> Modo: `technical` · Área: DevOps
> Uso: automatizar build, teste e deploy no [GitHub Actions/GitLab CI/outro].

---

Configure CI/CD para o projeto **[nome do projeto]** em **[plataforma]**.

## Objetivo

<<SUA FALA>>

## Etapas do pipeline

1. **CI (todo push/PR)**: instalar dependências (com cache), lint, typecheck, testes, build.
2. **CD ([só na main / por tag])**: deploy para [ambiente], apenas se o CI passou.

## Requisitos técnicos

- **Segredos via secrets da plataforma** — nunca no YAML, nunca em log.
- Cache de dependências para o pipeline ser rápido; falha rápida (lint antes de teste, teste antes de build).
- Versões de runtime pinadas ([ex.: node 20.x]) — não usar `latest`.
- Job de deploy com ambiente/aprovação conforme a plataforma; sem deploy automático de branch de feature.
- O pipeline deve falhar visivelmente se qualquer etapa falhar — nada de `|| true` escondendo erro.

## Validações

- Rode o pipeline de verdade (push numa branch) e cole o link/resultado da execução verde.
- Force uma falha (teste quebrado proposital) e confirme que o pipeline fica vermelho, depois reverta.

## Critérios de aceitação

- [ ] CI verde na branch atual, executando lint + testes + build.
- [ ] Deploy só ocorre nas condições definidas.
- [ ] Nenhum segredo exposto em logs de execução.

## Formato do relatório final

Arquivos criados, o que cada job faz, segredos que preciso cadastrar na plataforma (nome e propósito), e tempo de execução do pipeline.
