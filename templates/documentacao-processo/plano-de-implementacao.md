# Plano de Implementação (Quebra de Tarefa)

> Modo: `planning` · Área: documentação
> Uso: transformar um pedido grande ou vago em plano de etapas pequenas e verificáveis antes de codar.

---

Monte um plano de implementação para o pedido abaixo no projeto **[nome do projeto]**. NÃO escreva código ainda — primeiro o plano, depois a minha aprovação.

## Objetivo

<<SUA FALA>>

## Contexto do projeto

- Stack e versões relevantes: [linguagem, framework, banco]
- Restrições: [prazo, compatibilidade, o que não pode quebrar]
- O que já existe e deve ser reaproveitado: [módulos, serviços — ou escreva "descobrir lendo o código"]

## Regras do plano

1. **Leia o código antes de planejar.** Cada etapa deve citar os arquivos/módulos reais que serão tocados — derive do que existe, não invente estrutura.
2. **Quebre em etapas pequenas.** Cada etapa deve caber em uma sessão de trabalho e deixar o projeto em estado funcional (compilando, testes passando) ao terminar — etapa que quebra o build de todo mundo não é etapa, é risco.
3. **Toda etapa tem critério de pronto verificável**: um comando, teste ou verificação manual que prova que ela terminou. "Está pronto" sem prova não conta.
4. **Explicite dependências entre etapas** e ordene para que a incerteza maior apareça cedo — valide primeiro a parte que pode inviabilizar o resto, para falhar barato.
5. **Marque pontos de decisão.** Onde houver mais de um caminho razoável (biblioteca, modelagem, trade-off), liste as opções com prós e contras e pergunte — não decida sozinho.
6. **Sinalize etapas destrutivas ou arriscadas** (migração de dados, remoção de código, mudança de contrato de API): exigem confirmação explícita antes de executar e precisam de rota de reversão descrita.
7. **Liste riscos conhecidos** e o que fazer se cada um se confirmar: plano B ou ponto de parada para me consultar.
8. **Defina o que fica FORA do escopo**, para o plano não inchar durante a execução.

## O que o plano NÃO deve ter

- Estimativas de tempo inventadas — se não dá para estimar com base no código, diga isso.
- Etapas genéricas tipo "implementar backend": cada etapa nomeia o que muda e onde.
- Otimização de performance sem medição antes/depois prevista na própria etapa.

## Formato da resposta

1. **Resumo do entendimento** em 2-3 frases — se o pedido estiver ambíguo, faça as perguntas ANTES de propor o plano.
2. **Tabela de etapas**: número, descrição, arquivos afetados, dependências, critério de pronto.
3. **Pontos de decisão** que precisam da minha resposta, com opções e recomendação.
4. **Riscos** e **fora de escopo**.
5. Termine perguntando se aprovo o plano — só comece a implementar depois do meu OK explícito.
