// LICENSE NOTICE START
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
// LICENSE NOTICE END

use asm::{ASMFunction, ASMProgram, Instruction, Operand, Register};
use ast::{Decl, Expr, Function, LiteralExpression, Program, Stmt};

#[derive(Debug, Default)]
pub struct Codegen {}

impl Codegen {
    pub fn new() -> Self {
        Self {  }
    }
    pub fn codegen_program<'s>(&self, program: &Program<'s>) -> ASMProgram<'s> {
        let Program { declarations } = program;
        let mut functions = Vec::new();
        for decl in declarations {
            match decl {
                Decl::Fun(fun) => functions.push(self.codegen_function(fun)),
            }
        }
        ASMProgram { functions }
    }
    pub fn codegen_function<'s>(&self, function: &Function<'s>) -> ASMFunction<'s> {
        let Function { name, body } = function;
        let mut instructions = Vec::new();
        for statement in &body.0 {
            self.codegen_statement(&mut instructions, statement);
        }
        ASMFunction { name, instructions }
    }
    pub fn codegen_statement<'s>(&self, instructions: &mut Vec<Instruction>, stmt: &Stmt<'s>) {
        match stmt {
            Stmt::Return(val) => {
                if let Some(ret) = val {
                    match ret {
                        Expr::Lit(LiteralExpression::Int(int)) => {
                            instructions.push(Instruction::Mov {
                                from: Operand::Imm(*int),
                                to: Operand::Reg(Register::EAX),
                            })
                        }
                        _ => unimplemented!()
                    }
                }
                instructions.push(Instruction::Ret);
            }
        }
    }
}
