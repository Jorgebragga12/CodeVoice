use crate::domain::PromptMode;

/// Seções possíveis de um prompt técnico (PRODUCT-SPEC §5.4). Cada modo declara quais usa —
/// um "prompt rápido" não precisa de plano/validações, mas uma "nova funcionalidade" sim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    Objetivo,
    ContextoProjeto,
    RequisitosFuncionais,
    RequisitosTecnicos,
    Regras,
    Restricoes,
    ArquivosRelacionados,
    PlanoEsperado,
    Validacoes,
    CriteriosAceite,
    FormatoRelatorio,
}

impl Section {
    /// Título renderizado no prompt final.
    pub fn title(self) -> &'static str {
        match self {
            Self::Objetivo => "Objetivo",
            Self::ContextoProjeto => "Contexto do projeto",
            Self::RequisitosFuncionais => "Requisitos funcionais",
            Self::RequisitosTecnicos => "Requisitos técnicos",
            Self::Regras => "Regras",
            Self::Restricoes => "Restrições",
            Self::ArquivosRelacionados => "Arquivos possivelmente relacionados",
            Self::PlanoEsperado => "Plano esperado",
            Self::Validacoes => "Validações",
            Self::CriteriosAceite => "Critérios de aceitação",
            Self::FormatoRelatorio => "Formato do relatório final",
        }
    }
}

/// Metadados de um modo: como aparece na UI e como o prompt é montado.
pub struct ModeSpec {
    pub mode: PromptMode,
    /// Rótulo curto para a UI (PT).
    pub label: &'static str,
    /// Explicação de uma linha, mostrada na UI.
    pub description: &'static str,
    /// Seções que este modo emite (na ordem).
    pub sections: &'static [Section],
    /// Instrução dada ao LLM sobre o papel/estilo deste modo. Também vira o cabeçalho
    /// determinístico do `TemplateGenerator` quando não há LLM disponível.
    pub instruction: &'static str,
}

use Section::*;

/// Seções completas do prompt técnico — a base dos modos especializados.
const FULL: &[Section] = &[
    Objetivo,
    ContextoProjeto,
    RequisitosFuncionais,
    RequisitosTecnicos,
    Regras,
    Restricoes,
    ArquivosRelacionados,
    PlanoEsperado,
    Validacoes,
    CriteriosAceite,
    FormatoRelatorio,
];

