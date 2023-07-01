use std::cell::RefCell;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

use super::bytecode::{OptRegister, Register, UnoptRegister};
use crate::new_id_wrapper;

pub type UnoptOpcode = Opcode<UnoptRegister>;
pub type OptOpcode = Opcode<OptRegister>;

macro_rules! opcodes {
    (
        $(
            $(#[$delve:meta])?
            $name:ident $({
                $(
                    $($field:ident: $typ:ty)?
                    $([$reg_field:ident])?
                ),+ $(,)?
            })?
        ),* $(,)?
    ) => {

        #[derive(Debug, Clone, Copy, delve::EnumVariantNames, delve::EnumDisplay, delve::EnumToStr, delve::EnumFields, Serialize, Deserialize)]
        #[delve(rename_variants = "screamingsnakecase")]
        pub enum Opcode<R: Copy + std::fmt::Display> {
            $(
                $(#[$delve])?
                $name $({
                    $(
                        $($field: $typ)?
                        $($reg_field: R)?
                        ,
                    )+
                })?,
            )*
        }

        impl TryFrom<Opcode<UnoptRegister>> for Opcode<OptRegister> {
            type Error = ();

            fn try_from(value: Opcode<UnoptRegister>) -> Result<Self, Self::Error> {
                match value {
                    $(
                        Opcode::$name $({$(
                            $($field)?
                            $($reg_field)?
                            ,
                        )+})? => Ok(Opcode::$name $({$(
                            $($reg_field: $reg_field.try_into().map_err(|_| ())?,)?
                            $($field,)?
                        )+})?),
                    )*
                }
            }
        }

    };
}

new_id_wrapper! {
    ConstID: u16;
    OpcodePos: u16;
    ImportID: u16;
    TryCatchID: u16;
}

opcodes! {
    #[delve(display = |id, to| format!("load {id} -> {to}"))]
    LoadConst { id: ConstID, [to] },
    #[delve(display = |from, to| format!("{from} -> {to}"))]
    Copy { [from], [to] },

    #[delve(display = |from, to| format!("{from} ~> {to}"))]
    CopyMem { [from], [to] },

    #[delve(display = |a, b, to| format!("{a} + {b} -> {to}"))]
    Plus { [a], [b], [to] },
    #[delve(display = |a, b, to| format!("{a} - {b} -> {to}"))]
    Minus { [a], [b], [to] },
    #[delve(display = |a, b, to| format!("{a} * {b} -> {to}"))]
    Mult { [a], [b], [to] },
    #[delve(display = |a, b, to| format!("{a} / {b} -> {to}"))]
    Div { [a], [b], [to] },
    #[delve(display = |a, b, to| format!("{a} % {b} -> {to}"))]
    Mod { [a], [b], [to] },
    #[delve(display = |a, b, to| format!("{a} ^ {b} -> {to}"))]
    Pow { [a], [b], [to] },
    #[delve(display = |a, b, to| format!("{a} == {b} -> {to}"))]
    Eq { [a], [b], [to] },
    #[delve(display = |a, b, to| format!("{a} != {b} -> {to}"))]
    Neq { [a], [b], [to] },
    #[delve(display = |a, b, to| format!("{a} > {b} -> {to}"))]
    Gt { [a], [b], [to] },
    #[delve(display = |a, b, to| format!("{a} >= {b} -> {to}"))]
    Gte { [a], [b], [to] },
    #[delve(display = |a, b, to| format!("{a} < {b} -> {to}"))]
    Lt { [a], [b], [to] },
    #[delve(display = |a, b, to| format!("{a} <= {b} -> {to}"))]
    Lte { [a], [b], [to] },
    #[delve(display = |a, b, to| format!("{a} | {b} -> {to}"))]
    BinOr { [a], [b], [to] },
    #[delve(display = |a, b, to| format!("{a} & {b} -> {to}"))]
    BinAnd { [a], [b], [to] },
    #[delve(display = |a, b, to| format!("{a}..{b} -> {to}"))]
    Range { [a], [b], [to] },
    #[delve(display = |a, b, to| format!("{a} in {b} -> {to}"))]
    In { [a], [b], [to] },
    #[delve(display = |a, b, to| format!("{a} << {b} -> {to}"))]
    ShiftLeft { [a], [b], [to] },
    #[delve(display = |a, b, to| format!("{a} >> {b} -> {to}"))]
    ShiftRight { [a], [b], [to] },
    #[delve(display = |a, b, to| format!("{a} as {b} -> {to}"))]
    As { [a], [b], [to] },

    #[delve(display = |a, b| format!("{a} += {b}"))]
    PlusEq { [a], [b] },
    #[delve(display = |a, b| format!("{a} -= {b}"))]
    MinusEq { [a], [b] },
    #[delve(display = |a, b| format!("{a} *= {b}"))]
    MultEq { [a], [b] },
    #[delve(display = |a, b| format!("{a} /= {b}"))]
    DivEq { [a], [b] },
    #[delve(display = |a, b| format!("{a} ^= {b}"))]
    PowEq { [a], [b] },
    #[delve(display = |a, b| format!("{a} %= {b}"))]
    ModEq { [a], [b] },
    #[delve(display = |a, b| format!("{a} &= {b}"))]
    BinAndEq { [a], [b] },
    #[delve(display = |a, b| format!("{a} |= {b}"))]
    BinOrEq { [a], [b] },
    #[delve(display = |a, b| format!("{a} <<= {b}"))]
    ShiftLeftEq { [a], [b] },
    #[delve(display = |a, b| format!("{a} >>= {b}"))]
    ShiftRightEq { [a], [b] },

    #[delve(display = |v, to| format!("!{v} -> {to}"))]
    Not { [v], [to] },
    #[delve(display = |v, to| format!("-{v} -> {to}"))]
    Negate { [v], [to] },

    #[delve(display = |to| format!("to {to}"))]
    Jump { to: OpcodePos },
    // #[delve(display = |to: &FuncID| format!("jump to {to:?}"))]
    // FuncJump { to: FuncID },
    #[delve(display = |check, to| format!("if not {check}, to {to}"))]
    JumpIfFalse { [check], to: OpcodePos },
    #[delve(display = |check, to| format!("if {check}, to {to}"))]
    JumpIfTrue { [check], to: OpcodePos },
    #[delve(display = |check, to| format!("if {check} == ?, to {to}"))]
    UnwrapOrJump { [check], to: OpcodePos },


    #[delve(display = |src, dest| format!("{src}.iter() -> {dest}"))]
    WrapIterator { [src], [dest] },
    #[delve(display = |src, dest| format!("{src}.next() -> {dest}"))]
    IterNext { [src], [dest] },

    #[delve(display = |dest, len| format!("[...; {len}] -> {dest}"))]
    AllocArray { [dest], len: u16 },
    #[delve(display = |elem, dest| format!("push {elem} into {dest}"))]
    PushArrayElem { [elem], [dest] },

    #[delve(display = |dest, cap| format!("{{...; {cap}}} -> {dest}"))]
    AllocDict { [dest], capacity: u16 },
    #[delve(display = |elem, dest, key| format!("insert {key}:{elem} into {dest}"))]
    InsertDictElem { [elem], [dest], [key] },
    #[delve(display = |elem, dest, key| format!("insert priv {key}:{elem} into {dest}"))]
    InsertPrivDictElem { [elem], [dest], [key] },

    #[delve(display = |base, items, dest| format!("@{base}::{{{items}}} -> {dest}"))]
    MakeInstance { [base], [items], [dest] },


    #[delve(display = |skip| format!("skip to {skip}"))]
    EnterArrowStatement { skip: OpcodePos },
    #[delve(display = || "yeet")]
    YeetContext,


    #[delve(display = |to| format!("() -> {to}"))]
    LoadEmpty { [to] },
    #[delve(display = |to| format!("? -> {to}"))]
    LoadNone { [to] },
    #[delve(display = |to| format!("$ -> {to}"))]
    LoadBuiltins { [to] },
    #[delve(display = |to| format!("ε -> {to}"))]
    LoadEpsilon { [to] },


    #[delve(display = |reg| format!("make {reg} byte string"))]
    MakeByteString { [reg] },

    #[delve(display = |class: &IDClass, dest| format!("?{:?} -> {dest}", class.suffix()))]
    LoadArbitraryID { class: IDClass, [dest] },

    #[delve(display = |from, to| format!("{from}? -> {to}"))]
    WrapMaybe { [from], [to] },

    #[delve(display = |src, mr: &bool| format!("{} {src}", if *mr { "export" } else { "return" }))]
    Return { [src], module_ret: bool },

    #[delve(display = |reg| format!("dbg {reg}"))]
    Dbg { [reg] },

    #[delve(display = |reg| format!("throw {reg}"))]
    Throw { [reg] },

    #[delve(display = |id, dest| format!("import {id} -> {dest}"))]
    Import { id: ImportID, [dest] },

    #[delve(display = |from, dest| format!("@string({from}) -> {dest}"))]
    ToString { [from], [dest] },

    #[delve(display = |b, d, i| format!("{b}[{i}] -> {d}"))]
    Index { [base], [dest], [index] },
    #[delve(display = |f, d, i| format!("{f}.{i} -> {d}"))]
    Member { [from], [dest], [member] },
    #[delve(display = |f, d, i| format!("{f}::{i} -> {d}"))]
    Associated { [from], [dest], [member] },
    #[delve(display = |f, d, i| format!("{f}.@{i} -> {d}"))]
    TypeMember { [from], [dest], [member] },

    #[delve(display = |e, _a| format!("try {{...}} -> {e}"))]
    EnterTryCatch { [err], id: TryCatchID },

    #[delve(display = |_a| format!(" TODO "))]
    ExitTryCatch { id: TryCatchID },

    #[delve(display = |s, d| format!("{s}.type -> {d}"))]
    TypeOf { [src], [dest] },


    #[delve(display = |s, d| format!("{s}.len() -> {d}"))]
    Len { [src], [dest] },



    #[delve(display = |b, d, i| format!("{b}[{i}] ~> {d}"))]
    IndexMem { [base], [dest], [index] },
    #[delve(display = |f, d, i| format!("{f}.{i} ~> {d}"))]
    MemberMem { [from], [dest], [member] },
    #[delve(display = |f, d, i| format!("{f}::{i} ~> {d}"))]
    AssociatedMem { [from], [dest], [member] },


    #[delve(display = |r| format!("if not {r}, throw mismatch"))]
    MismatchThrowIfFalse { [reg] },

}
