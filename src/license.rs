use std::fmt::{self, Write};

use crate::formatter::Formatter;

/// represents the type of the license
#[derive(Debug, Clone)]
pub enum LicenseType {
    /// MIT License
    Mit,
    /// BSD License
    Bsd,
}

const MIT_LICENSE_TEXT: &'static str = "MIT License

{}

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the \"Software\"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.";

const BSD_LICENSE_TEXT: &'static str = "";

/// represents the license information
#[derive(Debug, Clone)]
pub struct License {
    copyrights: Vec<String>,
    title: String,
    ty: LicenseType,
}

impl License {
    /// constructor for a license object
    pub fn new(title: &str, ty: LicenseType) -> Self {
        Self {
            title: title.to_string(),
            copyrights: Vec::new(),
            ty,
        }
    }

    /// adds copy right information
    pub fn add_copyright(&mut self, copyright: &str) -> &mut Self {
        self.copyrights.push(String::from(copyright));
        self
    }

    /// formatting of the license
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if !self.title.is_empty() {
            writeln!(fmt, "// {}", self.title)?;
            writeln!(fmt, "//")?;
        }

        let lictext = match self.ty {
            LicenseType::Mit => MIT_LICENSE_TEXT,
            LicenseType::Bsd => BSD_LICENSE_TEXT,
        };

        for line in lictext.lines() {
            if line == "{}" {
                for c in &self.copyrights {
                    writeln!(fmt, "// Copyright (c) {}", c)?;
                }
            } else {
                writeln!(fmt, "// {}", line)?;
            }
        }

        let spdx = match self.ty {
            LicenseType::Mit => "MIT",
            LicenseType::Bsd => "BSD",
        };
        writeln!(fmt, "//")?;
        writeln!(fmt, "// SPDX-License-Identifier: {}", spdx)?;
        writeln!(fmt, "//\n")?;
        Ok(())
    }
}