pub const MODES: &[ModeSpec] = &[
    ModeSpec {
        mode: PromptMode::CleanTranscript,
        label: "Transcrição limpa",
        description: "Só remove hesitações e pontua. Não reestrutura nem inventa nada.",
        // Sem seções: a saída é o texto corrido, não um prompt estruturado.
        sections: &[],
        instruction: "Limpe a transcrição abaixo: remova hesitações (\"é\", \"ãã\", \
            repetições involuntárias), corrija pontuação e capitalização. NÃO reescreva o \
            conteúdo, NÃO resuma, NÃO adicione informação. Preserve exatamente nomes de \
            arquivos, comandos e tecnologias. Devolva apenas o texto limpo.",
    },
    ModeSpec {
        mode: PromptMode::Quick,
        label: "Prompt rápido",
        description: "Pedido direto em 1–3 parágrafos, sem seções.",
        sections: &[Objetivo, ContextoProjeto],
        instruction: "Transforme a fala abaixo em um pedido direto e claro para um agente de \
            programação, em 1 a 3 parágrafos. Sem seções, sem listas longas. Vá direto ao que \
            precisa ser feito.",
    },
    ModeSpec {
        mode: PromptMode::Technical,
        label: "Prompt técnico",
        description: "Estrutura completa: objetivo, requisitos, plano, critérios de aceite.",
        sections: FULL,
        instruction: "Transforme a fala abaixo em um prompt técnico estruturado para um agente \
            de programação. Seja específico e acionável.",
    },
    ModeSpec {
        mode: PromptMode::NewFeature,
        label: "Nova funcionalidade",
        description: "Implementar algo novo, com requisitos e critérios de aceite.",
        sections: FULL,
        instruction: "Transforme a fala abaixo em um prompt para implementar uma NOVA \
            FUNCIONALIDADE. Detalhe o comportamento esperado, os casos de borda e como validar \
            que ficou pronto.",
    },
    ModeSpec {
        mode: PromptMode::BugFix,
        label: "Correção de erro",
        description: "Investigar e corrigir um bug, com reprodução e validação.",
        sections: &[
            Objetivo,
            ContextoProjeto,
            RequisitosTecnicos,
            Regras,
            ArquivosRelacionados,
            PlanoEsperado,
            Validacoes,
            CriteriosAceite,
            FormatoRelatorio,
        ],
        instruction: "Transforme a fala abaixo em um prompt para CORRIGIR UM ERRO. Destaque o \
            sintoma observado, como reproduzir, e exija que a causa raiz seja encontrada antes \
            da correção (nada de remendo que só esconde o sintoma).",
    },
    ModeSpec {
        mode: PromptMode::Refactor,
        label: "Refatoração",
        description: "Melhorar código sem mudar comportamento.",
        sections: &[
            Objetivo,
            ContextoProjeto,
            RequisitosTecnicos,
            Regras,
            Restricoes,
            ArquivosRelacionados,
            PlanoEsperado,
            Validacoes,
            CriteriosAceite,
            FormatoRelatorio,
        ],
        instruction: "Transforme a fala abaixo em um prompt de REFATORAÇÃO. Deixe explícito que \
            o comportamento externo NÃO deve mudar e que os testes existentes precisam \
            continuar passando.",
    },
    ModeSpec {
        mode: PromptMode::Planning,
        label: "Planejamento",
        description: "Planejar antes de codar: análise, riscos, fases.",
        sections: &[
            Objetivo,
            ContextoProjeto,
            RequisitosFuncionais,
            RequisitosTecnicos,
            Restricoes,
            PlanoEsperado,
            CriteriosAceite,
            FormatoRelatorio,
        ],
        instruction: "Transforme a fala abaixo em um prompt de PLANEJAMENTO. O agente deve \
            analisar, levantar riscos e dúvidas, e propor um plano em fases — SEM escrever \
            código ainda.",
    },
    ModeSpec {
        mode: PromptMode::CodeReview,
        label: "Revisão de código",
        description: "Revisar código existente em busca de problemas.",
        sections: &[
            Objetivo,
            ContextoProjeto,
            Regras,
            ArquivosRelacionados,
            Validacoes,
            FormatoRelatorio,
        ],
        instruction: "Transforme a fala abaixo em um prompt de REVISÃO DE CÓDIGO. O agente deve \
            procurar bugs reais, riscos de segurança e problemas de manutenção — priorizando \
            por severidade, sem apontar preferência de estilo como se fosse defeito.",
    },
    ModeSpec {
        mode: PromptMode::UiCreation,
        label: "Criação de interface",
        description: "Construir tela/componente de UI.",
        sections: &[
            Objetivo,
            ContextoProjeto,
            RequisitosFuncionais,
            RequisitosTecnicos,
            Regras,
            Restricoes,
            ArquivosRelacionados,
            CriteriosAceite,
            FormatoRelatorio,
        ],
        instruction: "Transforme a fala abaixo em um prompt para CRIAR INTERFACE. Descreva \
            layout, estados (carregando/vazio/erro), acessibilidade e comportamento \
            responsivo.",
    },
    ModeSpec {
        mode: PromptMode::DbChange,
        label: "Alteração de banco de dados",
        description: "Mudar esquema/dados com migration e rollback.",
        sections: &[
            Objetivo,
            ContextoProjeto,
            RequisitosTecnicos,
            Regras,
            Restricoes,
            ArquivosRelacionados,
            PlanoEsperado,
            Validacoes,
            CriteriosAceite,
            FormatoRelatorio,
        ],
        instruction: "Transforme a fala abaixo em um prompt de ALTERAÇÃO DE BANCO DE DADOS. \
            Exija migration versionada, plano de rollback e cuidado explícito com dados \
            existentes (nada de operação destrutiva sem confirmação).",
    },
];

pub fn spec_for(mode: PromptMode) -> &'static ModeSpec {
    MODES
        .iter()
        .find(|s| s.mode == mode)
        .expect("todo PromptMode precisa ter um ModeSpec")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Todos os 10 modos do PRODUCT-SPEC §5.4 precisam existir — se alguém adicionar uma
    /// variante ao enum sem spec, `spec_for` entraria em pânico em runtime.
    #[test]
    fn every_prompt_mode_has_a_spec() {
        let all = [
            PromptMode::CleanTranscript,
            PromptMode::Quick,
            PromptMode::Technical,
            PromptMode::NewFeature,
            PromptMode::BugFix,
            PromptMode::Refactor,
            PromptMode::Planning,
            PromptMode::CodeReview,
            PromptMode::UiCreation,
            PromptMode::DbChange,
        ];
        assert_eq!(MODES.len(), 10, "esperado exatamente os 10 modos do spec");
        for mode in all {
            let spec = spec_for(mode);
            assert_eq!(spec.mode, mode);
            assert!(!spec.label.is_empty());
            assert!(!spec.instruction.is_empty());
        }
    }

    #[test]
    fn technical_mode_has_every_section() {
        assert_eq!(spec_for(PromptMode::Technical).sections.len(), 11);
    }

    #[test]
    fn clean_transcript_has_no_sections() {
        // É texto corrido, não prompt estruturado.
        assert!(spec_for(PromptMode::CleanTranscript).sections.is_empty());
    }

    #[test]
    fn mode_labels_are_unique() {
        for (i, a) in MODES.iter().enumerate() {
            for b in &MODES[i + 1..] {
                assert_ne!(a.label, b.label, "rótulo duplicado: {}", a.label);
            }
        }
    }
}
