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

use crate::Token;
use std::fmt::Display;

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Token::*;
        let lit = match self {
            // Else => "else",
            // For => "for",
            // If => "if",
            Return => "return",
            // While => "while",
            // Break => "break",
            // Continue => "continue",

            Void => "void",
            Int => "int",

            // BoolLit(true) => "true",
            // BoolLit(false) => "false",
            // StringLit(s) => return write!(f, "{s:?}"),
            // CharLiteral(c) => return c.fmt(f),
            IntLit(v) => return v.fmt(f),
            // FloatLit(v) => return v.fmt(f),
            Ident(i) => i,

            // EqEq => "==",
            // Ne => "!=",
            // Le => "<=",
            // Ge => ">=",
            // Plus => "+",
            // Minus => "-",
            // Star => "*",
            // Slash => "/",
            LParen => "(",
            RParen => ")",
            LBrace => "{",
            RBrace => "}",
            Semicolon => ";",
            // // Comma => ",",
            // Dot => ".",
            // Eq => "=",
            // Not => "!",
            // Gt => ">",
            // Lt => "<",
        };
        f.write_str(lit)
    }
}
