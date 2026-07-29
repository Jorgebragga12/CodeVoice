# Setup de Projeto do Zero

> Modo: `planning` · Área: arquitetura
> Uso: criar um projeto novo do zero com stack, tooling e estrutura prontos para a primeira funcionalidade.

---

Crie do zero o projeto **[nome do projeto]**.

## O que o projeto vai ser

<<SUA FALA>>

## Contexto e restrições

- Tipo de projeto: [API, app web, CLI, biblioteca, monorepo...]
- Stack que a equipe já domina: [linguagens/frameworks — ou "aberto a sugestão"]
- Onde vai rodar: [VPS, serverless, container, desktop — ou "ainda não sei"]
- Restrições: [prazo, orçamento, licença, integrações obrigatórias]

## Regras do setup

1. **Justifique a stack contra as restrições acima**, não contra tendência: stack que a equipe domina vence a "melhor" tecnologia, porque velocidade de entrega vem de familiaridade. Se sugerir algo fora do que a equipe conhece, diga o custo de aprendizado.
2. **Cada dependência precisa se pagar.** Liste as escolhidas com uma linha de justificativa; na dúvida, fique sem.
3. Estrutura de pastas rasa e organizada por [domínio ou camada]: nada de pasta vazia "para o futuro" — estrutura se deriva da necessidade, não se inventa antes dela.
4. Tooling desde o dia zero, porque "adicionar depois" nunca acontece: linter + formatador com configs que não conflitam, tipagem no modo estrito, framework de testes com um teste real de exemplo passando.
5. Variáveis de ambiente: `.env.example` versionado documentando todas as chaves; `.env` no `.gitignore`; o app falha no boot com mensagem clara se faltar variável obrigatória — nunca assume default silencioso.
6. Scripts padronizados — um comando para subir em dev, um para testar, um para lint, um para build — todos documentados no README.
7. Git desde o início: repositório inicializado, `.gitignore` adequado à stack, lockfile versionado, primeiro commit com o esqueleto já funcional.
8. README inicial responde três coisas: o que é o projeto, como rodar numa máquina limpa (passo a passo real, não "instale as dependências"), como rodar os testes.
9. **O esqueleto termina onde a primeira funcionalidade começa.** Nada de CRUD de exemplo, tela demo ou abstração especulativa.
10. Se uma escolha depender de informação que eu não dei (equipe, orçamento, infra existente), pergunte antes de assumir.

## Critérios de aceitação

- [ ] Projeto sobe numa máquina limpa seguindo apenas o README.
- [ ] Comandos de dev, teste, lint e build rodam sem erro no esqueleto.
- [ ] Um teste de exemplo real passa na suíte.
- [ ] `.env.example` cobre todas as variáveis e o boot falha com erro claro se faltar uma obrigatória.
- [ ] Repositório git com `.gitignore`, lockfile e primeiro commit feitos.

## Formato do relatório final

Stack escolhida com justificativa amarrada às restrições, árvore de pastas comentada, lista dos comandos disponíveis, variáveis de ambiente exigidas, e o que ficou de fora do esqueleto (com o porquê).
