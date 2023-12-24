#![deny(unused_must_use)]
#![deny(unused_must_use, clippy::nonstandard_macro_braces)]
#![allow(clippy::too_many_arguments, clippy::type_complexity)]
#![warn(clippy::branches_sharing_code)]
#![allow(unknown_lints)]
#![allow(clippy::should_implement_trait)]

use std::io::Write;
use std::path::PathBuf;
use std::rc::Rc;

use colored::Colorize;
use lasso::Rodeo;

use crate::lexer::Lexer;
pub mod bytecode;
pub mod error;
pub mod gd;
pub mod lexer;
pub mod parser;
pub mod source;
pub mod util;
pub mod vm;

fn main() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().unwrap();

    let mut interner = Rodeo::default();

    let src = source::SpwnSource::File(PathBuf::from("test.spwn"));
    let src = Rc::new(src);

    let code = src.read().unwrap();
    let mut lexer = Lexer::new(&code);

    loop {
        let g = lexer.next();

        let g = match g {
            Ok(Some(t)) => Ok(t),
            Err(err) => Err(err),
            Ok(None) => break,
        };

        println!(
            "{:?} {} {}",
            g,
            lexer.span(),
            lexer.slice().bright_blue().underline()
        );
    }

    // let mut parser = Parser::new(lexer, &src, &mut interner);

    // let stmts = match parser.parse() {
    //     Ok(v) => {
    //         // println!("{}", format!("{:#?}", v).bright_green().bold());
    //         v
    //     },
    //     Err(err) => {
    //         err.into_report().display();
    //         std::process::exit(1);
    //     },
    // };

    // let mut src_map = SourceMap::new();

    // match Compiler::new_compile_file(
    //     &stmts,
    //     &src,
    //     &mut interner,
    //     &mut src_map,
    //     (0..code.len()).into(),
    // ) {
    //     Ok(c) => {
    //         src_map.insert(src.clone(), c.build(&src));
    //     },
    //     Err(err) => {
    //         err.into_report().display();
    //         std::process::exit(1);
    //     },
    // };

    // let program = src_map
    //     .into_iter()
    //     .map(|(_, b)| b)
    //     .collect_vec()
    //     .into_boxed_slice();

    // for code in program.iter() {
    //     code.display();
    // }

    // let mut vm = Vm {
    //     memory: SlabMap::new(),
    // };
    // println!("{}", "------------------------------".dimmed());
    // match vm.run_func(
    //     RunInfo {
    //         program: &program,
    //         bytecode_idx: program.len() - 1,
    //         func_idx: 0,
    //     },
    //     dummy,
    // ) {
    //     Ok(_) => {
    //         // println!(
    //         //     "{} {}",
    //         //     "Stack:".bright_blue().bold(),
    //         //     vm.stack
    //         //         .iter()
    //         //         .map(|v| vm.memory[*v].value.to_str(&vm))
    //         //         .join(&", ".dimmed().to_string())
    //         // );
    //     },
    //     Err(err) => {
    //         err.into_report().display();
    //         std::process::exit(1);
    //     },
    // }
}
