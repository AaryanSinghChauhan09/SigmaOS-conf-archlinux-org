//! Front-matter parsing: `---`-delimited YAML-ish header + markdown body.

/// Parsed front matter: scalar key→value pairs plus the body.
#[derive(Debug, Clone, Default)]
pub struct FrontMatter {
    /// Scalar fields (title, date, draft, ...).
    pub fields: Vec<(String, String)>,
    /// Raw markdown body after the closing `---`.
    pub body: String,
}

impl FrontMatter {
    /// Get a field value.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.fields
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }
}

/// Parse `---\nkey: value\n---\nbody` format.
///
/// Handles `key: value`, `key: "quoted value"`, and `key:` with empty
/// value. Nested structures are not supported (conf site uses scalars).
pub fn parse_front_matter(text: &str) -> Result<FrontMatter, crate::ConfError> {
    let mut lines = text.lines();
    // Must start with ---
    if lines.next().map(str::trim) != Some("---") {
        return Err(crate::ConfError::Parse("missing opening ---".into()));
    }
    let mut fm = FrontMatter::default();
    let mut in_fm = true;
    let mut body_lines = Vec::new();
    for line in lines {
        if in_fm {
            if line.trim() == "---" {
                in_fm = false;
                continue;
            }
            if let Some((k, v)) = line.split_once(':') {
                let key = k.trim().to_string();
                let value = v.trim().trim_matches('"').to_string();
                fm.fields.push((key, value));
            }
        } else {
            body_lines.push(line);
        }
    }
    fm.body = body_lines.join("\n");
    Ok(fm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic() {
        let text = "---\ntitle: \"About\"\ndate: 2019-05-26\n---\nHello world.\n";
        let fm = parse_front_matter(text).unwrap();
        assert_eq!(fm.get("title"), Some("About"));
        assert_eq!(fm.get("date"), Some("2019-05-26"));
        assert!(fm.body.contains("Hello world"));
    }

    #[test]
    fn test_no_front_matter_errors() {
        assert!(parse_front_matter("no front matter\n").is_err());
    }
}
