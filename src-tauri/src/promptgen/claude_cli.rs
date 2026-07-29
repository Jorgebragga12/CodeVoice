use std::io::Write;
use std::process::{Command, Stdio};
use std::time::Duration;

use serde::Deserialize;

use super::PromptGenError;

/// Timeout do CLI. Um cold start do `claude` leva alguns segundos; 60 s cobre folgado uma
/// geração de prompt e evita travar a UI para sempre se algo pendurar.
const TIMEOUT: Duration = Duration::from_secs(60);

/// Ferramentas bloqueadas. Gerar prompt é uma tarefa de texto puro — o CLI não deve tocar em
/// arquivos, rodar comandos ou acessar a rede (SECURITY-MODEL: nada de execução implícita).
const DISALLOWED_TOOLS: &str =
    "Bash,Read,Write,Edit,MultiEdit,NotebookEdit,Glob,Grep,WebFetch,WebSearch,Task,TodoWrite";

/// Formato do `--output-format json` do Claude Code (v2.1.x). Só os campos que consumimos.
#[derive(Debug, Deserialize)]
struct CliResponse {
    #[serde(default)]
    result: String,
    #[serde(default)]
    is_error: bool,
    #[serde(default)]
    subtype: String,
}

/// `true` se o comando `claude` existe no PATH.
///
/// Só checa presença, não login: descobrir que a sessão expirou exige de fato invocar o CLI,
/// e isso é tratado como erro normal de geração (com fallback para template).
pub fn is_available() -> bool {
    Command::new("claude")
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Roda `claude -p` com o prompt vindo **por stdin**.
///
/// Segurança (SECURITY-MODEL §2, "injeção de argumentos"): o texto do usuário — transcrição
/// que pode conter qualquer coisa, inclusive `"; rm -rf /` — **nunca** entra em `argv`. Os
/// argumentos são uma lista fixa e o conteúdo variável trafega por stdin, então não há
/// interpretação de shell em momento algum (`Command` no Windows não passa por cmd.exe).
pub fn generate(full_prompt: &str) -> Result<String, PromptGenError> {
    let mut child = Command::new("claude")
        .args([
            "--print",
            "--output-format",
            "json",
            "--disallowed-tools",
            DISALLOWED_TOOLS,
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| PromptGenError::CliUnavailable(e.to_string()))?;

    {
        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| PromptGenError::Cli("não foi possível escrever no stdin".into()))?;
        stdin
            .write_all(full_prompt.as_bytes())
            .map_err(|e| PromptGenError::Cli(e.to_string()))?;
    }
    // Fecha o stdin: sem isso o CLI fica esperando mais entrada e nunca responde.
    drop(child.stdin.take());

    let output = wait_with_timeout(child)?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(PromptGenError::Cli(first_line(&stderr)));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: CliResponse = serde_json::from_str(stdout.trim())
        .map_err(|e| PromptGenError::Cli(format!("resposta inesperada do CLI: {e}")))?;

    // O CLI devolve exit 0 mesmo em erro lógico (ex.: "Not logged in"), sinalizando por
    // `is_error` — checar só o status de saída deixaria passar a mensagem de erro como se
    // fosse o prompt gerado.
    if parsed.is_error {
        return Err(PromptGenError::Cli(if parsed.result.is_empty() {
            parsed.subtype
        } else {
            parsed.result
        }));
    }

    let text = parsed.result.trim().to_string();
    if text.is_empty() {
        return Err(PromptGenError::Cli(
            "o CLI devolveu uma resposta vazia".into(),
        ));
    }
    Ok(text)
}

/// `std::process` não tem wait com timeout; esta thread auxiliar dá o efeito equivalente sem
/// puxar uma runtime async só para isso.
fn wait_with_timeout(child: std::process::Child) -> Result<std::process::Output, PromptGenError> {
    let (tx, rx) = std::sync::mpsc::channel();
    let handle = std::thread::spawn(move || {
        let result = child.wait_with_output();
        let _ = tx.send(result);
    });

    match rx.recv_timeout(TIMEOUT) {
        Ok(Ok(output)) => {
            let _ = handle.join();
            Ok(output)
        }
        Ok(Err(e)) => Err(PromptGenError::Cli(e.to_string())),
        Err(_) => Err(PromptGenError::Timeout(TIMEOUT.as_secs())),
    }
}

fn first_line(text: &str) -> String {
    text.lines()
        .map(str::trim)
        .find(|l| !l.is_empty())
        .unwrap_or("falha desconhecida ao executar o claude")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_successful_response() {
        let json =
            "{\"type\":\"result\",\"subtype\":\"success\",\"is_error\":false,\"result\":\"# Prompt feito\"}";
        let parsed: CliResponse = serde_json::from_str(json).unwrap();
        assert!(!parsed.is_error);
        assert!(parsed.result.contains("feito"));
    }

    #[test]
    fn detects_logical_error_despite_exit_zero() {
        // Formato real observado no CLI 2.1.201 quando a sessão não está logada.
        let json = r#"{"type":"result","subtype":"success","is_error":true,"result":"Not logged in · Please run /login"}"#;
        let parsed: CliResponse = serde_json::from_str(json).unwrap();
        assert!(
            parsed.is_error,
            "is_error precisa ser respeitado mesmo com exit 0"
        );
        assert!(parsed.result.contains("Not logged in"));
    }

    #[test]
    fn tolerates_missing_optional_fields() {
        let parsed: CliResponse = serde_json::from_str(r#"{"type":"result"}"#).unwrap();
        assert!(parsed.result.is_empty());
        assert!(!parsed.is_error);
    }

    #[test]
    fn first_line_picks_the_first_meaningful_line() {
        assert_eq!(first_line("\n\n  erro real  \noutra"), "erro real");
        assert_eq!(first_line("   "), "falha desconhecida ao executar o claude");
    }

    #[test]
    fn disallowed_tools_covers_every_side_effecting_tool() {
        // Se uma ferramenta de efeito colateral escapar desta lista, o CLI poderia mexer em
        // arquivos durante o que deveria ser só geração de texto.
        for tool in ["Bash", "Write", "Edit", "WebFetch", "Task"] {
            assert!(
                DISALLOWED_TOOLS.contains(tool),
                "{tool} deveria estar bloqueada"
            );
        }
    }
}
