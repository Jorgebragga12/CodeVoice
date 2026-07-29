# Remover código morto

> Modo: `refactor` · Área: manutenção
> Uso: detectar e remover código sem uso, com evidência e reversão fácil.

---

Remova código morto no projeto **[nome do projeto]**.

## Escopo

<<SUA FALA>>

## Regras da remoção

1. **Morte exige evidência, não intuição**: zero referências no código (busca por símbolo E por string, para pegar uso dinâmico), análise estática da linguagem, e — quando houver — logs/telemetria de produção mostrando que o caminho não executa há [período].
2. **Reflexão, injeção de dependência, rotas dinâmicas e serialização enganam a análise estática.** Na dúvida se algo é chamado dinamicamente, instrumente (log/contador) e espere [janela de observação] antes de deletar — nunca delete no palpite.
3. Inclua no escopo: feature flags já resolvidas (e o código do braço morto), dependências declaradas e nunca importadas, exports sem importador, assets órfãos.
4. **Lotes pequenos, um commit por lote**, com a evidência de morte na mensagem. Reverter um lote não pode arrastar os outros.
5. Suíte completa verde entre cada lote; se o projeto tem build/typecheck, ambos passam também.
6. Código morto se deleta, não se comenta — o git é o histórico.
7. Remoção é passo destrutivo: apresente a lista do primeiro lote e espere confirmação explícita antes de deletar.

## Critérios de aceitação

- [ ] Cada remoção tem a evidência registrada no commit.
- [ ] Suíte, build e typecheck verdes após cada lote.
- [ ] Itens dúbios (possível chamada dinâmica) listados para instrumentação, não deletados.
- [ ] Nenhum bloco de código comentado deixado para trás.

## Formato do relatório final

Lotes removidos (commit, o que saiu, evidência), totais (linhas, arquivos, dependências), e a lista de suspeitos aguardando instrumentação com a janela de observação de cada um.
