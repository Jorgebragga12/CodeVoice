# Conflito de dependências

> Modo: `bug_fix` · Área: depuração
> Uso: versões incompatíveis, lockfile quebrado, "peer dependency" reclamando, duas versões da mesma lib.

---

Tenho conflito de dependências no projeto **[nome do projeto]**.

## O que aconteceu

<<SUA FALA>>

## Saída do gerenciador

```
[COLE AQUI o erro do npm/cargo/pip/composer]
```

## Regras de investigação

1. **Veja a árvore antes de mexer no manifesto.** Rode [`npm ls [pacote]` / `cargo tree -d` / `pip show`] e me diga **quem** exige cada versão. Conflito não se resolve no arquivo onde ele aparece, e sim em quem puxa a restrição.
2. **Duas versões da mesma lib nem sempre é problema** — depende do ecossistema: no npm, coexistir costuma funcionar (exceto para libs com estado global, como React, ou singleton de contexto); no Cargo, dois `semver` maiores diferentes geram tipos incompatíveis e o erro aparece como "expected X, found X" (o mesmo nome, tipos distintos), que é confuso mas é isso.
3. **Não force `resolutions`/`overrides`/`patch` como primeira saída.** Isso obriga uma lib a rodar com versão que ela não declarou suportar — pode funcionar hoje e quebrar de forma sutil. Se for mesmo a saída, diga qual incompatibilidade você verificou e deixe registrado no manifesto **por que** o override existe.
4. **O lockfile é a fonte da verdade do que está instalado.** Se ele diverge do manifesto, resolva a divergência explicitamente — não apague o lockfile para "resolver": isso troca um problema conhecido por um conjunto de versões novo e não testado.
5. **Prefira a menor mudança que destrava**: atualizar só o pacote que exige, em vez de subir tudo junto. Upgrade em massa transforma um problema em vários ao mesmo tempo.
6. Se a única saída for **subir uma versão maior** (breaking change), diga o que quebra e trate como tarefa própria — não misture com a correção do conflito.

## Validações

- Instalação limpa a partir do zero ([apagar diretório de deps + instalar]) sem aviso de conflito.
- Build e testes verdes: [comandos].
- Se o app tem funcionalidade que depende da lib em conflito, exercitar essa funcionalidade — conflito de versão costuma passar no build e quebrar em runtime.

## Critérios de aceitação

- [ ] Está explicado quem exigia qual versão.
- [ ] A solução é a menor mudança possível, não um upgrade geral.
- [ ] Se houve override/resolution, o motivo está registrado no manifesto.
- [ ] Lockfile atualizado e commitado junto.

## Formato do relatório final

Quem conflitava com quem, a versão escolhida e por quê, o que foi alterado (manifesto/lock), e o risco residual se houver override.
