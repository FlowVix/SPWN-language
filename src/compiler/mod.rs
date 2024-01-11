use ahash::AHashMap;
use lasso::{Rodeo, Spur};

use self::builder::proto::{BlockID, ProtoBytecode};
use self::error::CompilerError;
use crate::bytecode::opcode::VarID;
use crate::errors::ErrorGuaranteed;
use crate::ids_helper;
use crate::parser::ast::pattern::PatternNode;
use crate::parser::ast::Ast;
use crate::session::Session;
use crate::source::{BytecodeMap, CodeSpan, SpwnSource};
use crate::util::interner::Interner;
use crate::util::slabmap::SlabMap;
use crate::util::ImmutStr;

pub mod builder;
pub mod error;
pub mod expr;
pub mod pattern;
pub mod stmt;
pub mod util;

pub type CompileResult<T> = Result<T, ErrorGuaranteed>;

ids_helper! {
    ScopeID(u16);
}

#[derive(Debug, Clone, Copy)]
pub struct VarData {
    mutable: bool,
    def_span: CodeSpan,
    id: VarID,
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ScopeType<'a> {
    Global,
    Loop(BlockID),
    MacroBody(Option<&'a PatternNode>), // return pattern
    TriggerFunc(CodeSpan),
    ArrowStmt(CodeSpan),
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Scope<'a> {
    vars: AHashMap<Spur, VarData>,
    parent: Option<ScopeID>,
    typ: Option<ScopeType<'a>>,
}

#[non_exhaustive]
pub struct Compiler<'a> {
    session: &'a mut Session,

    scopes: SlabMap<ScopeID, Scope<'a>>,
    // pub type_def_map: &'a mut TypeDefMap,
    // pub local_type_defs: SlabMap<LocalTypeID, Vis<TypeDef<Spur>>>,

    // pub available_custom_types: AHashMap<Spur, Vis<CustomTypeID>>,

    // deferred_trigger_func_stack: Vec<Vec<DeferredTriggerFunc>>,

    // pub deprecated_features: DeprecatedFeatures,

    // pub import_stack: &'a mut Vec<(Rc<SpwnSource>, CodeArea)>,
}

impl<'a> Compiler<'a> {
    // pub fn make_area(&'a self, span: CodeSpan) -> CodeArea {
    //     CodeArea {
    //         span,
    //         src: self.session.input,
    //     }
    // }

    pub fn new(session: &'a mut Session) -> Self {
        Self {
            session,
            scopes: SlabMap::new(),
        }
    }

    fn intern(&mut self, s: &str) -> Spur {
        self.session.interner.borrow_mut().get_or_intern(s)
    }

    pub fn resolve(&self, s: &Spur) -> &str {
        //self.session.interner.borrow().resolve(s)
        todo!()
    }

    pub fn resolve_immut(&self, s: &Spur) -> ImmutStr {
        self.session.interner.borrow().resolve(s).into()
    }

    pub fn get_var(&self, var: Spur, scope: ScopeID) -> Option<VarData> {
        match self.scopes[scope].vars.get(&var) {
            Some(v) => Some(*v),
            None => match self.scopes[scope].parent {
                Some(p) => self.get_var(var, p),
                None => None,
            },
        }
    }

    pub fn get_var_or_err(
        &mut self,
        var: Spur,
        scope: ScopeID,
        span: CodeSpan,
    ) -> CompileResult<VarData> {
        self.get_var(var, scope).ok_or_else(|| {
            self.session
                .diag_ctx
                .emit_error(CompilerError::NonexistentVariable {
                    span,
                    var: self.resolve_immut(&var),
                })
        })
    }

    pub fn derive_scope(&mut self, scope: ScopeID, typ: Option<ScopeType<'a>>) -> ScopeID {
        let scope = Scope {
            vars: AHashMap::new(),
            parent: Some(scope),
            typ,
        };
        self.scopes.insert(scope)
    }

    pub fn compile(&mut self, ast: &'a Ast) -> CompileResult<()> {
        let mut code = ProtoBytecode::new();

        code.new_func(
            |builder| {
                let base_scope = self.scopes.insert(Scope {
                    vars: Default::default(),
                    parent: None,
                    typ: Some(ScopeType::Global),
                });

                self.compile_stmts(&ast.statements, base_scope, builder)?;

                Ok(())
            },
            vec![],
        )?;

        // end compilation if we have errors that were not propogated from other functions
        if let Some(errors) = self.session.diag_ctx.abort_if_errors() {
            return Err(errors);
        }

        let code = code.build(self.session.input);

        self.session.bytecode_map.insert(self.session.input, code);

        Ok(())
    }
}
