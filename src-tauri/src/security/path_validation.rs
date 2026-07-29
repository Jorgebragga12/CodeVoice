//! Validação e canonicalização de caminhos de projeto (SECURITY-MODEL.md §3).
//!
//! Toda entrada de caminho vinda do usuário (campo `path` de `NewProject`, e qualquer
//! diretório/arquivo tocado pelo scanner de importação em `projects::scanner`) passa por aqui
//! antes de qualquer chamada de filesystem que possa ter efeito observável.

use std::path::{Component, Path, PathBuf};

use thiserror::Error;

/// Nomes de dispositivo reservados pelo Windows — não podem ser usados como nome de
/// arquivo/diretório (comparação ignora extensão e maiúsculas/minúsculas).
const RESERVED_DEVICE_NAMES: &[&str] = &[
    "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
    "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
];

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum PathValidationError {
    #[error("caminho vazio")]
    Empty,
    #[error("caminho contém um segmento de travessia de diretório (\"..\")")]
    Traversal,
    #[error("nome de dispositivo Windows reservado: {0}")]
    ReservedDeviceName(String),
    #[error("caminhos de rede (UNC) ou com prefixo \\\\?\\ não são suportados: {0}")]
    UnsupportedUnc(String),
    #[error("caminho não encontrado: {0}")]
    NotFound(String),
    #[error("caminho não é um diretório: {0}")]
    NotADirectory(String),
    #[error("falha ao resolver caminho: {0}")]
    Canonicalize(String),
    #[error("caminho resolvido está fora do diretório esperado")]
    EscapesRoot,
}

/// Caminhos UNC (`\\servidor\compartilhamento\...`) e o prefixo verbatim do Windows
/// (`\\?\...`) começam ambos com `\\` — nenhum dos dois é suportado (SECURITY-MODEL.md §2).
fn is_unc_or_verbatim(raw: &str) -> bool {
    raw.starts_with(r"\\") || raw.starts_with("//")
}

/// Devolve o nome do primeiro componente "normal" do caminho que corresponde a um nome de
/// dispositivo reservado do Windows.
///
/// Reproduz as regras reais de resolução de nome de dispositivo legado do Win32 (mais
/// completas que a checagem ingênua de prefixo):
/// - Extensão é ignorada pelo PRIMEIRO ponto, não pelo último — `CON.txt` e também
///   `CON.qualquercoisa.txt` são o dispositivo `CON`.
/// - Espaços e pontos finais no nome-base são ignorados pelo parser DOS legado — `"CON "`,
///   `"CON. "`, `"CON . "` etc. também valem como `CON` (mas espaço/ponto NO INÍCIO não —
///   `" CON"` é um nome de arquivo comum, distinto do dispositivo).
/// - Sintaxe de fluxo de dados alternativo (ADS) `nome:stream` — o dispositivo é resolvido
///   pela parte antes do primeiro `:`; o que vem depois é só o nome do stream.
fn reserved_device_name(path: &Path) -> Option<String> {
    path.components().find_map(|component| match component {
        Component::Normal(os_str) => {
            let name = os_str.to_string_lossy();
            let before_stream = name.split(':').next().unwrap_or(&name);
            let raw_stem = before_stream.split('.').next().unwrap_or(before_stream);
            let stem = raw_stem.trim_end_matches([' ', '.']);
            RESERVED_DEVICE_NAMES
                .iter()
                .any(|reserved| reserved.eq_ignore_ascii_case(stem))
                .then(|| name.to_string())
        }
        _ => None,
    })
}

fn contains_traversal(path: &Path) -> bool {
    path.components().any(|c| matches!(c, Component::ParentDir))
}

