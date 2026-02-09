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

use std::{iter::FusedIterator, ops::Range};

use crate::Span;

#[derive(Debug, Clone)]
pub struct LineIterator<'s> {
    pub(crate) text: &'s str,
    pub(crate) line_starts: &'s [usize],
    pub(crate) remaining: Range<usize>,
}

#[derive(Debug, Clone)]
pub struct TextLine<'s> {
    pub text: &'s str,
    pub span: Span,
    pub line: usize,
}

impl<'s> LineIterator<'s> {
    #[must_use]
    pub fn get_line(&self, line: usize) -> Option<TextLine<'s>> {
        let start = *self.line_starts.get(line)?;
        let end = self
            .line_starts
            .get(line + 1)
            .copied()
            .unwrap_or(self.text.len());
        let span = start..end;
        let text = &self.text[span.clone()];
        Some(TextLine { text, span, line })
    }
}

impl ExactSizeIterator for LineIterator<'_> {
    fn len(&self) -> usize {
        self.remaining.len()
    }
}

impl FusedIterator for LineIterator<'_> {}

impl<'s> Iterator for LineIterator<'s> {
    type Item = TextLine<'s>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.remaining.next()?;
        self.get_line(line)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n >= self.len() {
            self.remaining.start = self.remaining.end;
            return None;
        }
        self.remaining.start += n;
        self.next()
    }
}

impl DoubleEndedIterator for LineIterator<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let line = self.remaining.next_back()?;
        self.get_line(line)
    }
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        if n >= self.len() {
            self.remaining.start = self.remaining.end;
            return None;
        }
        self.remaining.end -= n;
        self.next_back()
    }
}
