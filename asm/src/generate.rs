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

use std::fmt::{self, Display, Write};

use crate::{ASMFunction, ASMProgram, Instruction, Operand, Register};

pub trait AssemblyRepr {
    fn generate_asm(&self, writer: &mut impl Write) -> fmt::Result;
}

const NOEXECSTACK: &str = ".section .note.GNU-stack,\"\",@progbits\n";
impl Display for ASMProgram<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for function in &self.functions {
            function.fmt(f)?;
        }
        f.write_str(NOEXECSTACK)
    }
}

const TAB: &str = "\t";

impl Display for ASMFunction<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{TAB}.globl {}", self.name)?;
        writeln!(f, "{}:", self.name)?;
        for instruction in &self.instructions {
            writeln!(f, "{TAB}{instruction}")?;
        }
        writeln!(f)
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mov { from, to } => write!(f, "movl {from},{to}"),
            Self::Ret => f.write_str("ret")
        }
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Reg(reg) => reg.fmt(f),
            Operand::Imm(num) => write!(f, "${num}"),
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Register::EAX => "%eax",
        };
        f.write_str(text)
    }
}
