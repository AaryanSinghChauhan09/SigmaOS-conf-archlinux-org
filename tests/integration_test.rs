//! Integration tests for sigmaos-conf-archlinux-org.

use conf_archlinux_org::{scan_content, parse_config, load_archetype, parse_front_matter};

#[test]
fn test_full_site_model() {
    let dir = tempfile::tempdir().unwrap();
    // config.toml
    std::fs::write(dir.path().join("config.toml"),
        "baseURL = \"https://conf.archlinux.org/\"\ntitle = \"Arch Conf\"\ntheme = \"arch\"\n\n[[menu.main]]\n  Name = \"Download\"\n  URL = \"https://archlinux.org/download\"\n  weight = 0\n").unwrap();
    // content
    let pages = dir.path().join("content/pages");
    std::fs::create_dir_all(&pages).unwrap();
    std::fs::write(pages.join("about.md"), "---\ntitle: \"About Arch Conf\"\ndate: 2019-05-26T16:19:54+02:00\n---\nText.\n").unwrap();
    let notes = dir.path().join("content/notes/archconf2019");
    std::fs::create_dir_all(&notes).unwrap();
    std::fs::write(notes.join("main.md"), "---\ntitle: Main\n---\nNotes.\n").unwrap();
    // archetype
    let arch = dir.path().join("archetypes");
    std::fs::create_dir_all(&arch).unwrap();
    std::fs::write(arch.join("default.md"), "---\ntitle: \"{{ .Name }}\"\n---\n").unwrap();

    let cfg = parse_config(&std::fs::read_to_string(dir.path().join("config.toml")).unwrap()).unwrap();
    assert_eq!(cfg.menus.len(), 1);
    assert_eq!(cfg.theme, "arch");

    let idx = scan_content(&dir.path().join("content")).unwrap();
    assert_eq!(idx.len(), 2);
    assert_eq!(idx.in_section("notes").len(), 1);

    let a = load_archetype(&arch, "default").unwrap();
    assert_eq!(a.name, "default");
}

#[test]
fn test_frontmatter_body() {
    let fm = parse_front_matter("---\ntitle: T\n---\nBody line 1\nline 2\n").unwrap();
    assert_eq!(fm.body.trim(), "Body line 1\nline 2");
}
