use crate::{EditorCommand, EditorSource, editor_parser::parse_editor};
use minerva_domain::MinervaError;

/// Resolution order: `MINERVA_EDITOR`, `VISUAL`, `EDITOR`, configured editor,
/// then a platform fallback. Unix-like targets default to `nvim`; Windows
/// defaults to `notepad`. GUI editors should include their own wait flag.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EditorEnvironment {
    pub minerva_editor: Option<String>,
    pub visual: Option<String>,
    pub editor: Option<String>,
}

impl EditorEnvironment {
    #[must_use]
    pub fn capture() -> Self {
        Self {
            minerva_editor: std::env::var("MINERVA_EDITOR").ok(),
            visual: std::env::var("VISUAL").ok(),
            editor: std::env::var("EDITOR").ok(),
        }
    }

    pub fn resolve(
        &self,
        configured: Option<&str>,
    ) -> Result<EditorCommand, MinervaError> {
        self.resolve_with_fallback(configured, platform_fallback())
    }

    pub fn resolve_with_fallback(
        &self,
        configured: Option<&str>,
        fallback: &str,
    ) -> Result<EditorCommand, MinervaError> {
        for (source, spec) in candidates(self, configured, fallback) {
            let Some(spec) = normalize(spec) else {
                continue;
            };
            let (program, args) = parse_editor(spec)?;
            return Ok(EditorCommand { source, spec: spec.into(), program, args });
        }
        unreachable!("platform fallback is always present")
    }
}

fn candidates<'a>(
    env: &'a EditorEnvironment,
    configured: Option<&'a str>,
    fallback: &'a str,
) -> [(EditorSource, Option<&'a str>); 5] {
    [
        (EditorSource::MinervaEditor, env.minerva_editor.as_deref()),
        (EditorSource::Visual, env.visual.as_deref()),
        (EditorSource::Editor, env.editor.as_deref()),
        (EditorSource::Configured, configured),
        (EditorSource::Fallback, Some(fallback)),
    ]
}

fn normalize(value: Option<&str>) -> Option<&str> {
    let trimmed = value?.trim();
    (!trimmed.is_empty()).then_some(trimmed)
}

#[cfg(windows)]
const fn platform_fallback() -> &'static str {
    "notepad"
}

#[cfg(not(windows))]
const fn platform_fallback() -> &'static str {
    "nvim"
}
