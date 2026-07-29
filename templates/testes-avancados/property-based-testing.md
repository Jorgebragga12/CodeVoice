# Testes de propriedade

> Modo: `technical` · Área: testes
> Uso: adicionar property-based testing sobre invariantes reais do código.

---

Adicione testes de propriedade com **[fast-check/proptest/hypothesis]** no projeto **[nome do projeto]**.

## Alvo

<<SUA FALA>>

## Passo 1 — identifique invariantes reais

Derive as propriedades do comportamento do código; não invente asserções vagas. Padrões que costumam existir:

- **Roundtrip:** `decode(encode(x)) == x` para todo x válido.
- **Idempotência:** aplicar duas vezes = aplicar uma ([normalizar, sanitizar]).
- **Comutatividade/ordem:** resultado independe da ordem da entrada, quando isso for contrato.
- **Totalidade:** nunca lança exceção para entrada válida (só retorna erro tipado).
- **Oráculo:** resultado igual ao de uma implementação ingênua/lenta de referência.

Liste as propriedades escolhidas antes de escrever os testes; propriedade que apenas repete a implementação não prova nada.

## Regras dos geradores

1. Geradores cobrem o espaço real de entrada: string inclui unicode, vazia e muito longa; número inclui zero, negativos e os limites do tipo.
2. Restrinja o gerador só quando o domínio restringir — filtro demais esconde exatamente o bug que você procura.
3. **Shrinking ligado:** o framework deve reduzir a falha ao contraexemplo mínimo antes de você depurar.
4. **Semente fixada na falha:** registre a seed que falhou como teste de regressão determinístico.

## Escopo

Testes de propriedade COMPLEMENTAM os testes de exemplo — não delete casos de exemplo existentes; eles documentam comportamento concreto.

## Critérios de aceitação

- [ ] Propriedades listadas e justificadas (por que cada uma é invariante do código).
- [ ] Geradores exercitam vazio, unicode e valores-limite.
- [ ] Falhas reproduzíveis por seed registrada.
- [ ] Testes de exemplo existentes intactos.
- [ ] Suíte completa verde: [comando de teste].

## Formato do relatório final

Propriedades adicionadas, bugs encontrados por elas (com o contraexemplo mínimo de cada um), e propriedades consideradas e descartadas com o motivo.
