use once_cell::sync::Lazy;
use regex::{Regex, RegexBuilder};

const REDACTED: &str = "[REDACTED]";

static ANTHROPIC_KEY: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"sk-ant-[A-Za-z0-9_-]{20,}").expect("invalid regex"));

static GITHUB_TOKEN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"gh[pousr]_[A-Za-z0-9]{20,}|github_pat_[A-Za-z0-9_]{20,}").expect("invalid regex")
});

static AWS_ACCESS_KEY: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"AKIA[0-9A-Z]{16}").expect("invalid regex"));

static BEARER_TOKEN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)bearer\s+[A-Za-z0-9\-_.=]+").expect("invalid regex"));

static KEY_VALUE_SECRET: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?i)(password|token|secret|api[_-]?key)\s*[:=]\s*['"]?[^\s'",;]+"#)
        .expect("invalid regex")
});

static JWT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"eyJ[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+").expect("invalid regex")
});

static PEM_BLOCK: Lazy<Regex> = Lazy::new(|| {
    RegexBuilder::new(r"-----BEGIN [A-Z ]+-----.*?-----END [A-Z ]+-----")
        .dot_matches_new_line(true)
        .build()
        .expect("invalid regex")
});

/// Redige segredos conhecidos (chaves de API, tokens, JWTs, blocos PEM) de uma string
/// antes que ela seja escrita em log. Ver SECURITY-MODEL.md §4.
pub fn redact(input: &str) -> String {
    let out = ANTHROPIC_KEY.replace_all(input, REDACTED);
    let out = GITHUB_TOKEN.replace_all(&out, REDACTED);
    let out = AWS_ACCESS_KEY.replace_all(&out, REDACTED);
    let out = PEM_BLOCK.replace_all(&out, REDACTED);
    let out = JWT.replace_all(&out, REDACTED);
    let out = BEARER_TOKEN.replace_all(&out, REDACTED);
    let out = KEY_VALUE_SECRET.replace_all(&out, REDACTED);
    out.into_owned()
}

#[cfg(test)]
mod tests {
    use super::redact;

    #[test]
    fn redacts_anthropic_api_key() {
        let out = redact("chave: sk-ant-api03-abcdefghijklmnopqrstuvwxyz0123456789");
        assert!(!out.contains("sk-ant-api03"));
        assert!(out.contains("[REDACTED]"));
    }

    #[test]
    fn redacts_github_tokens() {
        let out = redact("token ghp_abcdefghijklmnopqrstuvwxyzABCDEFGH usado no clone");
        assert!(!out.contains("ghp_abcdefghijklmnopqrstuvwxyzABCDEFGH"));
        assert!(out.contains("[REDACTED]"));

        let out2 = redact("github_pat_11ABCDEFG0abcdefghijklmnopqrstuvwxyz");
        assert!(!out2.contains("11ABCDEFG0abcdefghijklmnopqrstuvwxyz"));
    }

    #[test]
    fn redacts_aws_access_key() {
        let out = redact("AWS_ACCESS_KEY_ID=AKIAABCDEFGHIJKLMNOP");
        assert!(!out.contains("AKIAABCDEFGHIJKLMNOP"));
        assert!(out.contains("[REDACTED]"));
    }

    #[test]
    fn redacts_bearer_token() {
        let out = redact("Authorization: Bearer abc123.def456-ghi789");
        assert!(!out.contains("abc123.def456-ghi789"));
        assert!(out.contains("[REDACTED]"));
    }

    #[test]
    fn redacts_password_and_token_assignments() {
        let out = redact(r#"password="hunter2xyz" e token=abcDEF123456"#);
        assert!(!out.contains("hunter2xyz"));
        assert!(!out.contains("abcDEF123456"));
    }

    #[test]
    fn redacts_jwt() {
        let jwt = "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.dozjgNryP4J3jVmNHl0w5N_XgL0n3I9PlFUP0THsR8U";
        let out = redact(&format!("sessão: {jwt}"));
        assert!(!out.contains(jwt));
        assert!(out.contains("[REDACTED]"));
    }

    #[test]
    fn redacts_pem_private_key_block() {
        let pem =
            "-----BEGIN RSA PRIVATE KEY-----\nMIIBOgIBAAJBAK...\n-----END RSA PRIVATE KEY-----";
        let out = redact(pem);
        assert!(!out.contains("MIIBOgIBAAJBAK"));
        assert!(out.contains("[REDACTED]"));
    }

    #[test]
    fn leaves_ordinary_text_untouched() {
        let msg = "Transcrição gerada com sucesso para o projeto CodeVoice em 42ms.";
        assert_eq!(redact(msg), msg);
    }
}
