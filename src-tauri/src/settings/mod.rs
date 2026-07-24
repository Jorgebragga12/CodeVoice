use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::storage::{DbPool, StorageError};

/// Atalho global padrão (PRODUCT-SPEC §3). Configurável — ver `RecordingSettings::hotkey`.
pub const DEFAULT_HOTKEY: &str = "CmdOrCtrl+Shift+Space";

const KEY_MICROPHONE: &str = "microphone";
const KEY_HOTKEY: &str = "hotkey";
const KEY_KEEP_AUDIO: &str = "keep_audio";
const KEY_WHISPER_MODEL: &str = "whisper_model";

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct RecordingSettings {
    /// `None` = usar o microfone padrão do sistema.
    pub microphone: Option<String>,
    pub hotkey: String,
    /// Manter o WAV após o processamento. **Desligado por padrão** (PRODUCT-SPEC §5.2).
    pub keep_audio: bool,
    /// Id do modelo Whisper selecionado (ver `transcription::model_manager::MODELS`). Padrão
    /// `large-v3-turbo` (ADR-001).
    pub whisper_model: String,
}

impl Default for RecordingSettings {
    fn default() -> Self {
        Self {
            microphone: None,
            hotkey: DEFAULT_HOTKEY.to_string(),
            keep_audio: false,
            whisper_model: crate::transcription::model_manager::DEFAULT_MODEL_ID.to_string(),
        }
    }
}

/// Configurações não sensíveis em `app_settings` (ADR-003: SQLite só no lado Rust).
///
/// Valores sensíveis (chaves de API, quando existirem) **não** vêm para cá — vão para o
/// Credential Manager do Windows via `keyring`, conforme SECURITY-MODEL §2.
#[derive(Clone)]
pub struct SettingsRepo {
    pool: DbPool,
}

impl SettingsRepo {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    fn get_raw(&self, key: &str) -> Result<Option<String>, StorageError> {
        let conn = self.pool.get()?;
        conn.query_row(
            "SELECT value FROM app_settings WHERE key = ?1",
            params![key],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(Into::into)
    }

    fn set_raw(&self, key: &str, value: &str) -> Result<(), StorageError> {
        let conn = self.pool.get()?;
        conn.execute(
            "INSERT INTO app_settings (key, value, updated_at) \
             VALUES (?1, ?2, strftime('%Y-%m-%dT%H:%M:%SZ','now')) \
             ON CONFLICT(key) DO UPDATE SET value = excluded.value, \
             updated_at = excluded.updated_at",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn get_recording_settings(&self) -> Result<RecordingSettings, StorageError> {
        let defaults = RecordingSettings::default();
        Ok(RecordingSettings {
            // String vazia no banco significa "padrão do sistema" — normalizada para None aqui
            // para que o resto do código só precise lidar com uma representação de "sem escolha".
            microphone: self.get_raw(KEY_MICROPHONE)?.filter(|s| !s.is_empty()),
            hotkey: self
                .get_raw(KEY_HOTKEY)?
                .filter(|s| !s.is_empty())
                .unwrap_or(defaults.hotkey),
            keep_audio: self
                .get_raw(KEY_KEEP_AUDIO)?
                .map(|v| v == "true")
                .unwrap_or(defaults.keep_audio),
            whisper_model: self
                .get_raw(KEY_WHISPER_MODEL)?
                .filter(|s| !s.is_empty())
                .unwrap_or(defaults.whisper_model),
        })
    }

    pub fn save_recording_settings(
        &self,
        settings: &RecordingSettings,
    ) -> Result<(), StorageError> {
        self.set_raw(KEY_MICROPHONE, settings.microphone.as_deref().unwrap_or(""))?;
        self.set_raw(KEY_HOTKEY, &settings.hotkey)?;
        self.set_raw(KEY_KEEP_AUDIO, if settings.keep_audio { "true" } else { "false" })?;
        self.set_raw(KEY_WHISPER_MODEL, &settings.whisper_model)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::test_pool;

    #[test]
    fn returns_defaults_when_nothing_was_ever_saved() {
        let db = test_pool();
        let repo = SettingsRepo::new(db.pool.clone());

        let settings = repo.get_recording_settings().unwrap();
        assert_eq!(settings.microphone, None);
        assert_eq!(settings.hotkey, DEFAULT_HOTKEY);
        assert!(!settings.keep_audio, "manter áudio tem que vir desligado");
    }

    #[test]
    fn saves_and_reads_back() {
        let db = test_pool();
        let repo = SettingsRepo::new(db.pool.clone());

        repo.save_recording_settings(&RecordingSettings {
            microphone: Some("Microfone (Realtek)".into()),
            hotkey: "CmdOrCtrl+Alt+R".into(),
            keep_audio: true,
            whisper_model: "small".into(),
        })
        .unwrap();

        let settings = repo.get_recording_settings().unwrap();
        assert_eq!(settings.microphone.as_deref(), Some("Microfone (Realtek)"));
        assert_eq!(settings.hotkey, "CmdOrCtrl+Alt+R");
        assert!(settings.keep_audio);
        assert_eq!(settings.whisper_model, "small");
    }

    #[test]
    fn saving_twice_updates_instead_of_failing_on_the_primary_key() {
        let db = test_pool();
        let repo = SettingsRepo::new(db.pool.clone());

        repo.save_recording_settings(&RecordingSettings::default()).unwrap();
        repo.save_recording_settings(&RecordingSettings {
            hotkey: "CmdOrCtrl+Alt+R".into(),
            ..Default::default()
        })
        .unwrap();

        assert_eq!(repo.get_recording_settings().unwrap().hotkey, "CmdOrCtrl+Alt+R");
    }

    #[test]
    fn empty_microphone_is_normalized_to_system_default() {
        let db = test_pool();
        let repo = SettingsRepo::new(db.pool.clone());

        repo.save_recording_settings(&RecordingSettings {
            microphone: None,
            ..Default::default()
        })
        .unwrap();

        assert_eq!(repo.get_recording_settings().unwrap().microphone, None);
    }

    #[test]
    fn an_empty_stored_hotkey_falls_back_to_the_default() {
        let db = test_pool();
        let repo = SettingsRepo::new(db.pool.clone());
        // Um atalho vazio deixaria o app sem forma de gravar por teclado.
        repo.set_raw(KEY_HOTKEY, "").unwrap();

        assert_eq!(repo.get_recording_settings().unwrap().hotkey, DEFAULT_HOTKEY);
    }
}
