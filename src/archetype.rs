//! Hugo archetype support (`archetypes/<name>.md`).

use std::path::Path;

/// An archetype template: front-matter skeleton + body template.
#[derive(Debug, Clone)]
pub struct Archetype {
    /// Archetype name (filename without .md).
    pub name: String,
    /// Front-matter fields as raw `key: value` lines.
    pub fields: Vec<(String, String)>,
    /// Body template text.
    pub body: String,
}

/// Load `archetypes/<name>.md` (or `default.md` when name not found).
pub fn load_archetype(archetypes_dir: &Path, name: &str) -> Result<Archetype, crate::ConfError> {
    let path = archetypes_dir.join(format!("{name}.md"));
    let path = if path.exists() {
        path
    } else {
        archetypes_dir.join("default.md")
    };
    let text = std::fs::read_to_string(&path)?;
    let fm_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("default")
        .to_string();
    let fm = crate::frontmatter::parse_front_matter(&text)
        .unwrap_or_else(|_| crate::frontmatter::FrontMatter {
            fields: vec![],
            body: text.clone(),
        });
    Ok(Archetype {
        name: fm_name,
        fields: fm.fields,
        body: fm.body,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_default_archetype() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("default.md"),
            "---\ntitle: \"{{ replace .Name \"mytitle\" }}\"\n---\nBody here.\n",
        )
        .unwrap();
        let a = load_archetype(dir.path(), "nonexistent").unwrap();
        assert_eq!(a.name, "default");
        assert_eq!(a.get("title").unwrap(), "{{ replace .Name \"mytitle\" }}");
    }
}

impl Archetype {
    /// Get a front-matter field value.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.fields.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
    }
}
