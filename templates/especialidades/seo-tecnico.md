# SEO técnico

> Modo: `technical` · Área: especialidades
> Uso: tornar o site/app indexável e rápido, com verificação real antes de declarar pronto.

---

Implemente as melhorias de SEO técnico no projeto **[nome do projeto]** ([URL do site]).

## Escopo

<<SUA FALA>>

## Requisitos técnicos

1. **Conteúdo indexável renderizado no servidor** (SSR/SSG) nas páginas que importam para busca ([páginas]): confirme com `curl`/view-source que o conteúdo está no HTML inicial — o que só existe depois do JavaScript rodar é indexação incerta.
2. **Title e meta description únicos por página**, com **canonical** correto — página duplicada sem canonical divide o ranqueamento entre as cópias.
3. **Dados estruturados (schema.org)** nos tipos que se aplicam ([Article/Product/FAQ/...]), em JSON-LD, validados na ferramenta de rich results — dado estruturado inválido é ignorado em silêncio pelo buscador.
4. **sitemap.xml** gerado com as URLs canônicas atuais e **robots.txt** que não bloqueia o que deve ser indexado (erro clássico: robots de staging vazando para produção).
5. **Core Web Vitals com meta numérica**: medir [LCP/CLS/INP] antes, definir a meta ([valores alvo]) e medir depois — otimização sem medição antes/depois não conta.
6. **Migração/renomeação de URL** [se aplicável]: redirect 301 individual de cada URL antiga para a equivalente nova — perder os redirects é jogar fora o histórico de ranqueamento; nada de redirecionar tudo para a home.
7. **Verificação real antes de declarar pronto**: inspecionar as páginas-chave no Search Console (ou equivalente) e no teste de rich results — "deve estar indexável" não é verificação.

## Critérios de aceitação

- [ ] Conteúdo das páginas-chave presente no HTML inicial, sem JavaScript (verificado com curl/view-source)
- [ ] Title, meta description e canonical corretos por página (amostra verificada)
- [ ] Dados estruturados passam no teste de rich results sem erro
- [ ] sitemap.xml acessível e coerente com as URLs canônicas; robots.txt revisado
- [ ] Core Web Vitals medidos antes e depois, com metas atingidas ou desvio justificado
- [ ] Redirects 301 testados URL a URL [se migração]
- [ ] Páginas-chave inspecionadas no Search Console, com resultado documentado

## Formato do relatório final

O que foi verificado por página (render, metas, canonical), resultado da validação de dados estruturados, números de Web Vitals antes/depois, e o estado das páginas no Search Console.
