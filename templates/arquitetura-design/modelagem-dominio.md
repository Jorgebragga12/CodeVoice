# Modelagem de domínio

> Modo: `planning` · Área: arquitetura
> Uso: modelar entidades, agregados e regras de negócio antes de escrever código.

---

Modele o domínio de **[área de negócio — ex.: cobrança, agendamento]** do projeto **[nome do projeto]**.

## Como o negócio funciona

<<SUA FALA>>

## Contexto a preencher

- Termos que o negócio usa no dia a dia: [ex.: "pedido", "reserva", "fatura" — como as pessoas realmente falam]
- Regras que NUNCA podem ser violadas: [ex.: pedido pago não pode ser editado]
- O que já existe de código ou modelo: [tabelas, classes — ou "campo verde"]

## Regras da modelagem

1. **O modelo reflete o negócio real, não a estrutura do banco.** Use os termos do glossário em classe, tabela e API — se o negócio fala "reserva", não crie `BookingRequestItem`. Valide comigo qualquer termo que você inferir em vez de assumir.
2. **Monte a linguagem ubíqua**: glossário com termo → definição → nome no código. Ambiguidade de vocabulário aqui vira bug de regra de negócio depois.
3. **Defina agregados e fronteiras**: o que precisa mudar junto na mesma transação fica no mesmo agregado; referência entre agregados é por ID, não por objeto — fronteira frouxa vira transação gigante.
4. **Toda invariante vira validação no código do próprio agregado** — não em service externo nem só no frontend. Modelo anêmico (dados aqui, regra espalhada ali) é proibido: se a regra é do domínio, ela mora no domínio.
5. Liste os **eventos de domínio** (fatos no passado, ex.: `PedidoPago`) que outros módulos podem precisar consumir — eles são a fronteira natural de integração.
6. Aponte o que você NÃO conseguiu decidir com a informação dada e formule as perguntas para o negócio, em linguagem de negócio.

## Formato da resposta

1. Glossário (linguagem ubíqua): termo → definição → nome no código.
2. Entidades e agregados com fronteiras marcadas (diagrama ou lista aninhada).
3. Tabela: invariante → agregado onde é validada.
4. Eventos de domínio com o momento em que ocorrem.
5. Perguntas abertas para o negócio.