/// Valida e canonicaliza o caminho raiz de um projeto.
///
/// Rejeita: caminho vazio, com segmento de travessia (`..`), caminhos UNC/verbatim, nomes de
/// dispositivo reservados do Windows, caminhos inexistentes e caminhos que não sejam
/// diretórios. Usa `dunce::canonicalize` (em vez de `std::fs::canonicalize`) para resolver
/// symlinks e obter um caminho absoluto sem os prefixos `\\?\` feios do Windows.
pub fn validate_project_root(raw: &str) -> Result<PathBuf, PathValidationError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(PathValidationError::Empty);
    }

    if is_unc_or_verbatim(trimmed) {
        return Err(PathValidationError::UnsupportedUnc(trimmed.to_string()));
    }

    let raw_path = Path::new(trimmed);

    if contains_traversal(raw_path) {
        return Err(PathValidationError::Traversal);
    }

    if let Some(device) = reserved_device_name(raw_path) {
        return Err(PathValidationError::ReservedDeviceName(device));
    }

    let canonical = dunce::canonicalize(raw_path).map_err(|source| {
        if source.kind() == std::io::ErrorKind::NotFound {
            PathValidationError::NotFound(trimmed.to_string())
        } else {
            PathValidationError::Canonicalize(source.to_string())
        }
    })?;

    // `dunce::canonicalize` só remove o prefixo verbatim `\\?\` quando o caminho final (após
    // resolver symlinks/junctions) é "seguro" pras APIs Win32 legadas: cada componente é um
    // nome de arquivo válido E o comprimento total não passa de ~260 caracteres (MAX_PATH
    // clássico) — ver `dunce::is_safe_to_strip_unc`. Quando isso falha (caminho local
    // realmente longo — deep node_modules, monorepo, OneDrive, usuário com nome longo — ou o
    // alvo real fica atrás de um drive de rede mapeado), o valor devolvido AINDA começa com
    // `\\` (verbatim ou UNC de verdade).
    //
    // Aceitar esse valor quebraria a invariante da qual `commands::projects::create_project`
    // (que persiste o retorno daqui no SQLite) e `projects::scanner::scan_project` (que
    // re-valida o mesmo valor a cada scan) dependem: "o que `validate_project_root` devolve,
    // `validate_project_root` também aceita de volta". Sem este check, a MESMA string
    // resolvida aqui bateria no check de UNC/verbatim logo no início na PRÓXIMA chamada e
    // falharia — um projeto local legítimo validaria uma vez e nunca mais. Em vez de devolver
    // um valor não-re-validável, tratamos como o mesmo caso "não suportado" que caminhos UNC
    // brutos já são.
    if is_unc_or_verbatim(&canonical.to_string_lossy()) {
        return Err(PathValidationError::UnsupportedUnc(
            canonical.display().to_string(),
        ));
    }

    if !canonical.is_dir() {
        return Err(PathValidationError::NotADirectory(
            canonical.display().to_string(),
        ));
    }

    Ok(canonical)
}

