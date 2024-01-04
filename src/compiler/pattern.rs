use super::builder::CodeBuilder;
use super::{CompileResult, Compiler, ScopeID, VarData};
use crate::bytecode::constant::Constant;
use crate::parser::ast::pattern::{PatternNode, PatternType};

impl<'a> Compiler<'a> {
    pub fn compile_pattern_check(
        &mut self,
        pattern: &PatternNode,
        force_new_var: bool,
        scope: ScopeID,
        builder: &mut CodeBuilder,
    ) -> CompileResult<()> {
        match &*pattern.typ {
            PatternType::Any => {
                builder.load_const(Constant::Bool(true), pattern.span);
            },
            PatternType::Type(_) => todo!(),
            PatternType::Either(..) => todo!(),
            PatternType::Both(..) => todo!(),
            PatternType::Eq(_) => todo!(),
            PatternType::NEq(_) => todo!(),
            PatternType::Lt(_) => todo!(),
            PatternType::LtE(_) => todo!(),
            PatternType::Gt(_) => todo!(),
            PatternType::GtE(_) => todo!(),
            PatternType::In(_) => todo!(),
            PatternType::ArrayPattern(..) => todo!(),
            PatternType::DictPattern(_) => todo!(),
            PatternType::ArrayDestructure(_) => todo!(),
            PatternType::DictDestructure(_) => todo!(),
            PatternType::MaybeDestructure(_) => todo!(),
            PatternType::InstanceDestructure(..) => todo!(),
            PatternType::Empty => todo!(),
            PatternType::Path { var, path } => {
                if path.is_empty() {
                    let var_id = match self.get_var(*var, scope) {
                        Some(info) => info.id,
                        None => {
                            let var_id = builder.next_var();
                            self.scopes[scope].vars.insert(
                                *var,
                                VarData {
                                    mutable: false,
                                    def_span: pattern.span,
                                    id: var_id,
                                },
                            );
                            var_id
                        },
                    };
                    builder.set_var(var_id, pattern.span);
                    builder.load_const(Constant::Bool(true), pattern.span);
                } else {
                    todo!()
                }
            },
            PatternType::Mut { name } => todo!(),
            PatternType::Ref { name } => todo!(),
            PatternType::IfGuard { pat, cond } => todo!(),
            // PatternType::MacroPattern { args, ret } => todo!(),
        }
        Ok(())
    }
}
