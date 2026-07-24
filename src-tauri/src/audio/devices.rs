use cpal::traits::{DeviceTrait, HostTrait};
use serde::{Deserialize, Serialize};
use specta::Type;

use super::error::AudioError;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct AudioDevice {
    pub name: String,
    /// `true` para o microfone padrão do sistema — a UI marca ele como "(padrão)" e é o que
    /// usamos quando o usuário nunca escolheu nada.
    pub is_default: bool,
}

/// Lista os microfones disponíveis. Devices cujo nome o driver não consegue reportar são
/// pulados: sem nome não há como o usuário escolher nem como reencontrá-los depois.
pub fn list_input_devices() -> Result<Vec<AudioDevice>, AudioError> {
    let host = cpal::default_host();
    let default_name = host.default_input_device().and_then(|d| d.name().ok());

    let devices = host
        .input_devices()
        .map_err(|e| AudioError::Stream(e.to_string()))?;

    Ok(devices
        .filter_map(|device| device.name().ok())
        .map(|name| AudioDevice {
            is_default: Some(&name) == default_name.as_ref(),
            name,
        })
        .collect())
}

/// Resolve o microfone a usar: o de nome `preferred`, ou o padrão do sistema quando `preferred`
/// é `None`.
///
/// Um microfone escolhido que sumiu (desconectado desde a última vez) devolve erro em vez de
/// cair silenciosamente no padrão — gravar com o microfone errado sem avisar seria pior do que
/// falhar, já que o usuário só descobriria ao ouvir/transcrever o resultado.
pub fn resolve_input_device(preferred: Option<&str>) -> Result<cpal::Device, AudioError> {
    let host = cpal::default_host();

    match preferred {
        Some(wanted) => host
            .input_devices()
            .map_err(|e| AudioError::Stream(e.to_string()))?
            .find(|device| device.name().is_ok_and(|name| name == wanted))
            .ok_or_else(|| AudioError::DeviceNotFound(wanted.to_string())),
        None => host.default_input_device().ok_or(AudioError::NoInputDevice),
    }
}
