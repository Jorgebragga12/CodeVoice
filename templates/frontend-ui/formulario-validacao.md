# Formulário com validação

> Modo: `ui_creation` · Área: frontend/UI
> Uso: formulário com validação, mensagens de erro e envio.

---

Crie o formulário **[nome/finalidade]** no projeto **[nome do projeto]**.

## Objetivo

<<SUA FALA>>

## Campos

[liste campos, tipos e regras de validação — ou derive do objetivo]

## Comportamento de validação

- Validar no **blur** do campo e novamente no submit (não a cada tecla, para não irritar).
- Mensagem de erro específica, junto do campo, dizendo **como corrigir** ("mínimo 8 caracteres", não "campo inválido").
- Botão de envio desabilitado apenas **durante** o envio (não antes — deixe o usuário tentar e mostre os erros).
- Duplo clique no enviar não gera requisição dupla.
- Erro do servidor → mensagem geral no topo, sem perder o que o usuário digitou.
- Sucesso → [feedback: toast/redirect/limpar formulário].

## Requisitos técnicos

- Validação replicada no servidor — a do cliente é só UX.
- Labels associados aos inputs (acessibilidade); erros anunciados para leitores de tela (`aria-describedby`).
- Use a lib de formulário já adotada no projeto, se houver.

## Critérios de aceitação

- [ ] Todas as regras de validação funcionando com mensagens claras.
- [ ] Estados de envio/erro/sucesso implementados.
- [ ] Testes cobrindo validação e envio (sucesso e falha).

## Formato do relatório final

Campos e regras finais, como testar cada validação, e o endpoint que recebe o envio.
