use super::CallExpr;

#[macro_export]
macro_rules! ids_helper {
    (
        $(
            $name:ident($typ:ty);
        )*
    ) => {
        $(
            #[derive(
                Debug,
                Clone,
                Copy,
                PartialEq,
                Eq,
                derive_more::Into,
                derive_more::From,
                derive_more::Deref,
                derive_more::DerefMut,
                Hash,
            )]
            pub struct $name($typ);

            impl From<usize> for $name {
                fn from(value: usize) -> Self {
                    $name(value as $typ)
                }
            }
            impl From<$name> for usize {
                fn from(value: $name) -> Self {
                    value.0 as usize
                }
            }

            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}({})", stringify!($name), self.0)
                }
            }
        )*
    };
}

ids_helper! {
    ConstID(u16);
    CallExprID(u16);
    JumpPos(u16);
    VarID(u16);
    FuncID(u16);
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    delve::EnumVariantNames,
    delve::EnumDisplay,
    delve::EnumToStr,
    delve::EnumFields,
)]
#[delve(rename_variants = "screamingsnakecase")]
pub enum Opcode {
    #[delve(display = || "pop top")]
    PopTop,
    #[delve(display = |id| format!("load {id}"))]
    LoadConst(ConstID),

    // #[delve(display = |id| format!("check {id}"))]
    // CheckPattern(PatternID),
    #[delve(display = |id| format!("change {id} key"))]
    ChangeVarKey(VarID),
    #[delve(display = |id| format!("set {id}"))]
    SetVar(VarID),
    #[delve(display = |id| format!("load {id}"))]
    LoadVar(VarID),

    #[delve(display = "+")]
    Plus,
    #[delve(display = "-")]
    Minus,
    #[delve(display = "*")]
    Mult,
    #[delve(display = "/")]
    Div,
    #[delve(display = "%")]
    Mod,
    #[delve(display = "**")]
    Pow,

    #[delve(display = "==")]
    Eq,
    #[delve(display = "!=")]
    NEq,
    #[delve(display = ">")]
    Gt,
    #[delve(display = ">=")]
    GtE,
    #[delve(display = "<")]
    Lt,
    #[delve(display = "<=")]
    LtE,

    #[delve(display = "unary -")]
    UnaryMinus,
    #[delve(display = "unary !")]
    UnaryNot,

    #[delve(display = |to| format!("to {to}"))]
    Jump(JumpPos),
    #[delve(display = |to| format!("if false, to {to}"))]
    JumpIfFalse(JumpPos),
    #[delve(display = |to| format!("if true, to {to}"))]
    JumpIfTrue(JumpPos),

    #[delve(display = |len| format!("make array of length {len}"))]
    MakeArray { len: u16 },

    #[delve(display = "dbg")]
    Dbg,

    #[delve(display = |to| format!("arrow to {to}"))]
    EnterArrowStatement(JumpPos),
    #[delve(display = || "yeet")]
    YeetContext,

    #[delve(display = || "if false, throw mismatch")]
    MismatchThrowIfFalse,

    #[delve(display = || "return")]
    Return,

    #[delve(display = |f| format!("make macro, fID {f}"))]
    MakeMacro(FuncID),

    #[delve(display = |c| format!("call({c})"))]
    Call(CallExprID),
}

const _SIZE_CHECK: [u8; 4] = [0; std::mem::size_of::<Opcode>()];
