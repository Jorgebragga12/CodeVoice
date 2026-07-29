# Build não compila

> Modo: `bug_fix` · Área: depuração
> Uso: erro de compilação, linker, toolchain ou ferramenta nativa faltando — o código nem chega a rodar.

---

O build do projeto **[nome do projeto]** está quebrado.

## Contexto

<<SUA FALA>>

## Saída do build

```
[COLE AQUI a saída completa do comando que falhou]
```

Comando que falhou: `[ex.: npm run build, cargo build, docker build]`
Mudou algo antes de quebrar? [nova dependência / upgrade / máquina nova / nada, quebrou do nada]

## Regras de investigação

1. **Vá para a PRIMEIRA mensagem de erro, não a última.** Compilador gera erro em cascata: um tipo não resolvido no topo produz dezenas de erros derivados embaixo. Corrigir o último é perseguir sintoma.
2. **Classifique o erro antes de agir** — as causas são diferentes:
   - **Resolução de dependência** (versão não existe, conflito): problema de manifesto/lockfile.
   - **Compilação** (tipo, sintaxe, trait não satisfeita): problema de código.
   - **Linker** (`undefined reference`, `LNK2019`, `cannot find -l...`): falta biblioteca nativa ou o alvo/ABI está errado.
   - **Ferramenta ausente** (`cmake not found`, `no C compiler`, SDK/toolchain): problema de ambiente, não de código.
3. **Ferramenta nativa faltando é causa comum e não se resolve mexendo no código.** Se o build precisa de compilador C/C++, cmake, SDK ou variável de ambiente, diga **qual** falta e **como instalar nesta plataforma** ([Windows/macOS/Linux]) — sem me mandar instalar coisa que já existe.
4. **Não faça upgrade/downgrade de versão no chute.** Se propuser mudar versão, diga qual restrição isso resolve.
5. **Limpar cache é último recurso, não primeiro.** Apagar `node_modules`/`target`/`.venv` às vezes resolve por acidente e apaga a evidência junto — se for necessário, explique o que isso indica sobre a causa.

## Validações

- Build limpo do zero: [comando] a partir de um estado limpo, com a saída como prova.
- Se envolveu dependência ou toolchain: o `[lockfile / README / .env.example]` foi atualizado para o próximo dev (ou a próxima máquina) não passar pelo mesmo.

## Critérios de aceitação

- [ ] Build passa do zero, não só incremental.
- [ ] A causa foi nomeada (dependência / código / linker / ambiente).
- [ ] Se foi ambiente, o pré-requisito está documentado com o comando de instalação.
- [ ] Nenhuma versão foi alterada sem justificativa explícita.

## Formato do relatório final

Qual das quatro classes de erro era, a causa concreta, o que mudou (arquivo/manifesto/ambiente), e o comando exato que reproduz o build limpo.
