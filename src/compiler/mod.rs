use ahash::AHashMap;
use lasso::{Rodeo, Spur};

use self::builder::proto::{BlockID, ProtoBytecode};
use self::error::CompilerError;
use crate::bytecode::opcode::VarID;
use crate::ids_helper;
use crate::parser::ast::pattern::PatternNode;
use crate::parser::ast::Ast;
use crate::source::{BytecodeMap, CodeArea, CodeSpan, SpwnSource};
use crate::util::slabmap::SlabMap;
use crate::util::ImmutStr;

pub mod builder;
pub mod error;
pub mod expr;
pub mod pattern;
pub mod stmt;
pub mod util;

pub type CompileResult<T> = Result<T, CompilerError>;

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
pub enum ScopeType<'a> {
    Global,
    Loop(BlockID),
    MacroBody(Option<&'a PatternNode>), // return pattern
    TriggerFunc(CodeSpan),
    ArrowStmt(CodeSpan),
}

#[derive(Debug, Clone)]
pub struct Scope<'a> {
    vars: AHashMap<Spur, VarData>,
    parent: Option<ScopeID>,
    typ: Option<ScopeType<'a>>,
}

pub struct Compiler<'a> {
    src: &'static SpwnSource,

    interner: &'a mut Rodeo,
    bytecode_map: &'a mut BytecodeMap,

    scopes: SlabMap<ScopeID, Scope<'a>>,
    // pub type_def_map: &'a mut TypeDefMap,
    // pub local_type_defs: SlabMap<LocalTypeID, Vis<TypeDef<Spur>>>,

    // pub available_custom_types: AHashMap<Spur, Vis<CustomTypeID>>,

    // deferred_trigger_func_stack: Vec<Vec<DeferredTriggerFunc>>,

    // pub deprecated_features: DeprecatedFeatures,

    // pub import_stack: &'a mut Vec<(Rc<SpwnSource>, CodeArea)>,
}

impl<'a> Compiler<'a> {
    pub fn make_area(&self, span: CodeSpan) -> CodeArea {
        CodeArea {
            span,
            src: self.src,
        }
    }

    pub fn new(
        src: &'static SpwnSource,
        interner: &'a mut Rodeo,
        bytecode_map: &'a mut BytecodeMap,
    ) -> Self {
        Self {
            src,
            interner,
            bytecode_map,
            scopes: SlabMap::new(),
        }
    }

    fn intern(&mut self, s: &str) -> Spur {
        self.interner.get_or_intern(s)
    }

    pub fn resolve(&self, s: &Spur) -> &str {
        self.interner.resolve(s)
    }

    pub fn resolve_immut(&self, s: &Spur) -> ImmutStr {
        self.interner.resolve(s).into()
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
        &self,
        var: Spur,
        scope: ScopeID,
        span: CodeSpan,
    ) -> CompileResult<VarData> {
        self.get_var(var, scope)
            .ok_or_else(|| CompilerError::NonexistentVariable {
                area: self.make_area(span),
                var: self.resolve_immut(&var),
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

        let code = code.build(self.src);

        self.bytecode_map.insert(self.src, code);

        Ok(())
    }
}
