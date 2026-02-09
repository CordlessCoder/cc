// This file is part of CCcc, A simple x86-64 compiler for a tiny subset of C.
// Copyright (C) 2026 CordlessCoder
//
// CCcc is free software: you can redistribute it and/or modify it under the terms
// of the GNU General Public License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any later version.
//
// CCcc is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with CCcc.
// If not, see <https://www.gnu.org/licenses/>.

//! The main idea behind the diagnostic structure is that an error message can be composed of
//! multiple [`ErrorComponent`]s.
//! For example, an error message like:
//!
//!```rust,ignore
//!error[E0308]: mismatched types
//!   --> src/main.rs:2:18
//!    |
//! 2  |     let x: i32 = "42";
//!    |                  ^^^^ expected `i32`, found `&str`
//! ```
pub mod render;

// Errors can occur during:
// - Lexing
// - Parsing
// - Type checking
// - Codegen
// - Execution
//
// Everything that isn't execution can be a simple error message + point to source
//
// Errors during execution likely need extra information to describe the runtime context

use source::{SourceFile, Span};
#[derive(Debug, Clone)]
pub struct AggregateError {
    pub components: Vec<ErrorComponent>,
}

impl AggregateError {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.components.is_empty()
    }
    #[must_use]
    pub fn has_error(&self) -> bool {
        self.components.iter().any(|c| c.level == ErrorLevel::Error)
    }
    pub fn add_error(&mut self, component: ErrorComponent) -> &mut ErrorComponent {
        self.components.push(component);
        self.components.last_mut().unwrap()
    }
    pub fn append(&mut self, other: &mut AggregateError) {
        self.components.append(&mut other.components);
    }
}

impl Default for AggregateError {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorLevel {
    Error,
    Warning,
}

#[derive(Debug, Clone)]
pub struct ErrorComponent {
    pub level: ErrorLevel,
    pub short_message: String,
    pub long_message: String,
    source: SourceFile,
    highlight: Span,
    highlight_message: Option<String>,
}

impl ErrorComponent {
    #[must_use]
    pub const fn new(source: SourceFile, short_message: String, span: Span) -> Self {
        Self {
            short_message,
            level: ErrorLevel::Error,
            long_message: String::new(),
            source,
            highlight: span,
            highlight_message: None,
        }
    }
    pub fn set_highlight_message(&mut self, message: String) -> &mut Self {
        self.highlight_message = Some(message);
        self
    }
    pub const fn set_level(&mut self, level: ErrorLevel) -> &mut Self {
        self.level = level;
        self
    }
    pub fn set_long_message(&mut self, message: String) -> &mut Self {
        self.long_message = message;
        self
    }
}
