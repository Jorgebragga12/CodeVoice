# Ajuste pontual

> Modo: `quick` · Área: fala rápida
> Uso: mudança pequena e localizada — trocar texto, cor, valor, renomear, adicionar campo simples.

---

Faça um ajuste pontual no projeto **[nome do projeto]**.

## O ajuste

<<SUA FALA>>

## Regras

- **Faça exatamente o que eu pedi, e só isso.** Sem refatorar de brinde, sem "já que estou aqui", sem reorganizar import, sem trocar formatação de linha que eu não mencionei. Se você vir algo que merece atenção, **me diga no final** em vez de mudar.
- **Siga o padrão que já existe no arquivo** (nomenclatura, estilo, forma de resolver coisas parecidas). Um ajuste pontual não é hora de introduzir padrão novo.
- Se o ajuste tocar em mais de [3] arquivos ou exigir decisão de design, **pare e me avise** — provavelmente não é ajuste pontual e merece outro tratamento.
- Se algo no pedido estiver ambíguo, escolha a interpretação mais conservadora e diga qual escolheu.

## Validações

- Rodar [comando de teste] — mesmo em mudança pequena, para garantir que nada quebrou por tabela.
- Se houver lint configurado, deixá-lo verde.

## Critérios de aceitação

- [ ] O ajuste pedido está feito.
- [ ] Nenhuma outra mudança entrou junto.
- [ ] Testes e lint verdes.

## Formato do relatório final

Uma frase do que mudou e em quais arquivos. Se você notou algo que valeria mexer depois, liste separadamente como sugestão — sem ter mexido.