/// Garante que `candidate` — um arquivo ou diretório dentro de um projeto já validado —
/// resolve (após seguir symlinks) para um caminho que continua dentro de `root`. `root` deve
/// já ter passado por [`validate_project_root`]. Usado pelo scanner de importação assistida
/// para recusar symlinks que escapem do diretório do projeto.
pub fn ensure_within_root(root: &Path, candidate: &Path) -> Result<PathBuf, PathValidationError> {
    let canonical = dunce::canonicalize(candidate)
        .map_err(|source| PathValidationError::Canonicalize(source.to_string()))?;

    if !canonical.starts_with(root) {
        return Err(PathValidationError::EscapesRoot);
    }

    Ok(canonical)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_path() {
        assert_eq!(validate_project_root(""), Err(PathValidationError::Empty));
        assert_eq!(
            validate_project_root("   "),
            Err(PathValidationError::Empty)
        );
    }

    #[test]
    fn rejects_traversal() {
        let err = validate_project_root(r"C:\projects\..\Windows\System32");
        assert_eq!(err, Err(PathValidationError::Traversal));

        let err2 = validate_project_root(r"..\..\secrets");
        assert_eq!(err2, Err(PathValidationError::Traversal));
    }

    #[test]
    fn rejects_unc_path() {
        let err = validate_project_root(r"\\server\share\projeto");
        assert_eq!(
            err,
            Err(PathValidationError::UnsupportedUnc(
                r"\\server\share\projeto".into()
            ))
        );
    }

    #[test]
    fn rejects_verbatim_prefix_trick() {
        let err = validate_project_root(r"\\?\C:\projects\codevoice");
        assert!(matches!(err, Err(PathValidationError::UnsupportedUnc(_))));
    }

    #[test]
    fn rejects_reserved_device_names() {
        assert_eq!(
            validate_project_root(r"C:\projects\CON"),
            Err(PathValidationError::ReservedDeviceName("CON".into()))
        );
        assert_eq!(
            validate_project_root(r"C:\projects\com1\sub"),
            Err(PathValidationError::ReservedDeviceName("com1".into()))
        );
        assert_eq!(
            validate_project_root(r"C:\NUL.txt\projeto"),
            Err(PathValidationError::ReservedDeviceName("NUL.txt".into()))
        );
    }

    #[test]
    fn accepts_and_canonicalizes_existing_directory() {
        let dir = tempfile::tempdir().unwrap();
        let result = validate_project_root(dir.path().to_str().unwrap()).unwrap();
        assert!(result.is_absolute());
        assert!(result.is_dir());
        assert!(!result.display().to_string().starts_with(r"\\?\"));
    }

    #[test]
    fn rejects_missing_path() {
        let dir = tempfile::tempdir().unwrap();
        let missing = dir.path().join("nao-existe");
        let err = validate_project_root(missing.to_str().unwrap());
        assert!(matches!(err, Err(PathValidationError::NotFound(_))));
    }

    #[test]
    fn rejects_file_that_is_not_a_directory() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("arquivo.txt");
        std::fs::write(&file_path, "conteudo").unwrap();

        let err = validate_project_root(file_path.to_str().unwrap());
        assert!(matches!(err, Err(PathValidationError::NotADirectory(_))));
    }

    #[test]
    fn ensure_within_root_accepts_path_inside() {
        let dir = tempfile::tempdir().unwrap();
        let root = dunce::canonicalize(dir.path()).unwrap();
        let file_path = root.join("README.md");
        std::fs::write(&file_path, "ok").unwrap();

        let result = ensure_within_root(&root, &file_path).unwrap();
        assert!(result.starts_with(&root));
    }

    /// ATAQUE ADVERSARIAL: trick clássico do Windows — a API Win32 (CreateFileW e por
    /// consequência `std::fs::canonicalize`/`dunce::canonicalize`) ignora espaços e pontos
    /// finais em cada componente de um caminho DOS "legado" antes de resolver. Um componente
    /// literal `".. "` (dot dot space) ou `"..."` NÃO é igual a `".."` para o parser de
    /// `std::path::Component` (não vira `ParentDir`), então `contains_traversal` não o
    /// detecta — mas se o Win32 ainda o interpretar como parent-dir ao abrir o handle, isso
    /// seria um bypass real da checagem de travessia.
    #[test]
    fn adversarial_trailing_space_dotdot_does_not_bypass_traversal_check() {
        let base = tempfile::tempdir().unwrap();
        let project_root = base.path().join("project");
        std::fs::create_dir_all(&project_root).unwrap();
        // Sentinela: se ".. " for interpretado como parent-dir, o canonicalize abaixo
        // resolveria para `base` (que contém este arquivo) em vez de falhar/ficar dentro de
        // `project`.
        std::fs::write(base.path().join("sentinela.txt"), "fora do projeto").unwrap();

        for suffix in [".. ", "...", ".. .", "..  ", ".."] {
            let candidate = format!("{}\\{}", project_root.display(), suffix);
            let result = validate_project_root(&candidate);
            eprintln!("candidato={candidate:?} -> {result:?}");

            match result {
                Err(PathValidationError::Traversal) => {
                    // Caso ideal: pego pelo parser de componentes.
                }
                Err(_) => {
                    // Também aceitável: falhou por outro motivo (ex.: NotFound porque o
                    // diretório literal ".. " não existe) — desde que NÃO tenha resolvido
                    // para fora de `project_root`.
                }
                Ok(resolved) => {
                    assert!(
                        resolved.starts_with(dunce::canonicalize(&project_root).unwrap()),
                        "BYPASS CONFIRMADO: candidato {candidate:?} escapou da raiz do \
                         projeto e resolveu para {resolved:?}"
                    );
                }
            }
        }
    }

    /// ATAQUE ADVERSARIAL: byte NUL embutido no meio da string. Em C, muitas APIs Win32
    /// truncam strings no primeiro NUL — se o parser Rust (que opera sobre a string completa,
    /// já que `&str` no Rust pode conter NUL) e a API Win32 (que trunca no NUL) discordarem
    /// sobre onde o caminho "termina", poderia existir uma janela de bypass.
    #[test]
    fn adversarial_embedded_nul_byte_is_rejected_or_harmless() {
        let raw = "C:\\projects\\algumacoisa\0\\..\\..\\Windows\\System32";
        let result = validate_project_root(raw);
        eprintln!("resultado para caminho com NUL embutido: {result:?}");
        // Não deve ser Ok apontando pra fora — ou falha (qualquer erro), ou (idealmente)
        // detecta a travessia mesmo com o NUL no meio.
        assert!(
            result.is_err(),
            "caminho com NUL embutido não deveria validar com sucesso"
        );
    }

    /// ATAQUE ADVERSARIAL: travessia usando barra normal ("/") em vez de barra invertida,
    /// e caminhos com barras mistas — confirma que `Component::ParentDir` é reconhecido
    /// independentemente do separador usado no Windows.
    #[test]
    fn adversarial_forward_slash_and_mixed_slash_traversal_is_caught() {
        let cases = [
            "C:/projects/../Windows/System32",
            r"C:\projects/../Windows\System32",
            "C:/projects/..\\..\\Windows",
            "../../secrets",
            "..//..//secrets",
        ];
        for raw in cases {
            let result = validate_project_root(raw);
            eprintln!("candidato={raw:?} -> {result:?}");
            assert_eq!(
                result,
                Err(PathValidationError::Traversal),
                "travessia com barras mistas/normais não foi detectada para {raw:?}"
            );
        }
    }

    /// ATAQUE ADVERSARIAL: "URL-encoding" de "..\\" (%2e%2e%5c). Como `validate_project_root`
    /// nunca faz percent-decoding, isso deve ser tratado como um nome de arquivo/diretório
    /// literal (e portanto falhar com NotFound, não ser interpretado como travessia real).
    #[test]
    fn adversarial_url_encoded_traversal_is_treated_as_literal_and_not_found() {
        let dir = tempfile::tempdir().unwrap();
        let candidate = dir.path().join("%2e%2e%5cWindows");
        let result = validate_project_root(candidate.to_str().unwrap());
        eprintln!("resultado para candidato url-encoded: {result:?}");
        assert!(matches!(result, Err(PathValidationError::NotFound(_))));
    }

    /// ATAQUE ADVERSARIAL: caminho absoluto disfarçado com prefixo verbatim `\\?\` que tenta
    /// embutir travessia depois do prefixo — deve ser pego pelo check de UNC/verbatim antes
    /// mesmo de olhar para o `..`.
    #[test]
    fn adversarial_verbatim_prefix_with_traversal_after_it() {
        let result = validate_project_root(r"\\?\C:\projects\..\..\Windows\System32");
        assert!(matches!(
            result,
            Err(PathValidationError::UnsupportedUnc(_))
        ));
    }

    /// ATAQUE ADVERSARIAL: caminho relativo à unidade (drive-relative, sem `\` após `C:`).
    /// `C:secrets` no Windows não significa "raiz da unidade C" — significa "relativo ao
    /// diretório atual da unidade C" (um conceito por processo, via variável de ambiente
    /// oculta `=C:`). Confirma que isso não permite escapar silenciosamente sem passar por
    /// canonicalize (ou seja, o resultado final ainda é validado/absoluto).
    #[test]
    fn adversarial_drive_relative_path_is_still_canonicalized_or_rejected() {
        let result = validate_project_root("C:temp-nao-deveria-existir-xyz123");
        eprintln!("resultado para caminho drive-relative: {result:?}");
        // Não deve ser aceito silenciosamente sem canonicalização — ou falha, ou (se por
        // acaso o cwd em C: tiver essa pasta) resolve para um caminho absoluto real.
        if let Ok(resolved) = result {
            assert!(resolved.is_absolute());
        }
    }

    /// ATAQUE ADVERSARIAL (refinado): a variante anterior colocava o componente `".. "` no
    /// final da string inteira, onde o `.trim()` de `validate_project_root` (que só afeta as
    /// pontas do *raw* completo) acaba removendo o espaço de qualquer forma, mascarando o
    /// teste. Aqui o componente `".. "` fica no MEIO do caminho (seguido de mais um
    /// segmento), fora do alcance do `.trim()` externo — testando de verdade se
    /// `Component::ParentDir` reconhece um componente com espaço/ponto interno como
    /// equivalente a `".."`.
    #[test]
    fn adversarial_trailing_space_dotdot_mid_path_does_not_bypass_traversal_check() {
        let base = tempfile::tempdir().unwrap();
        let project_root = base.path().join("project");
        std::fs::create_dir_all(&project_root).unwrap();
        std::fs::write(base.path().join("sentinela.txt"), "fora do projeto").unwrap();
        let canonical_base = dunce::canonicalize(base.path()).unwrap();
        let canonical_project = dunce::canonicalize(&project_root).unwrap();

        let mid_path_cases = [
            format!("{}\\.. \\alvo", project_root.display()),
            format!("{}\\...\\alvo", project_root.display()),
            format!("{}\\.. .\\alvo", project_root.display()),
            format!("{}\\..\\alvo", project_root.display()),
        ];

        for candidate in mid_path_cases {
            let result = validate_project_root(&candidate);
            eprintln!("candidato (meio de caminho)={candidate:?} -> {result:?}");

            match result {
                Err(PathValidationError::Traversal) => {}
                Err(_) => {}
                Ok(resolved) => {
                    assert!(
                        resolved.starts_with(&canonical_project) || resolved == canonical_base,
                        "candidato aceito {candidate:?} não deveria resolver fora do projeto \
                         nem para a raiz `base`; resolveu para {resolved:?}"
                    );
                    // Mais importante: nunca deve escapar para o `sentinela.txt` de fora.
                    assert!(
                        resolved != base.path().join("alvo"),
                        "BYPASS CONFIRMADO: {candidate:?} escapou para fora de project_root via \
                         trick de espaço/ponto final em componente do meio do caminho"
                    );
                }
            }
        }
    }

    /// ATAQUE ADVERSARIAL: confusão de prefixo entre diretórios "irmãos" (`root` = `Foo`,
    /// candidato real = `FooBar`). Um bug clássico é comparar caminhos com prefixo de string
    /// crua (`"...FooBar".starts_with("...Foo")` → `true`, incorretamente). `ensure_within_root`
    /// usa `Path::starts_with`, que compara por *componentes*, não por bytes crus — confirma
    /// que isso realmente protege contra esse caso.
    #[test]
    fn adversarial_sibling_directory_prefix_confusion_is_rejected() {
        let base = tempfile::tempdir().unwrap();
        let root = base.path().join("Foo");
        let sibling = base.path().join("FooBar");
        std::fs::create_dir_all(&root).unwrap();
        std::fs::create_dir_all(&sibling).unwrap();
        std::fs::write(sibling.join("segredo.txt"), "fora do projeto Foo").unwrap();

        let canonical_root = dunce::canonicalize(&root).unwrap();
        let candidate_outside = sibling.join("segredo.txt");

        let result = ensure_within_root(&canonical_root, &candidate_outside);
        assert_eq!(
            result,
            Err(PathValidationError::EscapesRoot),
            "BYPASS CONFIRMADO: diretório irmão com nome que compartilha prefixo textual \
             ('FooBar' vs root 'Foo') não foi rejeitado por ensure_within_root"
        );
    }

    #[test]
    fn ensure_within_root_rejects_symlink_escaping_root() {
        let outside = tempfile::tempdir().unwrap();
        let outside_file = outside.path().join("segredo.txt");
        std::fs::write(&outside_file, "fora do projeto").unwrap();

        let project = tempfile::tempdir().unwrap();
        let root = dunce::canonicalize(project.path()).unwrap();
        let link_path = root.join("link.txt");

        #[cfg(windows)]
        let created = std::os::windows::fs::symlink_file(&outside_file, &link_path).is_ok();
        #[cfg(not(windows))]
        let created = std::os::unix::fs::symlink(&outside_file, &link_path).is_ok();

        if !created {
            eprintln!(
                "aviso: sem privilégio para criar symlink neste ambiente (SeCreateSymbolicLinkPrivilege \
                 ausente / modo desenvolvedor desligado) — teste pulado, não é uma falha da lógica"
            );
            return;
        }

        let err = ensure_within_root(&root, &link_path);
        assert_eq!(err, Err(PathValidationError::EscapesRoot));
    }

    /// REGRESSÃO (revisão adversarial): `reserved_device_name` usava um casamento de "stem"
    /// mais estreito que as regras reais do Win32, deixando passar (como `NotFound` em vez de
    /// `ReservedDeviceName`) variantes com espaço/ponto final em um componente que NÃO é o
    /// último do caminho inteiro (fora do alcance do `.trim()` de nível superior), e a sintaxe
    /// de fluxo de dados alternativo (`CON:stream`). Nenhum dos três chegava a interagir de
    /// verdade com um dispositivo real (o passo seguinte, canonicalize, sempre falhava com
    /// `NotFound` de qualquer forma) — mas a detecção deve ser completa por si só, sem
    /// depender de um segundo mecanismo pegar o que o primeiro deixou passar.
    #[test]
    fn regression_reserved_device_name_catches_space_dot_and_ads_variants() {
        let dir = tempfile::tempdir().unwrap();
        let base = dir.path().to_str().unwrap();

        // (a) nome reservado + espaço à direita como componente do MEIO do caminho (não o
        // último caractere da string inteira — fora do alcance do `.trim()` externo).
        let case_a = format!("{base}\\CON \\sub\\leaf.md");
        assert!(
            matches!(
                validate_project_root(&case_a),
                Err(PathValidationError::ReservedDeviceName(_))
            ),
            "esperava ReservedDeviceName para {case_a:?}, obteve {:?}",
            validate_project_root(&case_a)
        );

        // (b) espaço embutido antes do primeiro ponto.
        let case_b = format!("{base}\\CON . .txt");
        assert!(
            matches!(
                validate_project_root(&case_b),
                Err(PathValidationError::ReservedDeviceName(_))
            ),
            "esperava ReservedDeviceName para {case_b:?}, obteve {:?}",
            validate_project_root(&case_b)
        );

        // (c) sintaxe de fluxo de dados alternativo (ADS) `nome:stream`.
        let case_c = format!("{base}\\CON:stream");
        assert!(
            matches!(
                validate_project_root(&case_c),
                Err(PathValidationError::ReservedDeviceName(_))
            ),
            "esperava ReservedDeviceName para {case_c:?}, obteve {:?}",
            validate_project_root(&case_c)
        );
    }

    /// REGRESSÃO (revisão adversarial): um caminho local real e legítimo cujo comprimento
    /// canonicalizado passa de ~260 caracteres faz `dunce::canonicalize` devolver um valor
    /// ainda prefixado com `\\?\` (não consegue remover o prefixo com segurança — ver
    /// `dunce::is_safe_to_strip_unc`). Antes desta correção, `validate_project_root` aceitava
    /// esse valor como `Ok`, mas re-submeter a MESMA string (exatamente o que
    /// `commands::projects::create_project` faz ao persistir, e o que `scanner::scan_project`
    /// faz a cada novo scan) batia no check de UNC/verbatim logo no início e falhava com
    /// `UnsupportedUnc` — um projeto local válido validava uma vez e nunca mais podia ser
    /// re-validado. Agora a primeira chamada já rejeita de forma clara e determinística.
    #[test]
    fn regression_long_local_path_never_returns_a_verbatim_prefixed_ok_that_fails_on_revalidation()
    {
        let base = tempfile::tempdir().unwrap();
        let mut long_path = base.path().to_path_buf();
        for i in 0..10 {
            long_path.push(format!(
                "segmento-bem-comprido-numero-{i:02}-para-estourar-max-path"
            ));
        }
        assert!(
            long_path.to_string_lossy().len() > 260,
            "fixture não ficou longa o suficiente para o teste: {} chars",
            long_path.to_string_lossy().len()
        );

        // Cria a árvore usando o prefixo verbatim explicitamente — isso contorna o limite
        // clássico de MAX_PATH das APIs Win32 legadas na hora de CRIAR o diretório,
        // independentemente de o host de CI ter ou não a política "Enable Win32 long paths"
        // ligada. O que está sob teste é `validate_project_root` recebendo depois um caminho
        // *sem* o prefixo (exatamente como um usuário digitaria/colaria).
        let verbatim_for_setup = format!(r"\\?\{}", long_path.display());
        std::fs::create_dir_all(&verbatim_for_setup)
            .expect("setup: falha ao criar diretório de teste com caminho longo");

        let raw = long_path.to_str().unwrap();
        let first = validate_project_root(raw);
        eprintln!("primeira validação de caminho longo: {first:?}");

        match &first {
            Ok(resolved) => {
                // Se algum dia isso passar a ser aceito (ex.: dunce mudar de comportamento),
                // a invariante de re-validação tem que se manter: o valor devolvido precisa
                // continuar validando com sucesso na segunda chamada, com o MESMO resultado.
                let second = validate_project_root(resolved.to_str().unwrap());
                assert!(
                    second.is_ok(),
                    "BYPASS CONFIRMADO: validate_project_root devolveu Ok({resolved:?}) na \
                     primeira chamada, mas esse mesmo valor falha ao ser revalidado: {second:?}"
                );
            }
            Err(PathValidationError::UnsupportedUnc(_)) => {
                // Caso esperado: rejeitado de forma clara e determinística logo na primeira
                // chamada, em vez de aceito e quebrado depois.
            }
            Err(other) => {
                panic!(
                    "esperava Ok (com invariante preservada) ou UnsupportedUnc, obteve: {other:?}"
                );
            }
        }

        // Em qualquer caso, chamar duas vezes seguidas com o MESMO `raw` tem que produzir o
        // MESMO resultado (determinismo) — nunca "aceita da primeira vez, rejeita da segunda".
        let repeat = validate_project_root(raw);
        assert_eq!(
            first.is_ok(),
            repeat.is_ok(),
            "validate_project_root não é determinístico para o mesmo caminho longo: \
             primeira={first:?} segunda={repeat:?}"
        );
    }
}
