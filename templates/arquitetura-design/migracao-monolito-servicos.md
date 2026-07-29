# Extração de serviço do monolito

> Modo: `planning` · Área: arquitetura
> Uso: avaliar e planejar a extração de um serviço do monolito (strangler fig).

---

Avalie e planeje a extração de **[capacidade/módulo — ex.: faturamento]** do monolito do projeto **[nome do projeto]**.

## Motivação e situação atual

<<SUA FALA>>

## Contexto a preencher

- Dor concreta que motiva a extração: [ex.: deploy do módulo X trava o resto; times pisando no mesmo código]
- Tamanho do time e maturidade de operação: [CI/CD, observabilidade, on-call — o que já existe]
- Acoplamento conhecido: [tabelas compartilhadas, chamadas diretas, jobs em comum]

## Regras da avaliação

1. **Comece respondendo se vale a pena — e "não" é resposta válida.** Extrair serviço troca chamada de função por rede: latência, falha parcial, deploy duplo, observabilidade distribuída. Se a dor se resolve modularizando dentro do monolito, recomende isso.
2. **A fronteira é escolhida pelo domínio e pelos dados, não pela facilidade.** Módulo fácil de extrair mas com fronteira errada gera serviço acoplado — o pior dos dois mundos. Mapeie quais tabelas o módulo lê e escreve antes de propor o corte.
3. **Plano incremental (strangler fig): o monolito continua funcionando em produção após CADA passo.** Nada de big bang; cada etapa é pequena, reversível e entregável sozinha.
4. **Dados compartilhados têm estratégia explícita**: quem vira dono de cada tabela, como o outro lado passa a acessar (API, réplica de leitura, evento) e se haverá período de escrita dupla com verificação de consistência.
5. **Critérios de sucesso mensuráveis definidos antes de começar**: [ex.: deploy independente do módulo em menos de X min; incidente no módulo não derruba o resto]. Sem número, "melhorou" é opinião.
6. Passo que migra ou apaga dados de produção entra no plano marcado como **destrutivo — exige confirmação explícita** antes de executar, com backup verificado.

## Formato da resposta

1. Veredito: extrair, modularizar internamente ou não mexer — com justificativa.
2. Fronteira proposta (domínio + tabelas afetadas).
3. Plano passo a passo: entrega, risco e rollback de cada passo.
4. Estratégia para os dados compartilhados.
5. Critérios de sucesso mensuráveis.
6. Riscos e perguntas abertas.
