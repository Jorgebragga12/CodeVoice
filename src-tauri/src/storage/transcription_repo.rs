use rusqlite::{params, OptionalExtension, Row};

use crate::domain::{NewTranscription, Transcription};

use super::{DbPool, StorageError};

#[derive(Clone)]
pub struct TranscriptionRepo {
    pool: DbPool,
}

const SELECT_COLUMNS: &str =
    "id, recording_id, text, language, engine, model_name, duration_ms, created_at";

fn row_to_transcription(row: &Row<'_>) -> rusqlite::Result<Transcription> {
    Ok(Transcription {
        id: row.get("id")?,
        recording_id: row.get("recording_id")?,
        text: row.get("text")?,
        language: row.get("language")?,
        engine: row.get("engine")?,
        model_name: row.get("model_name")?,
        duration_ms: row.get("duration_ms")?,
        created_at: row.get("created_at")?,
    })
}

impl TranscriptionRepo {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn create(&self, input: &NewTranscription) -> Result<Transcription, StorageError> {
        let conn = self.pool.get()?;
        conn.execute(
            "INSERT INTO transcriptions \
             (recording_id, text, language, engine, model_name, duration_ms) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                input.recording_id,
                input.text,
                input.language,
                input.engine,
                input.model_name,
                input.duration_ms,
            ],
        )?;
        let id = conn.last_insert_rowid() as i32;
        drop(conn);
        self.get(id)?
            .ok_or_else(|| StorageError::NotFound(format!("transcription {id}")))
    }

    pub fn get(&self, id: i32) -> Result<Option<Transcription>, StorageError> {
        let conn = self.pool.get()?;
        let sql = format!("SELECT {SELECT_COLUMNS} FROM transcriptions WHERE id = ?1");
        conn.query_row(&sql, params![id], row_to_transcription)
            .optional()
            .map_err(Into::into)
    }

    pub fn get_for_recording(
        &self,
        recording_id: i32,
    ) -> Result<Option<Transcription>, StorageError> {
        let conn = self.pool.get()?;
        let sql = format!(
            "SELECT {SELECT_COLUMNS} FROM transcriptions WHERE recording_id = ?1 \
             ORDER BY id DESC LIMIT 1"
        );
        conn.query_row(&sql, params![recording_id], row_to_transcription)
            .optional()
            .map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::NewRecording;
    use crate::storage::{test_pool, RecordingRepo};

    fn a_recording(db: &super::super::TestDb) -> i32 {
        RecordingRepo::new(db.pool.clone())
            .create(&NewRecording {
                project_id: None,
                duration_ms: 1000,
                device_name: "mic".into(),
                audio_path: None,
                audio_kept: false,
                status: "transcribed".into(),
            })
            .unwrap()
            .id
    }

    fn sample(recording_id: i32) -> NewTranscription {
        NewTranscription {
            recording_id,
            text: "criar um botão de exportar".into(),
            language: "pt".into(),
            engine: "whisper-rs".into(),
            model_name: "large-v3-turbo".into(),
            duration_ms: 1200,
        }
    }

    #[test]
    fn creates_and_reads_back() {
        let db = test_pool();
        let rec = a_recording(&db);
        let repo = TranscriptionRepo::new(db.pool.clone());

        let created = repo.create(&sample(rec)).unwrap();
        assert!(created.id > 0);
        assert_eq!(created.text, "criar um botão de exportar");
        assert_eq!(
            repo.get(created.id).unwrap().unwrap().model_name,
            "large-v3-turbo"
        );
    }

    #[test]
    fn get_for_recording_returns_the_latest() {
        let db = test_pool();
        let rec = a_recording(&db);
        let repo = TranscriptionRepo::new(db.pool.clone());

        repo.create(&sample(rec)).unwrap();
        let mut second = sample(rec);
        second.text = "segunda tentativa".into();
        repo.create(&second).unwrap();

        assert_eq!(
            repo.get_for_recording(rec).unwrap().unwrap().text,
            "segunda tentativa"
        );
    }

    #[test]
    fn deleting_recording_cascades_transcriptions() {
        let db = test_pool();
        let rec = a_recording(&db);
        let repo = TranscriptionRepo::new(db.pool.clone());
        let t = repo.create(&sample(rec)).unwrap();

        let conn = db.pool.get().unwrap();
        conn.execute("DELETE FROM recordings WHERE id = ?1", params![rec])
            .unwrap();
        drop(conn);

        // FK ON DELETE CASCADE: a transcrição some junto com a gravação.
        assert!(repo.get(t.id).unwrap().is_none());
    }
}
