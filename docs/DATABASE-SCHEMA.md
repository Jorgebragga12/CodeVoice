# CodeVoice — DATABASE-SCHEMA

> Versão 0.1 · Fase 0 · 22/07/2026
> SQLite via `rusqlite` (bundled), acesso exclusivo no lado Rust (ADR-003). Banco em `%APPDATA%/com.jorgebraga.codevoice/codevoice.db`, WAL mode, `foreign_keys = ON`.

## 1. Convenções

- IDs: `INTEGER PRIMARY KEY` (rowid).
- Timestamps: `TEXT` ISO-8601 UTC (`2026-07-22T18:00:00Z`), colunas `created_at`/`updated_at` com default `strftime('%Y-%m-%dT%H:%M:%SZ','now')`.
- Booleanos: `INTEGER` 0/1 com `CHECK`.
- Enums: `TEXT` com `CHECK (col IN (...))`.
- Caminhos de arquivo: sempre absolutos, canonicalizados pelo Rust antes do INSERT (ver SECURITY-MODEL.md §3).

## 2. Migrations e versionamento

Migrations sequenciais embutidas no binário (`storage/migrations.rs`), aplicadas em transação na inicialização:

```sql
CREATE TABLE IF NOT EXISTS schema_migrations (
  version     INTEGER PRIMARY KEY,          -- 1, 2, 3…
  name        TEXT NOT NULL,                -- '001_initial'
  applied_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now'))
);
```

Regras: nunca editar migration já aplicada; toda mudança de esquema é uma nova migration `NNN_descricao`; cada migration tem teste que aplica do zero e sobre a versão anterior; `PRAGMA user_version` espelha a última versão aplicada. Downgrade não suportado (backup do arquivo `.db` antes de aplicar migrations em versão nova do app).

## 3. Esquema (migration 001_initial)

