# Dockerização

> Modo: `technical` · Área: DevOps
> Uso: containerizar a aplicação para dev e/ou produção.

---

Containerize o projeto **[nome do projeto]**.

## Objetivo

<<SUA FALA>>

## Requisitos técnicos

- **Multi-stage build**: estágio de build separado do de runtime; imagem final mínima ([alpine/slim/distroless]).
- Imagem base com versão pinada (ex.: `node:20-alpine`, nunca `latest`).
- `.dockerignore` cobrindo `node_modules`, `.git`, `.env`, artefatos de build.
- Rodar como usuário **não-root** no container final.
- Configuração 100% por variável de ambiente; `.env` nunca copiado para a imagem.
- Camadas ordenadas para cache: dependências antes do código-fonte.
- [docker-compose com app + banco + volumes para desenvolvimento local]
- Healthcheck definido.

## Validações

- `docker build` limpo e `docker run` com a aplicação respondendo — demonstre com comando + saída.
- Reporte o tamanho final da imagem.
- Confirme que nenhum segredo ficou na imagem **inspecionando o conteúdo, não o histórico**: `docker history` só mostra as instruções, então um segredo copiado por `COPY . .` (ou apagado num `RUN` posterior, que não remove da camada anterior) não aparece ali. Exporte e procure de verdade: `docker save [img] -o img.tar && tar -xOf img.tar | grep -aiE "api[_-]?key|secret|password|BEGIN .*PRIVATE KEY"` — ou rode um scanner de segredo na imagem. Confirme também que `.dockerignore` exclui `.env`, `.git` e credenciais.

## Critérios de aceitação

- [ ] Build e execução funcionando do zero (máquina limpa).
- [ ] Imagem final < [tamanho alvo, ex.: 200MB].
- [ ] Documentado no README: como buildar e rodar.

## Formato do relatório final

Comandos exatos para build/run, tamanho da imagem, variáveis de ambiente necessárias e decisões tomadas.
