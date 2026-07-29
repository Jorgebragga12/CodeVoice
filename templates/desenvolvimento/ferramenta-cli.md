# Ferramenta de linha de comando

> Modo: `new_feature` · Área: desenvolvimento
> Uso: criar um utilitário CLI (script ou binário).

---

Crie uma ferramenta de linha de comando em **[linguagem]**.

## Objetivo

<<SUA FALA>>

## Interface

- Comando: `[nome] [subcomando] [argumentos]`
- `--help` claro em todos os níveis, com exemplos de uso.
- Flags: [liste — ou derive do objetivo]
- Códigos de saída: 0 sucesso, 1 erro de execução, 2 uso incorreto.

## Requisitos técnicos

- Erros para o usuário em **stderr**, resultado em **stdout** (para permitir pipe).
- Entrada inválida → mensagem acionável dizendo o que corrigir, nunca stack trace cru.
- Operações destrutivas pedem confirmação, com flag `--yes` para automação.
- Se processar muitos itens, mostre progresso e suporte interrupção limpa (Ctrl+C).

## Validações

- Testes dos casos: uso correto, argumento faltando, entrada inválida, arquivo inexistente.
- Demonstre a saída real do `--help` e de uma execução de exemplo no relatório.

## Critérios de aceitação

- [ ] Funciona conforme o objetivo com a interface descrita.
- [ ] `--help` suficiente para alguém usar sem ler código.
- [ ] Testes verdes.

## Formato do relatório final

Como instalar/rodar, exemplos de uso reais (comando + saída), e limitações conhecidas.
