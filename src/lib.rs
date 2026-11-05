//! Community conference website
//!
//! SigmaOS component inspired by Arch Linux's `conf-archlinux-org` repository.
//! Designed for component isolation, loose coupling, and secure defaults.

use std::fmt;

/// Errors produced by this component. Fail-fast: errors are returned, not panicked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sigmaos_conf_archlinux_orgError {
    /// The requested operation is not valid in the current state.
    InvalidState,
    /// Input failed validation.
    InvalidInput(String),
}

impl fmt::Display for Sigmaos_conf_archlinux_orgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidState => write!(f, "invalid state"),
            Self::InvalidInput(msg) => write!(f, "invalid input: {msg}"),
        }
    }
}

impl std::error::Error for Sigmaos_conf_archlinux_orgError {}

/// Core component type. Holds configuration; performs no I/O by default.
#[derive(Debug, Clone, Default)]
pub struct Sigmaos_conf_archlinux_org {
    name: String,
}

impl Sigmaos_conf_archlinux_org {
    /// Create a new component instance. Fails fast on empty names.
    pub fn new(name: impl Into<String>) -> Result<Self, Sigmaos_conf_archlinux_orgError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(Sigmaos_conf_archlinux_orgError::InvalidInput("name must not be empty".into()));
        }
        Ok(Self { name })
    }

    /// Component identifier.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Perform the component's primary action. Returns a deterministic result.
    pub fn run(&self) -> Result<String, Sigmaos_conf_archlinux_orgError> {
        if self.name.is_empty() {
            return Err(Sigmaos_conf_archlinux_orgError::InvalidState);
        }
        Ok(format!("{}: ok", self.name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rejects_empty_name() {
        // Arrange / Act / Assert
        assert!(matches!(Sigmaos_conf_archlinux_org::new("  "), Err(Sigmaos_conf_archlinux_orgError::InvalidInput(_))));
    }

    #[test]
    fn run_returns_deterministic_result() {
        let c = Sigmaos_conf_archlinux_org::new("test").unwrap();
        assert_eq!(c.run().unwrap(), "test: ok");
    }

    #[test]
    fn name_is_accessible() {
        let c = Sigmaos_conf_archlinux_org::new("abc").unwrap();
        assert_eq!(c.name(), "abc");
    }
}
