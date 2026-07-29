# Upload de arquivos

> Modo: `new_feature` · Área: backend
> Uso: implementar upload de arquivos sem abrir brecha de segurança.

---

Implemente upload de arquivos no projeto **[nome do projeto]**.

## Escopo

<<SUA FALA>>

## Contexto a preencher

- Tipos aceitos: [ex.: jpg/png/webp, PDF] · Tamanho máximo: [valor]
- Destino: [object storage / disco do servidor]

## Regras de segurança (inegociáveis)

1. **Valide o tipo pelo CONTEÚDO (magic bytes), nunca pela extensão ou pelo Content-Type** — os dois são controlados pelo atacante. Tipo fora da lista → rejeita com erro claro.
2. **Limite de tamanho aplicado no servidor** (e no proxy/framework, para cortar o stream cedo), não só no front.
3. **O nome original do arquivo nunca vira caminho.** Gere nome próprio ([UUID]); o nome original é só metadado de exibição. Isso elimina path traversal e colisão de nomes.
4. Armazene **fora do webroot** ou em object storage; entregue via **URL assinada com expiração** ou endpoint que checa permissão — nunca arquivo servido direto de pasta pública com o nome que o usuário mandou.
5. **Imagens são re-processadas** (redimensionar/re-encodar) antes de servir — remove payload embutido e metadados EXIF sensíveis (localização, dispositivo).
6. [Se aplicável] **Scan antivírus/malware** antes de o arquivo ficar acessível a outros usuários.

## Experiência de upload

- Progresso real no cliente; erro de tipo/tamanho aparece o quanto antes, sem esperar o upload inteiro quando evitável.
- [Se houver arquivos grandes] Upload em partes com retomada ([multipart/chunks]), para falha de rede não recomeçar do zero.

## Critérios de aceitação

- [ ] Executável renomeado para .jpg é rejeitado (teste validando magic bytes)
- [ ] Arquivo acima do limite é rejeitado no servidor (teste)
- [ ] Nome malicioso tipo `../../etc/passwd` não afeta o caminho salvo (teste)
- [ ] Arquivo não é acessível por URL pública sem assinatura/permissão
- [ ] Imagem enviada é re-processada e perde metadados EXIF
- [ ] Erros de validação chegam claros ao usuário, nunca falham em silêncio

## Formato do relatório final

Fluxo do upload com as validações na ordem em que rodam, onde os arquivos ficam, como são servidos, e os testes de segurança executados.
