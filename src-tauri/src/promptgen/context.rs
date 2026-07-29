use crate::domain::{Project, ProjectRule};

/// Contexto do projeto ativo, renderizado como texto para injetar no prompt.
///
/// É o que diferencia o CodeVoice de um ditado genérico: o prompt gerado já chega ao Claude
/// Code sabendo a stack, as regras e o que é proibido naquele projeto (PRODUCT-SPEC §5.6).
#[derive(Debug, Clone, Default)]
pub struct ProjectContext {
    pub name: String,
    pub path: String,
    pub description: String,
    pub stack: String,
    pub architecture: String,
    pub dev_commands: String,
    pub test_commands: String,
    pub forbidden_tech: String,
    pub database_info: String,
    pub notes: String,
    pub rules: Vec<String>,
}

impl ProjectContext {
    pub fn from_project(project: &Project, rules: &[ProjectRule]) -> Self {
        Self {
            name: project.name.clone(),
            path: project.path.clone(),
            description: project.description.clone(),
            stack: project.stack.clone(),
            architecture: project.architecture.clone(),
            dev_commands: project.dev_commands.clone(),
            test_commands: project.test_commands.clone(),
            forbidden_tech: project.forbidden_tech.clone(),
            database_info: project.database_info.clone(),
            notes: project.notes.clone(),
            rules: rules.iter().map(|r| r.rule.clone()).collect(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.name.trim().is_empty()
            && self.stack.trim().is_empty()
            && self.architecture.trim().is_empty()
            && self.rules.is_empty()
    }

    /// Bloco de texto do contexto. Campos vazios são omitidos — encher o prompt de rótulos sem
    /// valor só desperdiça atenção do modelo e polui a leitura humana.
    pub fn render(&self) -> String {
        let mut out = String::new();
        let mut push = |label: &str, value: &str| {
            let value = value.trim();
            if !value.is_empty() {
                out.push_str(&format!("- {label}: {value}\n"));
            }
        };

        push("Projeto", &self.name);
        push("Caminho", &self.path);
        push("Descrição", &self.description);
        push("Stack", &self.stack);
        push("Arquitetura", &self.architecture);
        push("Banco de dados", &self.database_info);
        push("Comandos de desenvolvimento", &self.dev_commands);
        push("Comandos de teste", &self.test_commands);
        push("Tecnologias proibidas", &self.forbidden_tech);
        push("Observações", &self.notes);

        out
    }

    /// Regras do projeto como lista. Separadas do `render()` porque vão para a seção "Regras"
    /// do prompt, não para "Contexto".
    pub fn render_rules(&self) -> String {
        self.rules
            .iter()
            .map(|r| r.trim())
            .filter(|r| !r.is_empty())
            .map(|r| format!("- {r}\n"))
            .collect()
    }

    /// Termos técnicos deste projeto, para o `initial_prompt` do Whisper e para orientar o LLM
    /// sobre grafia (ex.: "Tauri", "SQLite").
    pub fn tech_glossary(&self) -> String {
        let mut parts: Vec<&str> = Vec::new();
        if !self.stack.trim().is_empty() {
            parts.push(self.stack.trim());
        }
        if !self.database_info.trim().is_empty() {
            parts.push(self.database_info.trim());
        }
        parts.join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn project() -> Project {
        Project {
            id: 1,
            name: "CodeVoice".into(),
            path: "C:\\projects\\codevoice".into(),
            description: "App de ditado".into(),
            stack: "Tauri 2, React, Rust".into(),
            architecture: "Camadas: UI, domínio, storage".into(),
            dev_commands: "npm run tauri dev".into(),
            test_commands: "cargo test".into(),
            forbidden_tech: "jQuery".into(),
            database_info: "SQLite".into(),
            notes: "".into(),
            created_at: "2026-07-24T00:00:00Z".into(),
            updated_at: "2026-07-24T00:00:00Z".into(),
        }
    }

    fn rule(id: i32, text: &str) -> ProjectRule {
        ProjectRule {
            id,
            project_id: 1,
            rule: text.into(),
            sort_order: id,
            created_at: "2026-07-24T00:00:00Z".into(),
        }
    }

    #[test]
    fn renders_filled_fields_only() {
        let ctx = ProjectContext::from_project(&project(), &[]);
        let rendered = ctx.render();

        assert!(rendered.contains("Projeto: CodeVoice"));
        assert!(rendered.contains("Stack: Tauri 2, React, Rust"));
        assert!(rendered.contains("Tecnologias proibidas: jQuery"));
        // `notes` está vazio — o rótulo não pode aparecer.
        assert!(!rendered.contains("Observações"));
    }

    #[test]
    fn renders_rules_as_list() {
        let ctx = ProjectContext::from_project(
            &project(),
            &[
                rule(1, "Sempre rodar lint"),
                rule(2, "Nunca commitar segredo"),
            ],
        );
        let rules = ctx.render_rules();
        assert!(rules.contains("- Sempre rodar lint"));
        assert!(rules.contains("- Nunca commitar segredo"));
    }

    #[test]
    fn skips_blank_rules() {
        let ctx = ProjectContext::from_project(&project(), &[rule(1, "   ")]);
        assert!(ctx.render_rules().is_empty());
    }

    #[test]
    fn empty_context_is_detected() {
        assert!(ProjectContext::default().is_empty());
        assert!(!ProjectContext::from_project(&project(), &[]).is_empty());
    }

    #[test]
    fn glossary_joins_stack_and_database() {
        let ctx = ProjectContext::from_project(&project(), &[]);
        let glossary = ctx.tech_glossary();
        assert!(glossary.contains("Tauri 2"));
        assert!(glossary.contains("SQLite"));
    }
}
