use rusqlite::{params, OptionalExtension, Row};

use crate::domain::{GeneratedPrompt, NewGeneratedPrompt};

use super::{DbPool, StorageError};

#[derive(Clone)]
pub struct PromptRepo {
    pool: DbPool,
}

const SELECT_COLUMNS: &str = "id, transcription_id, project_id, mode, generator, content, \
     original_content, created_at, updated_at";

fn row_to_prompt(row: &Row<'_>) -> rusqlite::Result<GeneratedPrompt> {
    Ok(GeneratedPrompt {
        id: row.get("id")?,
        transcription_id: row.get("transcription_id")?,
        project_id: row.get("project_id")?,
        mode: row.get("mode")?,
        generator: row.get("generator")?,
        content: row.get("content")?,
        original_content: row.get("original_content")?,
        created_at: row.get("created_at")?,
        updated_at: row.get("updated_at")?,
    })
}

impl PromptRepo {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Grava o prompt gerado e o item de histórico correspondente **na mesma transação** — um
    /// prompt sem entrada no histórico ficaria invisível na tela de Histórico (Fase 8).
    pub fn create_with_history(
        &self,
        input: &NewGeneratedPrompt,
        audio_duration_ms: i32,
        transcript_text: &str,
        recording_id: Option<i32>,
    ) -> Result<GeneratedPrompt, StorageError> {
        let mut conn = self.pool.get()?;
        let tx = conn.transaction()?;

        tx.execute(
            "INSERT INTO generated_prompts \
             (transcription_id, project_id, mode, generator, content, original_content) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?5)",
            params![
                input.transcription_id,
                input.project_id,
                input.mode,
                input.generator,
                input.content,
            ],
        )?;
        let prompt_id = tx.last_insert_rowid() as i32;

        tx.execute(
            "INSERT INTO prompt_history \
             (project_id, recording_id, transcription_id, generated_prompt_id, mode, \
              audio_duration_ms, status) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'completed')",
            params![
                input.project_id,
                recording_id,
                input.transcription_id,
                prompt_id,
                input.mode,
                audio_duration_ms,
            ],
        )?;
        let history_id = tx.last_insert_rowid() as i32;

        // Índice de busca do histórico (Fase 8).
        tx.execute(
            "INSERT INTO history_fts (transcript, prompt, history_id) VALUES (?1, ?2, ?3)",
            params![transcript_text, input.content, history_id],
        )?;

        tx.commit()?;

        self.get(prompt_id)?
            .ok_or_else(|| StorageError::NotFound(format!("generated_prompt {prompt_id}")))
    }

    pub fn get(&self, id: i32) -> Result<Option<GeneratedPrompt>, StorageError> {
        let conn = self.pool.get()?;
        let sql = format!("SELECT {SELECT_COLUMNS} FROM generated_prompts WHERE id = ?1");
        conn.query_row(&sql, params![id], row_to_prompt)
            .optional()
            .map_err(Into::into)
    }

    /// Atualiza o conteúdo editado pelo usuário. `original_content` permanece intocado — é o
    /// que permite comparar/reverter para o texto como foi gerado (Fase 7).
    pub fn update_content(&self, id: i32, content: &str) -> Result<GeneratedPrompt, StorageError> {
        let conn = self.pool.get()?;
        let changed = conn.execute(
            "UPDATE generated_prompts SET content = ?1, \
             updated_at = strftime('%Y-%m-%dT%H:%M:%SZ','now') WHERE id = ?2",
            params![content, id],
        )?;
        if changed == 0 {
            return Err(StorageError::NotFound(format!("generated_prompt {id}")));
        }
        drop(conn);
        self.get(id)?
            .ok_or_else(|| StorageError::NotFound(format!("generated_prompt {id}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{NewRecording, NewTranscription};
    use crate::storage::{test_pool, RecordingRepo, TranscriptionRepo};

    fn seed(db: &super::super::TestDb) -> (i32, i32) {
        let rec = RecordingRepo::new(db.pool.clone())
            .create(&NewRecording {
                project_id: None,
                duration_ms: 5000,
                device_name: "mic".into(),
                audio_path: None,
                audio_kept: false,
                status: "transcribed".into(),
            })
            .unwrap();
        let tr = TranscriptionRepo::new(db.pool.clone())
            .create(&NewTranscription {
                recording_id: rec.id,
                text: "criar tela de login".into(),
                language: "pt".into(),
                engine: "whisper-rs".into(),
                model_name: "large-v3-turbo".into(),
                duration_ms: 900,
            })
            .unwrap();
        (rec.id, tr.id)
    }

    fn sample(transcription_id: i32) -> NewGeneratedPrompt {
        NewGeneratedPrompt {
            transcription_id: Some(transcription_id),
            project_id: None,
            mode: "technical".into(),
            generator: "template".into(),
            content: "# Prompt técnico\n\n## Objetivo\ncriar tela de login".into(),
        }
    }

    #[test]
    fn creates_prompt_and_history_together() {
        let db = test_pool();
        let (rec, tr) = seed(&db);
        let repo = PromptRepo::new(db.pool.clone());

        let created = repo
            .create_with_history(&sample(tr), 5000, "criar tela de login", Some(rec))
            .unwrap();
        assert!(created.id > 0);
        assert_eq!(created.mode, "technical");
        // original_content é preenchido com o mesmo texto na criação.
        assert_eq!(created.content, created.original_content);

        let conn = db.pool.get().unwrap();
        let history: i64 = conn
            .query_row("SELECT COUNT(*) FROM prompt_history", [], |r| r.get(0))
            .unwrap();
        let fts: i64 = conn
            .query_row("SELECT COUNT(*) FROM history_fts", [], |r| r.get(0))
            .unwrap();
        assert_eq!(
            (history, fts),
            (1, 1),
            "histórico e índice de busca devem existir"
        );
    }

    #[test]
    fn history_row_is_searchable_by_prompt_text() {
        let db = test_pool();
        let (rec, tr) = seed(&db);
        let repo = PromptRepo::new(db.pool.clone());
        repo.create_with_history(&sample(tr), 5000, "criar tela de login", Some(rec))
            .unwrap();

        let conn = db.pool.get().unwrap();
        let found: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM history_fts WHERE history_fts MATCH 'login'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(found, 1);
    }

    #[test]
    fn editing_keeps_the_original_untouched() {
        let db = test_pool();
        let (rec, tr) = seed(&db);
        let repo = PromptRepo::new(db.pool.clone());
        let created = repo
            .create_with_history(&sample(tr), 5000, "criar tela de login", Some(rec))
            .unwrap();

        let updated = repo
            .update_content(created.id, "# Editado pelo usuário")
            .unwrap();

        assert_eq!(updated.content, "# Editado pelo usuário");
        assert_eq!(
            updated.original_content, created.original_content,
            "o texto original precisa sobreviver à edição"
        );
    }

    #[test]
    fn update_of_missing_prompt_is_not_found() {
        let db = test_pool();
        let repo = PromptRepo::new(db.pool.clone());
        assert!(matches!(
            repo.update_content(9999, "x"),
            Err(StorageError::NotFound(_))
        ));
    }

    #[test]
    fn rejects_mode_outside_the_schema_check() {
        let db = test_pool();
        let (rec, tr) = seed(&db);
        let repo = PromptRepo::new(db.pool.clone());
        let mut bad = sample(tr);
        bad.mode = "modo_inventado".into();

        // O CHECK da coluna é a última linha de defesa contra modo inválido vindo do código.
        assert!(repo
            .create_with_history(&bad, 5000, "texto", Some(rec))
            .is_err());
    }
}
