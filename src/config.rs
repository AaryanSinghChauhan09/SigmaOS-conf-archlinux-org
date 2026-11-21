//! config.toml parsing — menus and site metadata.

use crate::ConfError;

/// A menu entry from `[[menu.main]]` blocks.
#[derive(Debug, Clone)]
pub struct MenuItem {
    /// Display name.
    pub name: String,
    /// URL.
    pub url: String,
    /// Tooltip/title.
    pub title: String,
    /// Sort weight (lower = earlier).
    pub weight: i64,
}

/// Site configuration extracted from config.toml.
#[derive(Debug, Clone, Default)]
pub struct SiteConfig {
    /// baseURL.
    pub base_url: String,
    /// Site title.
    pub title: String,
    /// Theme name.
    pub theme: String,
    /// Menu entries (all `[[menu.*]]` tables flattened).
    pub menus: Vec<MenuItem>,
}

/// Parse config.toml text, extracting scalar keys and `[[menu.*]]` blocks.
pub fn parse_config(text: &str) -> Result<SiteConfig, ConfError> {
    let mut cfg = SiteConfig::default();
    let mut in_menu = false;
    let mut cur = MenuItem { name: String::new(), url: String::new(), title: String::new(), weight: 0 };
    let mut flush = |cfg: &mut SiteConfig, cur: &mut MenuItem| {
        if !cur.name.is_empty() || !cur.url.is_empty() {
            cfg.menus.push(MenuItem {
                name: std::mem::take(&mut cur.name),
                url: std::mem::take(&mut cur.url),
                title: std::mem::take(&mut cur.title),
                weight: cur.weight,
            });
        }
    };

    for raw in text.lines() {
        let line = raw.trim();
        if line.starts_with("[[menu.") {
            flush(&mut cfg, &mut cur);
            in_menu = true;
            continue;
        }
        if line.starts_with('[') {
            flush(&mut cfg, &mut cur);
            in_menu = false;
            continue;
        }
        let Some((k, v)) = line.split_once('=') else { continue };
        let (k, v) = (k.trim(), v.trim().trim_matches('"'));
        if in_menu {
            match k {
                "Name" => cur.name = v.to_string(),
                "URL" => cur.url = v.to_string(),
                "Title" => cur.title = v.to_string(),
                "weight" => cur.weight = v.parse().unwrap_or(0),
                _ => {}
            }
        } else {
            match k {
                "baseURL" => cfg.base_url = v.to_string(),
                "title" => cfg.title = v.to_string(),
                "theme" => cfg.theme = v.to_string(),
                _ => {}
            }
        }
    }
    flush(&mut cfg, &mut cur);
    Ok(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_menus() {
        let text = "baseURL = \"https://conf/\"\ntitle = \"Arch Conf\"\n\n[[menu.main]]\n  Name = \"Download\"\n  URL = \"https://archlinux.org/download\"\n  weight = 0\n\n[[menu.main]]\n  Name = \"AUR\"\n  URL = \"https://aur.archlinux.org/\"\n  weight = -1\n";
        let cfg = parse_config(text).unwrap();
        assert_eq!(cfg.base_url, "https://conf/");
        assert_eq!(cfg.menus.len(), 2);
        assert_eq!(cfg.menus[1].name, "AUR");
        assert_eq!(cfg.menus[1].weight, -1);
    }
}
