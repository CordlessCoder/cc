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

use std::{fmt::Display, ops::Range};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VarInt {
    Pos(u64),
    Neg(i64),
}

impl Display for VarInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pos(p) => p.fmt(f),
            Self::Neg(n) => n.fmt(f),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub inner: T,
    pub span: Range<usize>,
}

impl<T> From<(T, Range<usize>)> for Spanned<T> {
    fn from((inner, span): (T, Range<usize>)) -> Self {
        Self { inner, span }
    }
}

impl<T> Spanned<T> {
    #[must_use]
    pub const fn new(inner: T, span: Range<usize>) -> Self {
        Self { inner, span }
    }
    pub fn as_span(&self) -> Range<usize> {
        self.span.clone()
    }
    pub fn split(self) -> (T, Range<usize>) {
        (self.inner, self.span)
    }
}
