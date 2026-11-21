//! Content tree scanning (`content/pages`, `content/notes`, ...).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::frontmatter::{FrontMatter, parse_front_matter};
use crate::ConfError;

/// A content page (markdown file with front matter).
#[derive(Debug, Clone)]
pub struct Page {
    /// Path relative to `content/` (e.g. `pages/about.md`).
    pub rel_path: String,
    /// Section (`pages`, `notes`, ...).
    pub section: String,
    /// Parsed front matter + body.
    pub fm: FrontMatter,
}

impl Page {
    /// Page title (front-matter `title`, else filename stem).
    pub fn title(&self) -> String {
        self.fm
            .get("title")
            .map(str::to_string)
            .unwrap_or_else(|| {
                Path::new(&self.rel_path)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("untitled")
                    .to_string()
            })
    }

    /// Whether the page is a draft.
    pub fn is_draft(&self) -> bool {
        self.fm.get("draft").map(|v| v == "true").unwrap_or(false)
    }
}

/// Index of all content pages.
#[derive(Debug, Default)]
pub struct ContentIndex {
    /// Pages keyed by rel_path.
    pub pages: BTreeMap<String, Page>,
}

impl ContentIndex {
    /// Pages in a section.
    pub fn in_section(&self, section: &str) -> Vec<&Page> {
        self.pages
            .values()
            .filter(|p| p.section == section)
            .collect()
    }

    /// Non-draft pages only.
    pub fn published(&self) -> Vec<&Page> {
        self.pages.values().filter(|p| !p.is_draft()).collect()
    }

    /// Total page count.
    pub fn len(&self) -> usize {
        self.pages.len()
    }
}

/// Scan `content/` recursively for `.md` files.
pub fn scan_content(content_dir: &Path) -> Result<ContentIndex, ConfError> {
    let mut idx = ContentIndex::default();
    scan_dir(content_dir, content_dir, &mut idx)?;
    Ok(idx)
}

fn scan_dir(root: &Path, dir: &Path, idx: &mut ContentIndex) -> Result<(), ConfError> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            scan_dir(root, &path, idx)?;
        } else if path.extension().and_then(|e| e.to_str()) == Some("md") {
            let rel = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            let section = rel.split('/').next().unwrap_or("").to_string();
            let text = std::fs::read_to_string(&path)?;
            let fm = parse_front_matter(&text).unwrap_or(FrontMatter {
                fields: vec![],
                body: text,
            });
            idx.pages.insert(
                rel.clone(),
                Page { rel_path: rel, section, fm },
            );
        }
    }
    Ok(())
}

/// Absolute path for a page rel_path under content root.
pub fn page_path(content_root: &Path, rel: &str) -> PathBuf {
    content_root.join(rel)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_page(root: &Path, rel: &str, fm: &str, body: &str) {
        let p = root.join(rel);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, format!("---\n{fm}\n---\n{body}")).unwrap();
    }

    #[test]
    fn test_scan_content() {
        let dir = tempfile::tempdir().unwrap();
        write_page(dir.path(), "pages/about.md", "title: About\ndate: 2019-05-26", "Body text");
        write_page(dir.path(), "notes/s1/talk.md", "title: Talk\ndraft: true", "Draft");
        write_page(dir.path(), "pages/schedule.md", "title: Schedule", "Sched");
        let idx = scan_content(dir.path()).unwrap();
        assert_eq!(idx.len(), 3);
        assert_eq!(idx.in_section("pages").len(), 2);
        assert_eq!(idx.published().len(), 2); // draft excluded
        let about = idx.pages.get("pages/about.md").unwrap();
        assert_eq!(about.title(), "About");
    }
}
