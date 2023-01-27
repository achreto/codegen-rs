use crate::formatter::Formatter;
use crate::r#type::Type;
use std::fmt::{self, Write};

/// Defines a constant
#[derive(Debug, Clone)]
pub struct Const {
    /// Constant name
    pub name: String,

    /// Constant type
    pub ty: Type,

    /// Constant visibility
    pub vis: Option<String>,

    /// Constant documentation
    pub documentation: Vec<String>,

    /// Constant value
    pub value: String,
}

impl Const {
    /// Return a constant definition with the provided name, type, and value
    pub fn new<T>(name: &str, ty: T, value: &str) -> Self
    where
        T: Into<Type>,
    {
        Self {
            name: name.into(),
            ty: ty.into(),
            vis: None,
            documentation: Vec::new(),
            value: value.into(),
        }
    }

    /// Set constants's documentation.
    pub fn doc(&mut self, documentation: Vec<&str>) -> &mut Self {
        self.documentation = documentation.iter().map(|doc| doc.to_string()).collect();
        self
    }

    /// Set the constant's visibility
    pub fn vis(&mut self, vis: &str) {
        self.vis = Some(String::from(vis));
    }

    /// Formats the struct using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        // write the documentation
        for doc in &self.documentation {
            writeln!(fmt, "/// {doc}")?;
        }

        // write the name
        match &self.vis {
            Some(v) => write!(fmt, "{} const {} : ", v, self.name),
            None => write!(fmt, "const {} : ", self.name),
        }?;

        // write the type information
        self.ty.fmt(fmt)?;

        // write the value
        writeln!(fmt, " = {};", self.value)?;

        Ok(())
    }
}
