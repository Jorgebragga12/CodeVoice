# Servidor MCP

> Modo: `new_feature` · Área: engenharia de IA
> Uso: criar um servidor MCP que expõe ferramentas do projeto para agentes de IA.

---

Crie um servidor MCP no projeto **[nome do projeto]**.

## Objetivo

<<SUA FALA>>

## Contexto a preencher

- Host onde será instalado: [Claude Code / Claude Desktop / outro]
- Recurso que as ferramentas expõem: [API interna / banco / arquivos / serviço]
- Linguagem/SDK: [TypeScript / Python / derivar do que o projeto já usa]

## Regras de design das ferramentas

1. **Nome e descrição que um LLM entende sem adivinhar**: a descrição diz o que a ferramenta faz, quando usar e o que devolve — o modelo só sabe o que a descrição conta.
2. **Schema de entrada estrito com validação**: tipos, enums e campos obrigatórios declarados; entrada inválida é rejeitada na porta, não no meio da execução.
3. **Erro que orienta a correção**: a resposta de erro diz o que estava errado e como chamar certo, porque o agente vai tentar de novo com base nela. Nunca engula erro silenciosamente.
4. **Menor privilégio**: exponha só as operações necessárias ao caso de uso; ação destrutiva (deletar, sobrescrever, enviar) exige parâmetro de confirmação explícita.
5. Logue cada chamada (ferramenta, argumentos, resultado ou erro, duração) — sem log não dá para entender o que o agente fez.

## Validações

- Teste cada ferramenta com um cliente MCP real conectado ao host, não só com teste unitário — o contrato importa no fio.
- Teste os caminhos de erro: entrada fora do schema, recurso inexistente, ação destrutiva sem confirmação.

## Critérios de aceitação

- [ ] Servidor conecta no host e as ferramentas aparecem listadas.
- [ ] Cada ferramenta funciona chamada por um cliente real de ponta a ponta.
- [ ] Entrada fora do schema retorna erro claro sem derrubar o servidor.
- [ ] Ação destrutiva sem confirmação explícita é recusada.
- [ ] README com instalação no host [Claude Code / outro], incluindo o bloco de configuração pronto para colar.

## Formato do relatório final

Ferramentas expostas (nome + uma linha cada), como instalar e rodar, o que foi testado com cliente real e limitações conhecidas.
