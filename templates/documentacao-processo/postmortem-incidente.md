# Postmortem de incidente

> Modo: `quick` · Área: documentação
> Uso: escrever postmortem sem culpados de um incidente já resolvido.

---

Escreva o postmortem do incidente **[identificador/título]** do serviço **[nome do serviço]**.

## O que aconteceu

<<SUA FALA>>

## Material disponível

- Fontes: [logs, canal do incidente, gráficos, timeline da ferramenta de alerta]
- Derive a linha do tempo dessas fontes; horário sem fonte vira "aprox. [hh:mm]" — nunca invente precisão.

## Estrutura obrigatória

1. **Resumo em 3 frases:** o que quebrou, por quanto tempo, qual o impacto.
2. **Linha do tempo objetiva com horários:** detecção, diagnóstico, mitigação, resolução. Fatos, não julgamentos — "deploy X às 14:03", não "fulano errou o deploy".
3. **Impacto quantificado:** usuários/requisições/receita afetados e duração, com a fonte de cada número ([número] onde não houver dado).
4. **Causa raiz técnica E de processo** usando 5 porquês. Regra dura: NÃO pare em "erro humano" — erro humano é sintoma de um sistema que permite o erro; continue perguntando por que o sistema permitiu.
5. **O que funcionou:** detecção, resposta, ferramentas — o postmortem também consolida o que manter.
6. **Ações concretas com dono e prazo.** "Ter mais cuidado" não é ação; "adicionar validação X no pipeline de deploy — [dono], até [data]" é.
7. **Detecção e prevenção:** o que teria pego isso antes (alerta, teste, gate de deploy) — cada item é candidato a virar ação.

## Regras

- Sem nomes como culpados; use papéis só quando necessário ("o dev de plantão").
- Não invente horário, número ou causa: o que não estiver nas fontes fica marcado [confirmar].
- Toda ação precisa de dono e prazo — ação sem dono é ação que não acontece.

## Formato da resposta

O postmortem completo em Markdown na estrutura acima, seguido de:

- Lista dos itens marcados [confirmar].
- As 3 ações de maior efeito preventivo, destacadas em uma linha cada.
