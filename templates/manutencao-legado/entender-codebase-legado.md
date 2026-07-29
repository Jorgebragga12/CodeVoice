# Entender codebase legado

> Modo: `planning` · Área: manutenção
> Uso: mapear um código desconhecido antes de mexer em qualquer coisa.

---

Mapeie o codebase do projeto **[nome do projeto]** sem alterar nenhum arquivo.

## Objetivo

<<SUA FALA>>

## Contexto

- Stack e linguagem: [stack]
- Como rodar: [comando ou "descobrir faz parte da tarefa"]
- Como testar: [comando de teste ou "descobrir faz parte da tarefa"]
- Foco prioritário: [área que mais importa agora, ou "visão geral"]

## Regras do mapeamento

1. **Somente leitura.** Nenhum arquivo é alterado; instale apenas o necessário para rodar o projeto.
2. **Descreva o que o código FAZ, não o que parece que deveria fazer.** Onde os dois divergirem (nome que mente, comentário desatualizado), registre a divergência — é onde bugs se escondem.
3. Rode o projeto e a suíte de testes para validar o entendimento na prática; anote o que não roda e por quê.
4. Derive tudo do código real (busca, leitura, execução). Não confie na documentação sem conferir — em legado ela costuma estar defasada.

## O que o mapa precisa cobrir

- **Entrypoints e fluxos principais**: por onde o sistema é acionado e os 3-5 caminhos de execução mais importantes, passo a passo.
- **Mapa de módulos**: responsabilidade de cada módulo/pasta e as dependências entre eles.
- **Regras de negócio**: onde vive cada regra importante (arquivo e função), especialmente as escondidas em lugar inesperado.
- **Pontos perigosos**: código sem teste, funções de complexidade alta, comentários de medo ("não mexer", "gambiarra", TODO antigo), dependências abandonadas.

## Formato da resposta

Mapa em Markdown que outro dev consiga usar sem esta conversa: visão geral em um parágrafo, entrypoints e fluxos, tabela de módulos (responsabilidade + depende de), localização das regras de negócio, pontos perigosos com o motivo de cada um, e o que ficou sem entender (com a pergunta certa para quem souber responder).
