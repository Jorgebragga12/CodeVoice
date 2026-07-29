# Entender o erro (sem corrigir ainda)

> Modo: `quick` · Área: depuração
> Uso: só quero entender o que a mensagem quer dizer antes de decidir o que fazer.

---

Me explique este erro do projeto **[nome do projeto]**.

## Contexto

<<SUA FALA>>

## Mensagem

```
[COLE AQUI a mensagem/stack completa]
```

## O que eu quero

1. **Tradução**: o que essa mensagem está dizendo, em português claro, sem jargão desnecessário.
2. **Onde**: qual linha do **meu** código está envolvida (ignore os frames internos de biblioteca, exceto se a causa estiver mesmo lá).
3. **Por que acontece**: a condição que leva a esse erro — o que precisa ser verdade no estado do programa para ele estourar.
4. **Causas prováveis**, em ordem, com o que verificar em cada uma para confirmar ou descartar.
5. **É grave?** Se isso chegar em produção, o que quebra e para quem.

## Regras

- **NÃO corrija nada ainda.** Nem edite arquivo, nem proponha patch pronto. Só explicação — eu decido o passo seguinte.
- Se a mensagem for ambígua ou faltar contexto para explicar com segurança, diga o que falta em vez de preencher com suposição.
- Se for erro conhecido/comum daquela stack, diga isso — saber que é pegadinha clássica muda minha decisão.

## Formato da resposta

Explicação curta em prosa (não bullet gigante), a linha suspeita do meu código, as causas prováveis em ordem, e a pergunta que eu deveria responder para escolher entre elas.
