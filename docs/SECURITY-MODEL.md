# CodeVoice — SECURITY-MODEL

> Versão 0.1 · Fase 0 · 22/07/2026
> Regras obrigatórias para todas as fases. Violações bloqueiam a conclusão da fase.

## 1. Princípios

1. **Local por padrão**: áudio, transcrição e dados nunca saem da máquina, exceto quando o usuário usa o provedor `claude` CLI (que segue a conta/assinatura já configurada por ele).
2. **Nenhuma telemetria** sem opt-in explícito (não há telemetria no MVP).
3. **Texto ≠ comando**: transcrição e prompt são sempre dados. Nada gerado é executado como comando, em nenhuma circunstância.
4. **Menor privilégio**: cada janela Tauri recebe apenas as capabilities necessárias; o frontend não tem acesso a fs/shell/rede.

## 2. Superfícies de ameaça e mitigações

| Superfície                                 | Ameaça                                            | Mitigação                                                                                                                                                      |
| ------------------------------------------ | ------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Scanner de projetos                        | Ler/armazenar secrets (.env, chaves)              | Denylist absoluta (§3) + autorização explícita por projeto + preview do que será importado                                                                     |
| Caminho do projeto                         | Path traversal (`..\`, symlinks, UNC)             | Canonicalização (`dunce::canonicalize`) + rejeição de caminhos fora do diretório declarado + rejeição de `\\?\` truques e devices (`CON`, `NUL`…)              |
| Spawn de terminal / claude CLI             | Injeção de argumentos                             | Texto do usuário nunca em argv: sempre stdin ou clipboard; spawn com lista de args fixa (`CreateProcess` sem shell interpretando string); working dir validado |
| Prompt gerado colado no terminal           | Usuário colar comando malicioso ditado por engano | Colar somente sob ação explícita (ADR-005); nunca enviar Enter automaticamente                                                                                 |
| Logs                                       | Vazamento de secrets                              | Filtro no logger (§4); nunca logar conteúdo de transcrição/prompt em nível `info`                                                                              |
| Áudio temporário                           | Retenção indevida de voz                          | WAV em `%APPDATA%/com.jorgebraga.codevoice/tmp/`, excluído após processamento; opção "manter" off por padrão; limpeza de órfãos no startup                     |
| Banco SQLite                               | SQL injection                                     | Somente prepared statements (rusqlite); FTS5 com parâmetro bind                                                                                                |
| Download de modelo Whisper                 | Modelo adulterado / MITM                          | HTTPS + verificação SHA-256 contra hash embutido no binário                                                                                                    |
| Settings sensíveis (futuras chaves de API) | Armazenamento em texto plano                      | `keyring` → Windows Credential Manager; nunca no SQLite/store/JSON                                                                                             |
| Claude Code                                | Bypass de confirmações                            | Proibido usar `--dangerously-skip-permissions` ou equivalentes; proibido executar comandos destrutivos automaticamente                                         |

## 3. Denylist do scanner de projetos (absoluta)

Nunca ler, importar, armazenar ou logar:

- `.env`, `.env.*`, `*.pem`, `*.key`, `*.pfx`, `*.p12`, `id_rsa*`, `*.crt` privados
- Qualquer arquivo cujo nome contenha `secret`, `credential`, `token` (case-insensitive)
- Diretórios: `.git/`, `node_modules/`, `dist/`, `build/`, `target/`, `.next/`, `out/`, `coverage/`, `venv/`, `.venv/`, `__pycache__/`
- Binários e mídia (extensões não-texto); arquivos > 512 KB

Allowlist de importação assistida (só estes são lidos, e só com autorização): `CLAUDE.md`, `README.md`, `package.json`, `tsconfig.json`, `Cargo.toml`, `pyproject.toml`, `composer.json`, `go.mod`, `docker-compose*.yml`, `.nvmrc`, e listagem de diretórios (nomes apenas, 2 níveis, respeitando a denylist).

Implementação em `src-tauri/src/security/` + `projects/scanner.rs`, com testes unitários cobrindo: `.env` na raiz, symlink para fora do projeto, `..\..\`, caminho UNC, arquivo `secrets.json`.

## 4. Filtro de secrets nos logs

Camada de sanitização aplicada a toda mensagem de log antes da escrita:

- Redação por regex de padrões conhecidos: `sk-ant-*`, `ghp_*`, `github_pat_*`, `AKIA[0-9A-Z]{16}`, `Bearer …`, `password=`, `token=`, JWTs (`eyJ…`), chaves PEM.
- Conteúdo de transcrição/prompt só é logado em nível `trace` (nunca habilitado por padrão) e mesmo assim passa pelo filtro.
- Teste unitário obrigatório: logar string com cada padrão e assertar redação.

## 5. Confirmações destrutivas

Exigem diálogo de confirmação: excluir projeto, excluir item do histórico, excluir modelo de prompt, limpar histórico, trocar/apagar modelo Whisper baixado. Exclusões usam a lixeira lógica? **Não** — exclusão real com confirmação (simplicidade no MVP), exceto histórico que pode ganhar undo em memória na sessão.

## 6. Checklist de segurança por fase (colar no relatório de cada fase)

- [ ] Nenhum secret em logs (rodar teste do filtro)
- [ ] Nenhum caminho aceito sem canonicalização/validação
- [ ] Nenhum texto de usuário interpolado em linha de comando
- [ ] Capabilities Tauri da fase revisadas (mínimo necessário)
- [ ] Nenhuma dependência nova sem justificativa
- [ ] Ações destrutivas com confirmação
