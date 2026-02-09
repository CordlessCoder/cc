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

use ast::{Block, Decl, Expr, Function, LiteralExpression, Program, Stmt};
use diagnostics::{AggregateError, ErrorComponent};
use lexer::{SToken, Token};
use source::SourceFile;
use std::{collections::VecDeque, rc::Rc};

// use crate::expr::BindingPower;

mod basic_ops;
mod expr;

pub struct Parser<'s, Tokens: Iterator> {
    tokens: Tokens,
    source: SourceFile,
    peeked: VecDeque<SToken<'s>>,
    lexer_errors: AggregateError,
    errors: AggregateError,
}

// If a parsing function returns None, an error occurred and we must synchronize to try to
// continue parsing
impl<'s, Tokens: Iterator<Item = Result<SToken<'s>, ErrorComponent>>> Parser<'s, Tokens> {
    // TODO: The thing
    pub fn post_error_sync(&mut self) {
        let Some(mut prev) = self.advance() else {
            return;
        };
        while let Some(tok) = self.advance() {
            use Token::*;
            if matches!(prev.inner, Token::Semicolon | Int | Return) {
                self.put_back(tok);
                return;
            }
            prev = tok;
        }
    }
    pub fn parse_stmt(&mut self) -> Option<Stmt<'s>> {
        self.expect(&Token::Return, " as the sole statement")?;
        let (Some(Token::IntLit(int)), _) = self.advance_split() else {
            todo!()
        };
        self.expect(&Token::Semicolon, " to terminate return")?;
        Some(Stmt::Return(Some(Expr::Lit(LiteralExpression::Int(int)))))
    }
    pub fn parse_block(&mut self) -> Option<Block<'s>> {
        self.expect(&Token::LBrace, " to start function body")?;
        let mut statements = Vec::new();
        while !self.consume_if_eq(&Token::RBrace) {
            statements.push(self.parse_stmt()?);
        }
        Some(Block(statements))
    }
    pub fn parse_function(&mut self) -> Option<Function<'s>> {
        self.expect(&Token::Int, " to start function declaration")?;
        let name = self.expect_ident(" after int type for function declaration")?;
        self.expect(&Token::LParen, " to start function parameter list")?;
        self.expect(&Token::Void, " in function parameter list")?;
        self.expect(&Token::RParen, " to end function parameter list")?;
        let body = self.parse_block()?;
        Some(Function { name, body })
    }
    pub fn parse_decl(&mut self) -> Option<Decl<'s>> {
        Some(Decl::Fun(self.parse_function()?))
    }
    pub fn parse(&mut self) -> (Program<'s>, AggregateError) {
        let mut declarations = Vec::new();
        while self.peek_next().is_some() {
            let Some(stmt) = self.parse_decl() else {
                self.post_error_sync();
                continue;
            };
            declarations.push(stmt);
        }
        let components: Vec<ErrorComponent> = self
            .lexer_errors
            .components
            .drain(..)
            .chain(self.errors.components.drain(..))
            .collect();
        (Program { declarations }, AggregateError { components })
    }
}
