use delve::{EnumToStr, EnumVariantNames};

use super::memory::{StoredValue, ValueKey};
use crate::bytecode::opcode::FuncID;
use crate::source::CodeSpan;

macro_rules! value {
    (
        $(
            $var:ident
            $((
                $($t1:tt)*
            ))?
            $({
                $($t2:tt)*
            })?,
        )*
    ) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum Value {
            $(
                $var $(($($t1)*))? $({$($t2)*})?,
            )*
        }
        impl Value {
            pub fn get_type(&self) -> ValueType {
                match self {
                    $(
                        Self::$var {..} => ValueType::$var,
                    )*
                }
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, EnumVariantNames, EnumToStr)]
        #[delve(rename_all = "snake_case")]
        pub enum ValueType {
            $(
                $var,
            )*
        }
    };
}

value! {
    Int(i64),
    Float(f64),
    Bool(bool),

    Array(Vec<ValueKey>),

    Macro {
        func: FuncID,
    },

    Empty,
}

impl ValueType {
    pub fn runtime_display(self) -> String {
        format!("@{}", <ValueType as Into<&str>>::into(self))
    }
}

impl Value {
    pub fn into_stored(self, def_span: CodeSpan) -> StoredValue {
        StoredValue {
            value: self,
            def_span,
        }
    }
}
