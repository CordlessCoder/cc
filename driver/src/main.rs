#![expect(unused)]
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

use ast::tree::{TreeCtx, TreeDisplay};
use codegen::Codegen;
use diagnostics::{
    AggregateError, ErrorComponent,
    render::{RenderContext, RenderableError},
};
use lexer::{Logos, SToken};

use std::{
    ffi::{OsStr, OsString},
    fs::File,
    io::{self, stdout, Write},
    path::PathBuf,
    process::Command,
};
use std::{fmt, path::Path};

struct FmtToIoWrite<W: io::Write>(pub W);
impl<W: io::Write> fmt::Write for FmtToIoWrite<W> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write_all(s.as_bytes()).map_err(|_| fmt::Error)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Stage {
    Lex,
    Parse,
    Codegen,
    Assemble,
    #[default]
    Link,
}

#[derive(Debug, Default, Clone)]
struct Config {
    stop_at_stage: Stage,
}

impl Config {
    fn from_flags<S: AsRef<OsStr>>(args: impl Iterator<Item = S>) -> Self {
        let mut config = Config::default();
        for flag in args {
            config.stop_at_stage = match &flag.as_ref().as_encoded_bytes()[1..] {
                b"-lex" => Stage::Lex,
                b"-parse" => Stage::Parse,
                b"-codegen" => Stage::Codegen,
                b"S" => Stage::Assemble,
                _ => continue,
            }
        }
        config
    }
}

struct Driver {
    config: Config,
}

impl Driver {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    fn run(&mut self, paths: Vec<OsString>) -> AggregateError {
        let mut errors = AggregateError::new();
        for path in paths {
            let input_path = PathBuf::from(path);
            // let input_file = File::open(&path).expect("the input file should be accessible");
            let preprocessed_path = input_path.with_added_extension("i");
            Command::new("gcc")
                .args(["-E", "-P"])
                .arg(&input_path)
                .arg("-o")
                .arg(&preprocessed_path)
                .spawn()
                .unwrap()
                .wait()
                .unwrap();

            let source = source::SourceFile::new(
                input_path.to_string_lossy().to_string(),
                std::fs::read_to_string(&preprocessed_path).unwrap(),
            );
            let lexer = lexer::Token::lexer(source.text())
                .spanned()
                .map(|r| match r {
                    (Ok(token), span) => Ok(SToken::new(token, span)),
                    (Err(()), span) => Err(ErrorComponent::new(
                        source.clone(),
                        String::from("Failed to lex token"),
                        span,
                    )),
                });
            if self.config.stop_at_stage == Stage::Lex {
                for error in lexer.filter_map(Result::err) {
                    errors.add_error(error);
                }
                continue;
            }
            let mut parser = parser::Parser::new(source.clone(), lexer);
            let (program, mut parser_errors) = parser.parse();
            errors.append(&mut parser_errors);

            let writer = stdout();
            let mut writer = FmtToIoWrite(writer);
            let mut ctx = TreeCtx::new();
            program.fmt_tree(&mut ctx, &mut writer).unwrap();

            if !errors.is_empty() {
                continue;
            }

            if self.config.stop_at_stage == Stage::Parse {
                continue;
            }

            let codegen = Codegen::new();
            let asm = codegen.codegen_program(&program);
            dbg!(&asm);

            if self.config.stop_at_stage == Stage::Codegen {
                continue;
            }

            let assembled_path = input_path.with_added_extension("S");
            let executable_path = input_path.with_extension("");

            let mut asm_file = File::create(&assembled_path).unwrap();
            writeln!(&mut asm_file, "{asm}").unwrap();
            asm_file.flush().unwrap();

            if self.config.stop_at_stage == Stage::Assemble {
                continue;
            }

            Command::new("gcc")
                .arg(&assembled_path)
                .arg("-o")
                .arg(&executable_path)
                .spawn()
                .unwrap()
                .wait()
                .unwrap();

            // let mut vm = LoxVm::default();
            // dbg!(vm.run(&program).unwrap());
        }
        errors
    }
}

fn main() {
    let mut args: Vec<OsString> = std::env::args_os().skip(1).collect();
    let flags = args.extract_if(.., |arg| arg.as_encoded_bytes().starts_with(b"-"));
    let config = Config::from_flags(flags);

    let mut driver = Driver::new(config);
    let errors = driver.run(args);
    if !errors.is_empty() {
        let render_context = RenderContext::default();
        eprint!("{}", errors.display(render_context));
        std::process::exit(1);
    }
}
