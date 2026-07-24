import { useEffect, useState } from "react";
import { commands, type AudioDevice, type RecordingSettings } from "../../ipc/bindings";

export function Settings() {
  const [devices, setDevices] = useState<AudioDevice[]>([]);
  const [settings, setSettings] = useState<RecordingSettings | null>(null);
  const [hotkeyDraft, setHotkeyDraft] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);

  useEffect(() => {
    async function load() {
      const [devicesResult, settingsResult] = await Promise.all([
        commands.listAudioDevices(),
        commands.getRecordingSettings(),
      ]);

      if (devicesResult.status === "ok") {
        setDevices(devicesResult.data);
      } else {
        setError(devicesResult.error);
      }

      if (settingsResult.status === "ok") {
        setSettings(settingsResult.data);
        setHotkeyDraft(settingsResult.data.hotkey);
      } else {
        setError(settingsResult.error);
      }
    }
    void load();
  }, []);

  async function persist(next: RecordingSettings) {
    setSettings(next);
    const result = await commands.saveRecordingSettings(next);
    if (result.status === "error") setError(result.error);
  }

  async function applyHotkey() {
    setError(null);
    setNotice(null);
    const result = await commands.updateHotkey(hotkeyDraft);
    if (result.status === "error") {
      // O backend restaura o atalho anterior quando o novo está em conflito, então a UI volta
      // a mostrar o que de fato está valendo.
      setError(result.error);
      if (settings) setHotkeyDraft(settings.hotkey);
      return;
    }
    setNotice("Atalho atualizado.");
    if (settings) setSettings({ ...settings, hotkey: hotkeyDraft });
  }

  if (!settings) {
    return <p className="text-sm text-zinc-500">Carregando configurações…</p>;
  }

  return (
    <div className="mx-auto flex max-w-lg flex-col gap-6">
      <section className="flex flex-col gap-2">
        <label htmlFor="microphone" className="text-sm font-medium text-zinc-300">
          Microfone
        </label>
        <select
          id="microphone"
          value={settings.microphone ?? ""}
          onChange={(e) => void persist({ ...settings, microphone: e.target.value || null })}
          className="rounded border border-zinc-700 bg-zinc-900 px-2 py-1.5 text-sm text-zinc-100"
        >
          <option value="">Padrão do sistema</option>
          {devices.map((device) => (
            <option key={device.name} value={device.name}>
              {device.name}
              {device.is_default ? " (padrão)" : ""}
            </option>
          ))}
        </select>
        {devices.length === 0 && (
          <p className="text-xs text-amber-400">Nenhum microfone detectado.</p>
        )}
      </section>

      <section className="flex flex-col gap-2">
        <label htmlFor="hotkey" className="text-sm font-medium text-zinc-300">
          Atalho global
        </label>
        <div className="flex gap-2">
          <input
            id="hotkey"
            value={hotkeyDraft}
            onChange={(e) => setHotkeyDraft(e.target.value)}
            placeholder="CmdOrCtrl+Shift+Space"
            className="flex-1 rounded border border-zinc-700 bg-zinc-900 px-2 py-1.5 font-mono text-sm text-zinc-100"
          />
          <button
            type="button"
            onClick={() => void applyHotkey()}
            className="rounded bg-zinc-800 px-3 py-1.5 text-sm text-zinc-100 hover:bg-zinc-700"
          >
            Aplicar
          </button>
        </div>
        <p className="text-xs text-zinc-500">
          Pressione o atalho para começar a gravar e novamente para encerrar.
        </p>
      </section>

      <section className="flex items-start gap-2">
        <input
          id="keep-audio"
          type="checkbox"
          checked={settings.keep_audio}
          onChange={(e) => void persist({ ...settings, keep_audio: e.target.checked })}
          className="mt-1"
        />
        <label htmlFor="keep-audio" className="text-sm text-zinc-300">
          Manter o arquivo de áudio após o processamento
          <span className="block text-xs text-zinc-500">
            Desativado por padrão: o áudio é apagado assim que vira texto.
          </span>
        </label>
      </section>

      {error && <p className="text-sm text-red-400">{error}</p>}
      {notice && <p className="text-sm text-emerald-400">{notice}</p>}
    </div>
  );
}
