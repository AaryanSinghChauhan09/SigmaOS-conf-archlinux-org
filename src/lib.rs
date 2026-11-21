//! SigmaOS conf-archlinux-org component.
//!
//! Rust implementation of `archlinux/conf.archlinux.org`: the Arch Conf
//! website — a Hugo content site with TOML front-matter pages, menu config,
//! and archetypes.
//!
//! This port implements the content pipeline: parsing `---`-delimited front
//! matter, indexing `content/` trees into pages, reading `config.toml`
//! menus, and resolving archetypes.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod archetype;
pub mod config;
pub mod content;
pub mod error;
pub mod frontmatter;

pub use archetype::{Archetype, load_archetype};
pub use config::{MenuItem, SiteConfig, parse_config};
pub use content::{ContentIndex, Page, scan_content};
pub use error::ConfError;
pub use frontmatter::{FrontMatter, parse_front_matter};
