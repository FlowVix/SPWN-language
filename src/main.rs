#![allow(clippy::type_complexity)]
#![deny(unused_must_use)]

use std::rc::Rc;

use colored::Colorize;
use compiler::Compiler;
use errors::ErrorGuaranteed;
use lasso::Rodeo;
use lexer::Lexer;
use logos::Logos;
use parser::Parser;
use session::Session;
use source::{BytecodeMap, Source, SpwnSource};
use vm::context::Context;
use vm::{RunInfo, Vm};

mod bytecode;
mod compiler;
pub mod errors;
mod lexer;
mod parser;
pub mod session;
mod source;
mod util;
mod vm;

fn run_spwn(spwn_session: &mut Session, source_file: Rc<Source>) -> Result<(), ErrorGuaranteed> {
    let src = source_file
        .src
        .as_ref()
        .expect("no source provided with source file");

    let lexer = Lexer::new(src, source_file.id);
    let mut parser = Parser::new(lexer, spwn_session);
    let ast = parser.parse()?;

    let mut compiler = Compiler::new(spwn_session);
    compiler.compile(&ast)?;

    for (k, v) in spwn_session.bytecode_map.iter() {
        v.debug();
    }

    // TOOD: fix...
    // let start_info = RunInfo::from_start(spwn_session);

    // let mut vm = Vm::new(spwn_session);

    // vm.run_func(Context::new(), start_info);

    Ok(())
}

fn main() {
    let source = SpwnSource::File("test.spwn".into());
    let mut spwn_session = Session::new_standard(source.clone(), vec![]);

    let source = spwn_session
        .source_map
        .new_source_file(source)
        .expect("failed to read spwn source file");

    // all errors will have been printed by now
    if run_spwn(&mut spwn_session, source).is_err() {
        std::process::exit(1);
    }

    // println!("{:#?}", bytecode_map);
}
