use rusqlite::{params, Row};

use crate::domain::{NewFlow, PromptHistoryEntry};

use super::{DbPool, StorageError};

#[derive(Clone)]
pub struct HistoryRepo {
    pool: DbPool,
}

fn row_to_entry(row: &Row<'_>) -> rusqlite::Result<PromptHistoryEntry> {
    Ok(PromptHistoryEntry {
        id: row.get("id")?,
        project_id: row.get("project_id")?,
        mode: row.get("mode")?,
        favorite: row.get::<_, i64>("favorite")? != 0,
        audio_duration_ms: row.get("audio_duration_ms")?,
        status: row.get("status")?,
        created_at: row.get("created_at")?,
    })
}

impl HistoryRepo {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Grava o fluxo inteiro (recording → transcription → generated_prompt → prompt_history
    /// → history_fts) em uma única transação. Ver DATABASE-SCHEMA.md §5.
    pub fn save_flow(&self, flow: &NewFlow) -> Result<i64, StorageError> {
        let mut conn = self.pool.get()?;
        let tx = conn.transaction()?;

        tx.execute(
            "INSERT INTO recordings (project_id, duration_ms, device_name, status) \
             VALUES (?1, ?2, ?3, 'transcribed')",
            params![flow.project_id, flow.duration_ms, flow.device_name],
        )?;
        let recording_id = tx.last_insert_rowid();

        tx.execute(
            "INSERT INTO transcriptions \
             (recording_id, text, language, engine, model_name, duration_ms) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                recording_id,
                flow.transcript_text,
                flow.transcript_language,
                flow.engine,
                flow.model_name,
                flow.transcribe_duration_ms,
            ],
        )?;
        let transcription_id = tx.last_insert_rowid();

        tx.execute(
            "INSERT INTO generated_prompts \
             (transcription_id, project_id, mode, generator, content, original_content) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?5)",
            params![
                transcription_id,
                flow.project_id,
                flow.mode.as_db_str(),
                flow.generator.as_db_str(),
                flow.prompt_content,
            ],
        )?;
        let generated_prompt_id = tx.last_insert_rowid();

        tx.execute(
            "INSERT INTO prompt_history \
             (project_id, recording_id, transcription_id, generated_prompt_id, mode, \
              audio_duration_ms, status) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'completed')",
            params![
                flow.project_id,
                recording_id,
                transcription_id,
                generated_prompt_id,
                flow.mode.as_db_str(),
                flow.duration_ms,
            ],
        )?;
        let history_id = tx.last_insert_rowid();

        tx.execute(
            "INSERT INTO history_fts (transcript, prompt, history_id) VALUES (?1, ?2, ?3)",
            params![flow.transcript_text, flow.prompt_content, history_id],
        )?;

        tx.commit()?;
        Ok(history_id)
    }

    /// Busca full-text (transcrição + prompt) via FTS5. `query` é passado direto pro `MATCH`
    /// do FTS5 — a Fase 8 deve sanitizar caracteres especiais antes de expor isso numa tela real.
    pub fn search(&self, query: &str) -> Result<Vec<PromptHistoryEntry>, StorageError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT ph.id, ph.project_id, ph.mode, ph.favorite, ph.audio_duration_ms, \
             ph.status, ph.created_at \
             FROM prompt_history ph \
             JOIN history_fts f ON f.history_id = ph.id \
             WHERE history_fts MATCH ?1 \
             ORDER BY ph.created_at DESC",
        )?;
        let rows = stmt.query_map(params![query], row_to_entry)?;
        rows.collect::<rusqlite::Result<Vec<_>>>().map_err(Into::into)
    }

    pub fn list_for_project(&self, project_id: i64) -> Result<Vec<PromptHistoryEntry>, StorageError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT id, project_id, mode, favorite, audio_duration_ms, status, created_at \
             FROM prompt_history WHERE project_id = ?1 ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map(params![project_id], row_to_entry)?;
        rows.collect::<rusqlite::Result<Vec<_>>>().map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{PromptMode, PromptSource};
    use crate::storage::test_pool;

    fn sample_flow() -> NewFlow {
        NewFlow {
            project_id: None,
            duration_ms: 12_000,
            device_name: "Microfone padrão".into(),
            transcript_text: "criar um botão de exportar relatório em PDF".into(),
            transcript_language: "pt".into(),
            engine: "whisper-rs".into(),
            model_name: "large-v3-turbo".into(),
            transcribe_duration_ms: 800,
            mode: PromptMode::NewFeature,
            generator: PromptSource::Template,
            prompt_content: "Objetivo: adicionar exportação de relatório em PDF.".into(),
        }
    }

    #[test]
    fn save_flow_persists_all_related_rows_atomically() {
        let db = test_pool();
        let repo = HistoryRepo::new(db.pool.clone());

        let history_id = repo.save_flow(&sample_flow()).unwrap();
        assert!(history_id > 0);

        let conn = db.pool.get().unwrap();
        let recordings: i64 = conn.query_row("SELECT COUNT(*) FROM recordings", [], |r| r.get(0)).unwrap();
        let transcriptions: i64 = conn
            .query_row("SELECT COUNT(*) FROM transcriptions", [], |r| r.get(0))
            .unwrap();
        let prompts: i64 = conn
            .query_row("SELECT COUNT(*) FROM generated_prompts", [], |r| r.get(0))
            .unwrap();
        let history: i64 = conn.query_row("SELECT COUNT(*) FROM prompt_history", [], |r| r.get(0)).unwrap();

        assert_eq!((recordings, transcriptions, prompts, history), (1, 1, 1, 1));
    }

    #[test]
    fn search_finds_by_transcript_and_by_prompt_content() {
        let db = test_pool();
        let repo = HistoryRepo::new(db.pool.clone());
        repo.save_flow(&sample_flow()).unwrap();

        let by_transcript = repo.search("relatório").unwrap();
        assert_eq!(by_transcript.len(), 1);

        let by_prompt = repo.search("exportação").unwrap();
        assert_eq!(by_prompt.len(), 1);

        let no_match = repo.search("inexistente").unwrap();
        assert!(no_match.is_empty());
    }

    #[test]
    fn search_ignores_accents() {
        let db = test_pool();
        let repo = HistoryRepo::new(db.pool.clone());
        repo.save_flow(&sample_flow()).unwrap();

        // "relatorio" sem acento deve encontrar "relatório" (tokenizer remove_diacritics).
        let results = repo.search("relatorio").unwrap();
        assert_eq!(results.len(), 1);
    }
}