```sql
-- Projetos cadastrados
CREATE TABLE projects (
  id                     INTEGER PRIMARY KEY,
  name                   TEXT NOT NULL,
  path                   TEXT NOT NULL UNIQUE,      -- absoluto, canonicalizado
  description            TEXT NOT NULL DEFAULT '',
  stack                  TEXT NOT NULL DEFAULT '',  -- texto livre (ex.: "Tauri 2, React, SQLite")
  architecture           TEXT NOT NULL DEFAULT '',
  dev_commands           TEXT NOT NULL DEFAULT '',  -- 1 comando por linha
  test_commands          TEXT NOT NULL DEFAULT '',
  forbidden_tech         TEXT NOT NULL DEFAULT '',
  database_info          TEXT NOT NULL DEFAULT '',
  notes                  TEXT NOT NULL DEFAULT '',
  created_at             TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now')),
  updated_at             TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now'))
);

-- Regras por projeto (injetadas no contexto do prompt)
CREATE TABLE project_rules (
  id          INTEGER PRIMARY KEY,
  project_id  INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
  rule        TEXT NOT NULL,
  sort_order  INTEGER NOT NULL DEFAULT 0,
  created_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now'))
);
CREATE INDEX idx_project_rules_project ON project_rules(project_id);

-- Modelos de prompt salvos pelo usuário ("salvar como modelo" no editor)
-- ATENÇÃO: substituída pela migration 003 (ver §5) — esta é a forma original, da 001.
CREATE TABLE prompt_templates (
  id          INTEGER PRIMARY KEY,
  name        TEXT NOT NULL,
  mode        TEXT NOT NULL,                -- ver CHECK de generated_prompts.mode
  content     TEXT NOT NULL,
  project_id  INTEGER REFERENCES projects(id) ON DELETE SET NULL,  -- NULL = global
  created_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now')),
  updated_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now'))
);

-- Gravações (metadados; o WAV é temporário e normalmente já foi excluído)
CREATE TABLE recordings (
  id            INTEGER PRIMARY KEY,
  project_id    INTEGER REFERENCES projects(id) ON DELETE SET NULL,
  duration_ms   INTEGER NOT NULL,
  device_name   TEXT NOT NULL DEFAULT '',
  audio_path    TEXT,                        -- NULL após exclusão (padrão)
  audio_kept    INTEGER NOT NULL DEFAULT 0 CHECK (audio_kept IN (0,1)),
  status        TEXT NOT NULL DEFAULT 'recorded'
                CHECK (status IN ('recorded','transcribing','transcribed','failed','cancelled')),
  created_at    TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now'))
);

-- Transcrições
CREATE TABLE transcriptions (
  id             INTEGER PRIMARY KEY,
  recording_id   INTEGER NOT NULL REFERENCES recordings(id) ON DELETE CASCADE,
  text           TEXT NOT NULL,
  language       TEXT NOT NULL DEFAULT 'pt',
  engine         TEXT NOT NULL DEFAULT 'whisper-rs',   -- p/ futura API
  model_name     TEXT NOT NULL DEFAULT '',             -- ex.: 'large-v3-turbo'
  duration_ms    INTEGER NOT NULL DEFAULT 0,           -- tempo de processamento
  created_at     TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now'))
);
CREATE INDEX idx_transcriptions_recording ON transcriptions(recording_id);

-- Prompts gerados
CREATE TABLE generated_prompts (
  id               INTEGER PRIMARY KEY,
  transcription_id INTEGER REFERENCES transcriptions(id) ON DELETE SET NULL,
  project_id       INTEGER REFERENCES projects(id) ON DELETE SET NULL,
  mode             TEXT NOT NULL CHECK (mode IN (
                     'clean_transcript','quick','technical','new_feature','bug_fix',
                     'refactor','planning','code_review','ui_creation','db_change')),
  generator        TEXT NOT NULL CHECK (generator IN ('claude_cli','template')),
  content          TEXT NOT NULL,            -- versão atual (após edições)
  original_content TEXT NOT NULL,            -- como foi gerado
  created_at       TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now')),
  updated_at       TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now'))
);
CREATE INDEX idx_generated_prompts_project ON generated_prompts(project_id);

-- Histórico (a linha que a tela de Histórico lista; junta o fluxo inteiro)
CREATE TABLE prompt_history (
  id                  INTEGER PRIMARY KEY,
  project_id          INTEGER REFERENCES projects(id) ON DELETE SET NULL,
  recording_id        INTEGER REFERENCES recordings(id) ON DELETE SET NULL,
  transcription_id    INTEGER REFERENCES transcriptions(id) ON DELETE SET NULL,
  generated_prompt_id INTEGER REFERENCES generated_prompts(id) ON DELETE SET NULL,
  mode                TEXT NOT NULL,
  favorite            INTEGER NOT NULL DEFAULT 0 CHECK (favorite IN (0,1)),
  audio_duration_ms   INTEGER NOT NULL DEFAULT 0,
  status              TEXT NOT NULL DEFAULT 'completed'
                      CHECK (status IN ('completed','copied','sent_to_terminal','failed')),
  created_at          TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now'))
);
CREATE INDEX idx_history_project ON prompt_history(project_id);
CREATE INDEX idx_history_created ON prompt_history(created_at DESC);
CREATE INDEX idx_history_favorite ON prompt_history(favorite) WHERE favorite = 1;

-- Configurações não-sensíveis (chave-valor JSON; sensíveis vão pro Credential Manager)
CREATE TABLE app_settings (
  key        TEXT PRIMARY KEY,      -- ex.: 'hotkey', 'microphone', 'whisper_model',
                                    --      'keep_audio', 'autostart', 'default_mode'
  value      TEXT NOT NULL,         -- JSON
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now'))
);
```

## 4. Busca full-text (migration 002_fts)

```sql
CREATE VIRTUAL TABLE history_fts USING fts5(
  transcript,
  prompt,
  history_id UNINDEXED,                      -- id de prompt_history, pra fazer o join na busca
  tokenize = 'unicode61 remove_diacritics 2'  -- busca sem acentos (PT)
);
```

**Revisão da Fase 2**: o design original desta seção previa uma FTS5 "external content" (`content=''`) alimentada por triggers SQL em `transcriptions`/`generated_prompts`. Isso foi abandonado na implementação: o texto buscável vem de duas tabelas diferentes que não compartilham rowid com `prompt_history`, o que tornaria os triggers frágeis. Em vez disso, `history_fts` é uma tabela independente, **populada explicitamente pelo código Rust** (`HistoryRepo::save_flow`, na mesma transação que grava `prompt_history`) — não por triggers.

