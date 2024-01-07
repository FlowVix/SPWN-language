#![allow(clippy::type_complexity)]
#![deny(unused_must_use)]

use colored::Colorize;
use compiler::Compiler;
use lasso::Rodeo;
use lexer::Lexer;
use logos::Logos;
use parser::Parser;
use session::Session;
use source::{BytecodeMap, SpwnSource};
use vm::context::Context;
use vm::{RunInfo, Vm};

mod bytecode;
mod compiler;
mod error;
pub mod errors;
mod lexer;
mod parser;
pub mod session;
mod source;
mod util;
mod vm;

fn main() {
    let spwn_session = Session::new_standard(SpwnSource::File("test.spwn".into()), vec![]);

    let code = spwn_session.input.read().unwrap();

    let mut parser = Parser::new(
        Lexer::new(&code),
        &spwn_session.input,
        spwn_session.interner.clone(),
    );

    let ast = match parser.parse() {
        Ok(v) => v,
        Err(err) => {
            println!("{}", err.into_report());
            std::process::exit(1);
        },
    };

    //let bytecode_map = Box::leak(Box::new(BytecodeMap::new()));
    let mut compiler = Compiler::new(
        &spwn_session.input,
        spwn_session.interner,
        &mut spwn_session.bytecode_map,
    );

    match compiler.compile(&ast) {
        Ok(_) => {},
        Err(err) => {
            println!("{}", err.into_report());
            std::process::exit(1);
        },
    }
    for (k, v) in spwn_session.bytecode_map.iter() {
        v.debug();
    }

    let mut vm = Vm {};

    let bytecode = &spwn_session.bytecode_map[&spwn_session.input];
    let start_info = RunInfo {
        bytecode: &bytecode,
        function: &bytecode.funcs[0],
    };

    vm.run_func(Context::new(), start_info);

    // println!("{:#?}", bytecode_map);
}
