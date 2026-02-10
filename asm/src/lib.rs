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

pub mod generate;

use utils::VarInt;

#[derive(Debug, Clone)]
pub struct ASMProgram<'s> {
    pub functions: Vec<ASMFunction<'s>>,
}

#[derive(Debug, Clone)]
pub struct ASMFunction<'s> {
    pub name: &'s str,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Mov { from: Operand, to: Operand },
    Ret,
}

#[derive(Debug, Clone)]
pub enum Operand {
    Reg(Register),
    Imm(VarInt),
}

#[derive(Debug, Clone)]
pub enum Register {
    EAX,
}