A pesquisa do Histórico usa `history_fts MATCH ?` com join em `history_id`, combinado com filtros `project_id`/`mode`/`favorite` na `prompt_history`. Sanitização de caracteres especiais da query FTS5 fica para a Fase 8 (tela real de busca).

## 5. Biblioteca de modelos (migration 003_prompt_templates)

A Fase 7 importou os **117 modelos** de `templates/` (18 categorias) para dentro do app. A tabela
da 001 não comportava isso, e SQLite não tem `ALTER TABLE ADD CONSTRAINT`, então a 003 **recria**
a tabela copiando as linhas existentes:

```sql
CREATE TABLE prompt_templates (
  id          INTEGER PRIMARY KEY,
  name        TEXT NOT NULL,
  mode        TEXT NOT NULL CHECK (mode IN (   -- mesmo CHECK de generated_prompts.mode
                'clean_transcript','quick','technical','new_feature','bug_fix',
                'refactor','planning','code_review','ui_creation','db_change')),
  category    TEXT NOT NULL DEFAULT '',        -- slug da pasta em templates/ (ex.: 'depuracao')
  description TEXT NOT NULL DEFAULT '',        -- a linha "> Uso:" do modelo
  content     TEXT NOT NULL,                   -- só o corpo, sem o cabeçalho de metadados
  source      TEXT NOT NULL DEFAULT 'user' CHECK (source IN ('builtin','user')),
  slug        TEXT,                            -- 'categoria/arquivo' nos builtin; NULL nos do usuário
  project_id  INTEGER REFERENCES projects(id) ON DELETE SET NULL,
  created_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now')),
  updated_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now'))
);
CREATE UNIQUE INDEX idx_prompt_templates_slug ON prompt_templates(slug) WHERE slug IS NOT NULL;
CREATE INDEX idx_prompt_templates_category ON prompt_templates(category);
```

**Origem das linhas.** `source = 'builtin'` são os 117 modelos embutidos no binário por `build.rs`
(via `include_str!`); a cada startup eles são apagados e reinseridos a partir do binário, o que
mantém banco e código em sincronia após um update — inclusive quando um modelo é renomeado ou
removido. `source = 'user'` são os do "salvar como modelo": nunca tocados pelo seed, e os únicos
que podem ser excluídos pela UI.

**Categoria = pasta, não o campo `Área`.** Os arquivos declaram uma `Área` livre no cabeçalho, mas
há 22 valores distintos para 18 pastas (`negocios-produto/` sozinha produz "negócios" e "produto").
A pasta é o que casa com a navegação do repositório e com o índice do README.

### 5.1 Convenção de placeholders

Uma só, travada na Fase 7: **`<<SUA FALA>>`** marca onde entra a transcrição. A sintaxe
`{{transcript}}`/`{{project_context}}`, documentada na versão 0.1 deste arquivo, nunca chegou a
virar código e foi descartada — `<<SUA FALA>>` já está nos 117 arquivos versionados e é legível
para quem lê os modelos direto no repositório.

Além dela, `promptgen::library::render` substitui apenas os campos entre colchetes que o app tem
como **dado** no banco: `[nome do projeto]` (`projects.name`) e `[comando de teste]`
(`projects.test_commands`). Os outros ~560 campos entre colchetes da biblioteca são decisões do
usuário (`[N]`, `[valor]`, `[período]`) e ficam **literais** — inclusive `[nome do projeto]` quando
não há projeto ativo, porque um literal visível pede preenchimento e um vazio silencioso não.

## 6. Regras de integridade no código

- Repositórios expõem operações de alto nível (`save_flow(recording, transcription, prompt)`) em transação única — nunca metade do fluxo persistido.
- `projects.path` é validado/canonicalizado antes de qualquer INSERT/UPDATE.
- Exclusão de item do histórico apaga em cascata transcrição/prompt vinculados **somente** se não referenciados por outro item (no MVP a relação é 1:1, então cascade direto).
- Toda escrita atualiza `updated_at` via código (não confiar em trigger).
