use crate::MinervaError;

const TITLE: &str = "# Declaration";
const SECTIONS: [&str; 8] = [
    "Objective",
    "Current State",
    "Completed Work",
    "Remaining Work",
    "Decisions",
    "Risks",
    "Verification",
    "Open Questions",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclarationDocument;

impl DeclarationDocument {
    #[must_use]
    pub fn template() -> String {
        let mut output = String::from(TITLE);
        for heading in SECTIONS {
            output.push_str("\n\n## ");
            output.push_str(heading);
            output.push('\n');
        }
        output.push('\n');
        output
    }

    pub fn parse(contents: &str) -> Result<Self, MinervaError> {
        if !contents.lines().any(|line| line.trim() == TITLE) {
            return invalid("declaration.title", "must include `# Declaration`");
        }
        for section in SECTIONS {
            let heading = format!("## {section}");
            if !contents.lines().any(|line| line.trim() == heading) {
                return invalid("declaration.heading", &format!("missing `{heading}`"));
            }
        }
        Ok(Self)
    }

    #[must_use]
    pub fn content_hash(contents: &str) -> String {
        let mut hash = 0xcbf2_9ce4_8422_2325_u64;
        for byte in contents.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        format!("{hash:016x}")
    }
}

fn invalid<T>(key: &str, reason: &str) -> Result<T, MinervaError> {
    Err(MinervaError::InvalidConfiguration { key: key.into(), reason: reason.into() })
}
