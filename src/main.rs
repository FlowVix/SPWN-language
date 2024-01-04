#![allow(clippy::type_complexity)]
#![deny(unused_must_use)]

use colored::Colorize;
use compiler::Compiler;
use lasso::Rodeo;
use lexer::Lexer;
use logos::Logos;
use parser::Parser;
use source::{BytecodeMap, SpwnSource};
use vm::context::Context;
use vm::{RunInfo, Vm};

mod bytecode;
mod compiler;
mod error;
mod lexer;
mod parser;
mod source;
mod util;
mod vm;

fn main() {
    // let guh = "bob".bright_red().clear();
    // println!("{}", guh.len());

    let src = Box::leak(Box::new(SpwnSource::File("test.spwn".into())));
    let code = src.read().unwrap();

    let mut interner = Rodeo::new();

    let mut parser = Parser::new(Lexer::new(&code), src, &mut interner);

    let ast = match parser.parse() {
        Ok(v) => v,
        Err(err) => {
            println!("{}", err.into_report());
            std::process::exit(1);
        },
    };

    let bytecode_map = Box::leak(Box::new(BytecodeMap::new()));
    let mut compiler = Compiler::new(src, &mut interner, bytecode_map);

    match compiler.compile(&ast) {
        Ok(_) => {},
        Err(err) => {
            println!("{}", err.into_report());
            std::process::exit(1);
        },
    }
    for (k, v) in bytecode_map.iter() {
        v.debug();
    }

    let mut vm = Vm {};

    let start_info = RunInfo {
        bytecode: &bytecode_map[src],
        function: &bytecode_map[src].funcs[0],
    };

    vm.run_func(Context::new(), start_info);

    // println!("{:#?}", bytecode_map);
}
