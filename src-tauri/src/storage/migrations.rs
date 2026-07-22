use rusqlite::Connection;

/// Cada migration é aplicada em ordem, uma única vez, controlada por `PRAGMA user_version`.
/// Nunca editar uma migration já publicada — sempre adicionar uma nova ao final desta lista.
/// Ver DATABASE-SCHEMA.md.
const MIGRATIONS: &[(i64, &str, &str)] = &[(1, "001_initial", MIGRATION_001), (2, "002_fts", MIGRATION_002)];

const MIGRATION_001: &str = r#"
CREATE TABLE schema_migrations (
  version     INTEGER PRIMARY KEY,
  name        TEXT NOT NULL,
  applied_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now'))
);

CREATE TABLE projects (
  id                     INTEGER PRIMARY KEY,
  name                   TEXT NOT NULL,
  path                   TEXT NOT NULL UNIQUE,
  description            TEXT NOT NULL DEFAULT '',
  stack                  TEXT NOT NULL DEFAULT '',
  architecture           TEXT NOT NULL DEFAULT '',
  dev_commands           TEXT NOT NULL DEFAULT '',
  test_commands          TEXT NOT NULL DEFAULT '',
  forbidden_tech         TEXT NOT NULL DEFAULT '',
  database_info          TEXT NOT NULL DEFAULT '',
  notes                  TEXT NOT NULL DEFAULT '',
  created_at             TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now')),
  updated_at             TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now'))
);

CREATE TABLE project_rules (
  id          INTEGER PRIMARY KEY,
  project_id  INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
  rule        TEXT NOT NULL,
  sort_order  INTEGER NOT NULL DEFAULT 0,
  created_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now'))
);
CREATE INDEX idx_project_rules_project ON project_rules(project_id);

CREATE TABLE prompt_templates (
  id          INTEGER PRIMARY KEY,
  name        TEXT NOT NULL,
  mode        TEXT NOT NULL,
  content     TEXT NOT NULL,
  project_id  INTEGER REFERENCES projects(id) ON DELETE SET NULL,
  created_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now')),
  updated_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now'))
);

CREATE TABLE recordings (
  id            INTEGER PRIMARY KEY,
  project_id    INTEGER REFERENCES projects(id) ON DELETE SET NULL,
  duration_ms   INTEGER NOT NULL,
  device_name   TEXT NOT NULL DEFAULT '',
  audio_path    TEXT,
  audio_kept    INTEGER NOT NULL DEFAULT 0 CHECK (audio_kept IN (0,1)),
  status        TEXT NOT NULL DEFAULT 'recorded'
                CHECK (status IN ('recorded','transcribing','transcribed','failed','cancelled')),
  created_at    TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now'))
);

CREATE TABLE transcriptions (
  id             INTEGER PRIMARY KEY,
  recording_id   INTEGER NOT NULL REFERENCES recordings(id) ON DELETE CASCADE,
  text           TEXT NOT NULL,
  language       TEXT NOT NULL DEFAULT 'pt',
  engine         TEXT NOT NULL DEFAULT 'whisper-rs',
  model_name     TEXT NOT NULL DEFAULT '',
  duration_ms    INTEGER NOT NULL DEFAULT 0,
  created_at     TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now'))
);
CREATE INDEX idx_transcriptions_recording ON transcriptions(recording_id);

CREATE TABLE generated_prompts (
  id               INTEGER PRIMARY KEY,
  transcription_id INTEGER REFERENCES transcriptions(id) ON DELETE SET NULL,
  project_id       INTEGER REFERENCES projects(id) ON DELETE SET NULL,
  mode             TEXT NOT NULL CHECK (mode IN (
                     'clean_transcript','quick','technical','new_feature','bug_fix',
                     'refactor','planning','code_review','ui_creation','db_change')),
  generator        TEXT NOT NULL CHECK (generator IN ('claude_cli','template')),
  content          TEXT NOT NULL,
  original_content TEXT NOT NULL,
  created_at       TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now')),
  updated_at       TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now'))
);
CREATE INDEX idx_generated_prompts_project ON generated_prompts(project_id);

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

