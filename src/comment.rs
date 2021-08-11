use std::fmt::{self, Write};

use crate::formatter::Formatter;

/// a comment block
#[derive(Debug, Clone)]
pub struct Comment {
    comment: String,
}

impl Comment {
    pub fn new(comment: &str) -> Self {
        Self {
            comment: comment.to_string(),
        }
    }

    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        for line in self.comment.lines() {
            writeln!(fmt, "// {}", line)?;
        }

        Ok(())
    }
}
