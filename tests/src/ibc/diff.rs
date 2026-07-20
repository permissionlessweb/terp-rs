//! Diff report types for predict vs observe / invariant checks.

/// Severity of a diff item.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffSeverity {
    Error,
    Warn,
    Info,
}

/// One finding from validation or compare.
#[derive(Debug, Clone)]
pub struct DiffItem {
    pub severity: DiffSeverity,
    pub code: String,
    pub message: String,
    pub path: Option<String>,
}

/// Collection of diff items with helpers for fail-closed behavior.
#[derive(Debug, Clone, Default)]
pub struct DiffReport {
    pub items: Vec<DiffItem>,
}

impl DiffReport {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, severity: DiffSeverity, code: impl Into<String>, message: impl Into<String>) {
        self.items.push(DiffItem {
            severity,
            code: code.into(),
            message: message.into(),
            path: None,
        });
    }

    pub fn push_at(
        &mut self,
        severity: DiffSeverity,
        code: impl Into<String>,
        message: impl Into<String>,
        path: impl Into<String>,
    ) {
        self.items.push(DiffItem {
            severity,
            code: code.into(),
            message: message.into(),
            path: Some(path.into()),
        });
    }

    pub fn extend(&mut self, other: DiffReport) {
        self.items.extend(other.items);
    }

    pub fn has_errors(&self) -> bool {
        self.items
            .iter()
            .any(|i| i.severity == DiffSeverity::Error)
    }

    /// Process exit code: 1 if any Error, else 0.
    pub fn exit_code(&self) -> i32 {
        if self.has_errors() { 1 } else { 0 }
    }

    pub fn errors(&self) -> impl Iterator<Item = &DiffItem> {
        self.items
            .iter()
            .filter(|i| i.severity == DiffSeverity::Error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exit_code_fail_closed() {
        let mut r = DiffReport::new();
        assert_eq!(r.exit_code(), 0);
        r.push(DiffSeverity::Warn, "w", "warn");
        assert_eq!(r.exit_code(), 0);
        r.push(DiffSeverity::Error, "e", "err");
        assert!(r.has_errors());
        assert_eq!(r.exit_code(), 1);
    }
}