CREATE TABLE app_settings (
  key        TEXT PRIMARY KEY,
  value      TEXT NOT NULL,
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now'))
);
"#;

/// FTS5 é uma tabela independente (não "external content"), populada explicitamente pelo
/// código Rust em `HistoryRepo::save_flow` — não por triggers SQL. O motivo: o texto buscável
/// vem de duas tabelas diferentes (transcriptions.text, generated_prompts.content) que não
/// compartilham rowid com prompt_history, o que tornaria triggers de sincronização frágeis.
/// `history_id` guarda o id de `prompt_history` pra fazer o join na hora da busca.
const MIGRATION_002: &str = r#"
CREATE VIRTUAL TABLE history_fts USING fts5(
  transcript,
  prompt,
  history_id UNINDEXED,
  tokenize = 'unicode61 remove_diacritics 2'
);
"#;

pub fn apply_all(conn: &mut Connection) -> rusqlite::Result<()> {
    let current: i64 = conn.query_row("PRAGMA user_version", [], |row| row.get(0))?;

    for &(version, name, sql) in MIGRATIONS {
        if version <= current {
            continue;
        }
        let tx = conn.transaction()?;
        tx.execute_batch(sql)?;
        tx.execute(
            "INSERT INTO schema_migrations (version, name) VALUES (?1, ?2)",
            (version, name),
        )?;
        tx.pragma_update(None, "user_version", version)?;
        tx.commit()?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table_names(conn: &Connection) -> Vec<String> {
        let mut stmt = conn
            .prepare("SELECT name FROM sqlite_master WHERE type IN ('table','view') ORDER BY name")
            .unwrap();
        stmt.query_map([], |row| row.get::<_, String>(0))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    }

    #[test]
    fn applies_from_zero_creates_every_table() {
        let mut conn = Connection::open_in_memory().unwrap();
        apply_all(&mut conn).unwrap();

        let tables = table_names(&conn);
        for expected in [
            "schema_migrations",
            "projects",
            "project_rules",
            "prompt_templates",
            "recordings",
            "transcriptions",
            "generated_prompts",
            "prompt_history",
            "app_settings",
            "history_fts",
        ] {
            assert!(tables.iter().any(|t| t == expected), "tabela {expected} não foi criada");
        }

        let version: i64 = conn.query_row("PRAGMA user_version", [], |r| r.get(0)).unwrap();
        assert_eq!(version, 2);

        let applied: i64 = conn.query_row("SELECT COUNT(*) FROM schema_migrations", [], |r| r.get(0)).unwrap();
        assert_eq!(applied, 2);
    }

    #[test]
    fn applying_twice_is_idempotent() {
        let mut conn = Connection::open_in_memory().unwrap();
        apply_all(&mut conn).unwrap();
        apply_all(&mut conn).unwrap();

        let applied: i64 = conn.query_row("SELECT COUNT(*) FROM schema_migrations", [], |r| r.get(0)).unwrap();
        assert_eq!(applied, 2);
    }

    #[test]
    fn applies_incrementally_from_partial_state() {
        let mut conn = Connection::open_in_memory().unwrap();

        // Simula um banco que já tem só a migration 001 aplicada.
        conn.execute_batch(MIGRATION_001).unwrap();
        conn.execute(
            "INSERT INTO schema_migrations (version, name) VALUES (1, '001_initial')",
            [],
        )
        .unwrap();
        conn.pragma_update(None, "user_version", 1).unwrap();

        apply_all(&mut conn).unwrap();

        let version: i64 = conn.query_row("PRAGMA user_version", [], |r| r.get(0)).unwrap();
        assert_eq!(version, 2);
        assert!(table_names(&conn).iter().any(|t| t == "history_fts"));
    }
}
