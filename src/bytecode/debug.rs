use colored::Colorize;
use itertools::Itertools;
use regex::{Captures, Regex};

use super::{Bytecode, CallExpr};
use crate::source::CodeSpan;
use crate::util::clear_ansi;

macro_rules! rows {
    (
        $(
            $($name:ident: $right_align:literal)?
            $($lit:literal)?
            ,
        )*
    ) => {
        struct TableRow {
            $(
                $($name: String,)?
            )*
        }
        #[derive(Debug)]
        struct TableRowMax {
            $(
                $($name: usize,)?
            )*
        }

        impl TableRowMax {
            pub fn from(v: &[TableRow]) -> Self {
                Self {
                    $(
                        $($name: v.iter().map(|v| clear_ansi(&v.$name).len()).max().unwrap_or(2),)?
                    )*
                }
            }
            pub fn width(&self) -> usize {
                [
                    $(
                        $(
                            $lit.chars().count(),
                        )?
                        $(
                            self.$name,
                        )?
                    )*
                ].into_iter().sum()
            }
        }
        impl TableRow {
            pub fn display(&self, max: &TableRowMax) -> String {
                let mut s = String::new();
                $(
                    $(
                        s += &$lit.bright_white().to_string();
                    )?
                    $(
                        s += &{
                            let pad = max.$name + self.$name.len() - clear_ansi(&self.$name).len();
                            if $right_align {
                                format!("{:>pad$}", &self.$name)
                            } else {
                                format!("{:<pad$}", &self.$name)
                            }
                        };
                    )?
                )*
                s
            }
        }
    };
}

rows! {
    "│ ",
    idx: true,
    "  ",
    opcode_name: true,
    "  ",
    opcode_str: false,
    "     ",
    span: true,
    " ",
    snippet: false,
    " │",
}

fn debug_call_expr(c: &CallExpr) -> String {
    c.positional
        .iter()
        .map(|v| if *v { "mut _" } else { "_" })
        .join(", ")
}

impl Bytecode {
    pub fn debug(&self) {
        let const_regex = Regex::new(r"ConstID\((\d+)\)").unwrap();
        let call_expr_regex = Regex::new(r"CallExprID\((\d+)\)").unwrap();
        let var_regex = Regex::new(r"VarID\((\d+)\)").unwrap();
        let func_regex = Regex::new(r"FuncID\((\d+)\)").unwrap();
        let jump_pos_regex = Regex::new(r"JumpPos\((\d+)\)").unwrap();

        let code = self.src.read().unwrap();

        println!(
            "\n{}",
            format!(
                "============================ {} ============================",
                self.src.name().italic()
            )
            .bright_white()
            .bold()
        );
        println!(
            "{} {}",
            "Consts:".dimmed(),
            self.consts
                .iter()
                .map(|v| v.to_string().bright_green())
                .join(", ")
        );
        println!();

        for (fn_idx, func) in self.funcs.iter().enumerate() {
            let rows = func
                .opcodes
                .iter()
                .enumerate()
                .map(|(idx, &(opcode, span))| TableRow {
                    idx: format!("{}.", idx).dimmed().to_string(),
                    opcode_name: Into::<&str>::into(opcode).bright_white().bold().to_string(),
                    opcode_str: {
                        let c = format!("{}", opcode);
                        let c = var_regex.replace_all(&c, "V$1".bright_yellow().to_string());
                        let c = func_regex.replace_all(&c, "F$1".bright_magenta().to_string());
                        let c = const_regex.replace_all(&c, |c: &Captures| {
                            let id = c.get(1).unwrap().as_str().parse::<usize>().unwrap();
                            format!("{}", self.consts[id]).bright_green().to_string()
                        });
                        let c = call_expr_regex.replace_all(&c, |c: &Captures| {
                            let id = c.get(1).unwrap().as_str().parse::<usize>().unwrap();
                            debug_call_expr(&self.call_exprs[id])
                        });
                        let c = jump_pos_regex.replace_all(&c, "$1".bright_blue().to_string());
                        c.bright_white().to_string()
                    },
                    span: format!("{:?}", span).dimmed().to_string(),
                    snippet: {
                        let snippet = format!("{:?}", &code[span.start..span.end]);
                        let snippet = &snippet[1..(snippet.len() - 1)];
                        if snippet.len() < 16 {
                            snippet.bright_cyan().to_string()
                        } else {
                            let start = &snippet[..7];
                            let end = &snippet[(snippet.len() - 7)..];
                            format!(
                                "{}{}{}",
                                start.bright_cyan(),
                                "...".bright_cyan().dimmed(),
                                end.bright_cyan()
                            )
                        }
                    },
                })
                .collect_vec();
            let max = TableRowMax::from(&rows);
            let width = max.width();

            let fn_title = format!("Function {}", fn_idx);
            println!(
                "{}",
                format!(
                    "╭─────── {} {}╮",
                    fn_title.italic(),
                    "─".repeat(width - 11 - fn_title.len())
                )
                .bright_white()
            );
            for row in rows {
                println!("{}", row.display(&max));
            }
            println!("{}", format!("├{}╯", "─".repeat(width - 2)).bright_white());

            let extra = [
                ("var count", func.var_count.to_string()),
                (
                    "args",
                    func.args
                        .iter()
                        .map(|arg| {
                            let name = arg
                                .name
                                .as_ref()
                                .map(|v| v.clear())
                                .unwrap_or(r"\".bright_red());
                            let is_mut = if arg.needs_mut {
                                "mut ".bright_magenta()
                            } else {
                                "".clear()
                            };
                            format!("{}{}", is_mut, name)
                        })
                        .join(", "),
                ),
            ];
            for (n, v) in extra {
                println!("│ {} {}", format!("{n}:").dimmed(), v.bright_white());
            }

            println!("{}", "╰──────────────────────╼".bright_white())
        }
    }
}
