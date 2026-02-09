#![expect(unused)]
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

use crate::escapes::{unescape, unescape_string};
use crate::int::parse_int;
pub use logos::{Lexer, Logos};
use std::borrow::Cow;
use std::fmt::Display;
use utils::{Spanned, VarInt};

mod display;
mod escapes;
mod int;

pub type SToken<'s> = Spanned<Token<'s>>;


#[derive(Debug, Clone, Logos, PartialEq)]
#[logos(skip "[ \r\n\t]+")]
// Comments should be stripped by the preprocessor so we'll ignore them for now
pub enum Token<'s> {
    // // keywords
    // #[token("else")]
    // /// else
    // Else,
    // #[token("for")]
    // /// for
    // For,
    // #[token("if")]
    // /// if
    // If,
    #[token("return")]
    /// return
    Return,
    // #[token("while")]
    // /// while
    // While,
    // #[token("break")]
    // /// break
    // Break,
    // #[token("continue")]
    // /// continue
    // Continue,

    // keyword types
    #[token("void")]
    /// void
    Void,
    #[token("int")]
    /// int
    Int,
    // #[token("float")]
    // /// float
    // Float,
    // #[token("double")]
    // /// double
    // Double,

    // #[regex("\"", unescape_string)]
    // StringLit(Cow<'s, str>),
    // #[regex(r"'[^']'", |lex| {
    //     let text = lex.slice();
    //     text[1..].chars().next().unwrap()
    // })]
    // #[regex(r"'\\[^']'", (|lex: &mut Lexer<'s, Token<'s>>| -> Option<char> {
    //     let text = lex.slice();
    //     let text = &text[2..text.len() - 1];
    //     let mut chars = text.chars();
    //     let c = chars.next().unwrap();
    //     unescape(c)
    // }))]
    // CharLiteral(char),
    #[regex(r"0x[0-9a-fA-F][0-9a-fA-F_]*", |lex| parse_int(16, &lex.slice()[2..]))]
    #[regex(r"0o[0-9a-fA-F][0-9a-fA-F_]*", |lex| parse_int(8, &lex.slice()[2..]))]
    #[regex(r"0p[0-9a-fA-F][0-9a-fA-F_]*", |lex| parse_int(2, &lex.slice()[2..]))]
    #[regex(r"[0-9][0-9a-fA-F_]*", |lex| parse_int(10, lex.slice()))]
    IntLit(VarInt),
    // #[regex(r"\.\d+", |lex| lex.slice().parse().ok())]
    // #[regex(r"\d+\.\d+", |lex| lex.slice().parse().ok())]
    // FloatLit(f64),
    #[regex(r"[A-Za-z_][A-Za-z0-9_]*")]
    Ident(&'s str),

    // #[token("==")]
    // /// ==
    // EqEq,
    // #[token("!=")]
    // /// !=
    // Ne,
    // #[token("<=")]
    // /// <=
    // Le,
    // #[token(">=")]
    // /// =>
    // Ge,
    // #[token("<")]
    // /// <
    // Lt,
    // #[token(">")]
    // /// >
    // Gt,
    // #[token("+")]
    // /// +
    // Plus,
    // #[token("-")]
    // /// -
    // Minus,
    // #[token("*")]
    // /// *
    // Star,
    // #[token("=")]
    // /// =
    // Eq,
    // #[token("!")]
    // /// !
    // Not,
    // #[token("/")]
    /// /
    // Slash,
    #[token("(")]
    /// (
    LParen,
    #[token(")")]
    /// )
    RParen,
    #[token("{")]
    /// {
    LBrace,
    #[token("}")]
    /// }
    RBrace,
    // #[token(",")]
    // /// ,
    // Comma,
    #[token(";")]
    /// ;
    Semicolon,
    // #[token(".")]
    // /// .
    // Dot,
}

#[cfg(test)]
mod tests {
    use logos::Logos;
    use pretty_assertions::assert_eq;
    use std::borrow::Cow;

    use crate::Token;

    // fn string<'s>(text: impl Into<Cow<'s, str>>) -> Token<'s> {
    //     Token::StringLit(text.into())
    // }

    // #[test]
    // fn lex_example() {
    //     use Token::*;
    //     let example = "";
    //     let tokens: Vec<_> = Token::lexer(example).collect::<Result<_, _>>().unwrap();
    //     assert_eq!(
    //         &tokens,
    //         [
    //             // Your first Lox program!
    //             // print "\"Hello, world\"!";
    //             Ident("print"),
    //             LParen,
    //             string("\"Hello, world\"!"),
    //             RParen,
    //             Semicolon
    //         ]
    //         .as_slice()
    //     );
    // }
}
