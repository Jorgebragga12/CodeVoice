# Não reproduz local

> Modo: `bug_fix` · Área: depuração
> Uso: quebra em produção (ou na máquina do outro) e funciona na minha — preciso descobrir a diferença.

---

Tenho um bug no projeto **[nome do projeto]** que não reproduz no meu ambiente.

## O problema

<<SUA FALA>>

## Onde acontece

- Quebra em: [produção / homologação / máquina de outra pessoa]
- Funciona em: [minha máquina / CI]
- Evidência que eu tenho: [log, print, relato do usuário — cole abaixo]

```
[COLE AQUI log/stack/print, se houver]
```

## Regras de investigação

1. **Pare de tentar reproduzir no chute. Faça a tabela de diferenças** entre o ambiente que quebra e o que funciona, e marque o que é **plausível causar ISSO**:
   - versão de runtime/lib/SO
   - **dados** (produção tem volume, acento, nulo, duplicata, registro antigo que o seed não tem)
   - concorrência e carga (só quebra com N simultâneos)
   - configuração e variáveis de ambiente
   - fuso horário e locale (formatação de data/número, ordenação)
   - **sistema de arquivos** (Windows não diferencia maiúscula de minúscula; Linux sim — import que funciona local e quebra no deploy)
   - rede: latência, timeout, proxy, DNS
   - permissões e usuário que roda o processo
2. **Instrumente em vez de adivinhar.** Se a evidência não basta, adicione log estruturado no caminho suspeito (com o dado que distingue as hipóteses) e me diga exatamente o que olhar quando reproduzir de novo. Log temporário de investigação deve ser marcado para remoção depois.
3. **Nunca logue dado sensível** para investigar: sem senha, token, CPF, cartão ou conteúdo pessoal — logue o formato/tamanho/hash, não o valor.
4. **Não mexa em produção para testar.** Se precisar de alguma ação lá, descreva o passo e o risco e **espere minha confirmação** — e sempre com plano de reverter.
5. Só proponha correção depois de ter uma hipótese que **explique por que funciona aqui e quebra lá**. Correção que não explica a assimetria é chute.

## Validações

- Um teste que reproduza a **condição** identificada (dado com acento, timezone diferente, chamada concorrente, arquivo com maiúscula), rodando localmente.
- Rodar: [comando de teste].

## Critérios de aceitação

- [ ] A diferença de ambiente que causa o bug está nomeada.
- [ ] Existe explicação de por que passava despercebido localmente.
- [ ] Teste cobre a condição, não só o resultado.
- [ ] Nenhum log de investigação com dado sensível ficou no código.

## Formato do relatório final

A diferença encontrada, por que ela causa o bug, a correção, o teste que fixa a condição, e o que remover depois (logs temporários).
