use rusqlite::{params, OptionalExtension, Row};

use crate::domain::{NewRecording, Recording};

use super::{DbPool, StorageError};

#[derive(Clone)]
pub struct RecordingRepo {
    pool: DbPool,
}

const SELECT_COLUMNS: &str =
    "id, project_id, duration_ms, device_name, audio_path, audio_kept, status, created_at";

fn row_to_recording(row: &Row<'_>) -> rusqlite::Result<Recording> {
    Ok(Recording {
        id: row.get("id")?,
        project_id: row.get("project_id")?,
        duration_ms: row.get("duration_ms")?,
        device_name: row.get("device_name")?,
        audio_path: row.get("audio_path")?,
        audio_kept: row.get::<_, i64>("audio_kept")? != 0,
        status: row.get("status")?,
        created_at: row.get("created_at")?,
    })
}

impl RecordingRepo {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn create(&self, input: &NewRecording) -> Result<Recording, StorageError> {
        let conn = self.pool.get()?;
        conn.execute(
            "INSERT INTO recordings (project_id, duration_ms, device_name, audio_path, \
             audio_kept, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                input.project_id,
                input.duration_ms,
                input.device_name,
                input.audio_path,
                i64::from(input.audio_kept),
                input.status,
            ],
        )?;
        let id = conn.last_insert_rowid() as i32;
        drop(conn);
        self.get(id)?
            .ok_or_else(|| StorageError::NotFound(format!("recording {id}")))
    }

    pub fn get(&self, id: i32) -> Result<Option<Recording>, StorageError> {
        let conn = self.pool.get()?;
        let sql = format!("SELECT {SELECT_COLUMNS} FROM recordings WHERE id = ?1");
        conn.query_row(&sql, params![id], row_to_recording)
            .optional()
            .map_err(Into::into)
    }

    pub fn set_status(&self, id: i32, status: &str) -> Result<(), StorageError> {
        let conn = self.pool.get()?;
        let changed = conn.execute(
            "UPDATE recordings SET status = ?1 WHERE id = ?2",
            params![status, id],
        )?;
        if changed == 0 {
            return Err(StorageError::NotFound(format!("recording {id}")));
        }
        Ok(())
    }

    /// Esquece o caminho do áudio após ele ter sido apagado do disco.
    ///
    /// Deixar o caminho apontando para um arquivo que não existe mais faria o banco "lembrar"
    /// de uma gravação de voz já descartada — contra a promessa de privacidade do
    /// PRODUCT-SPEC §6. `audio_path` vira NULL, que é o estado esperado no schema.
    pub fn clear_audio_path(&self, id: i32) -> Result<(), StorageError> {
        let conn = self.pool.get()?;
        conn.execute(
            "UPDATE recordings SET audio_path = NULL, audio_kept = 0 WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    pub fn list_recent(&self, limit: i32) -> Result<Vec<Recording>, StorageError> {
        let conn = self.pool.get()?;
        let sql = format!(
            "SELECT {SELECT_COLUMNS} FROM recordings ORDER BY created_at DESC, id DESC LIMIT ?1"
        );
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params![limit], row_to_recording)?;
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::test_pool;

    fn sample() -> NewRecording {
        NewRecording {
            project_id: None,
            duration_ms: 4_200,
            device_name: "Microfone (Realtek)".into(),
            audio_path: Some("C:\\tmp\\rec-1.wav".into()),
            audio_kept: false,
            status: "recorded".into(),
        }
    }

    #[test]
    fn creates_and_reads_back_a_recording() {
        let db = test_pool();
        let repo = RecordingRepo::new(db.pool.clone());

        let created = repo.create(&sample()).unwrap();
        assert!(created.id > 0);
        assert_eq!(created.duration_ms, 4_200);
        assert_eq!(created.status, "recorded");
        assert!(!created.audio_kept);

        let fetched = repo.get(created.id).unwrap().unwrap();
        assert_eq!(fetched.device_name, "Microfone (Realtek)");
    }

    #[test]
    fn updates_status() {
        let db = test_pool();
        let repo = RecordingRepo::new(db.pool.clone());
        let created = repo.create(&sample()).unwrap();

        repo.set_status(created.id, "cancelled").unwrap();

        assert_eq!(repo.get(created.id).unwrap().unwrap().status, "cancelled");
    }

    #[test]
    fn rejects_a_status_outside_the_schema_check() {
        let db = test_pool();
        let repo = RecordingRepo::new(db.pool.clone());
        let created = repo.create(&sample()).unwrap();

        // O CHECK da coluna é a última linha de defesa contra status inventado no código.
        assert!(repo.set_status(created.id, "status_invalido").is_err());
    }

    #[test]
    fn clearing_audio_path_forgets_the_deleted_file() {
        let db = test_pool();
        let repo = RecordingRepo::new(db.pool.clone());
        let created = repo.create(&sample()).unwrap();
        assert!(created.audio_path.is_some());

        repo.clear_audio_path(created.id).unwrap();

        let after = repo.get(created.id).unwrap().unwrap();
        assert!(after.audio_path.is_none());
        assert!(!after.audio_kept);
    }

    #[test]
    fn set_status_on_missing_recording_returns_not_found() {
        let db = test_pool();
        let repo = RecordingRepo::new(db.pool.clone());
        assert!(matches!(
            repo.set_status(9_999, "failed"),
            Err(StorageError::NotFound(_))
        ));
    }

    #[test]
    fn lists_most_recent_first_respecting_the_limit() {
        let db = test_pool();
        let repo = RecordingRepo::new(db.pool.clone());
        for i in 0..3 {
            let mut input = sample();
            input.duration_ms = 1_000 * (i + 1);
            repo.create(&input).unwrap();
        }

        let recent = repo.list_recent(2).unwrap();
        assert_eq!(recent.len(), 2);
        // Mais recente primeiro: a última criada tem duração 3000.
        assert_eq!(recent[0].duration_ms, 3_000);
    }

    #[test]
    fn deleting_a_project_keeps_its_recordings_but_nulls_the_link() {
        let db = test_pool();
        let conn = db.pool.get().unwrap();
        conn.execute(
            "INSERT INTO projects (id, name, path) VALUES (1, 'P', 'C:\\p')",
            [],
        )
        .unwrap();
        drop(conn);

        let repo = RecordingRepo::new(db.pool.clone());
        let mut input = sample();
        input.project_id = Some(1);
        let created = repo.create(&input).unwrap();

        let conn = db.pool.get().unwrap();
        conn.execute("DELETE FROM projects WHERE id = 1", []).unwrap();
        drop(conn);

        let after = repo.get(created.id).unwrap().unwrap();
        assert!(after.project_id.is_none(), "ON DELETE SET NULL deveria preservar a gravação");
    }
}
