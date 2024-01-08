use itertools::Itertools;

use super::value::ValueType;
use crate::error::error_maker;
use crate::errors::diagnostic;
use crate::parser::operators::BinOp;
use crate::source::CodeArea;

diagnostic! {
    #[title = "Runtime Error"]
    #[level = Error]
    RuntimeError {

        #[message = "Variable not initialized"]
        #[labels = [
            area => "This variable has not been initialized yet";
        ]]
        VarNotInitialized {
            area: CodeArea,
        },

        #[message = "Invalid operands"]
        #[labels = [
            area => "Operator `{}` cannot be applied to {} and {}": op.to_str(), v1.0.runtime_display(), v2.0.runtime_display();
            v1.1 => "This is of type {}": v1.0.runtime_display();
            v2.1 => "This is of type {}": v2.0.runtime_display();
        ]]
        InvalidOperands {
            v1: (ValueType, CodeArea),
            v2: (ValueType, CodeArea),
            op: BinOp,
            area: CodeArea,
        },

        #[message = "Type mismatch"]
        #[labels = [
            area => "Expected {}, found {}": {
                let len = expected.len();
                expected.iter().enumerate().map(|(i, t)| {
                    if len > 1 && i == len - 1 {
                        format!("or {}", t.runtime_display())
                    } else {
                        t.runtime_display()
                    }
                }).join(if len > 2 { ", " } else { " " })
            }, value.0.runtime_display();
            value.1 => "Value defined as {} here": value.0.runtime_display();
        ]]
        TypeMismatch {
            value: (ValueType, CodeArea),
            expected: &'static [ValueType],
            area: CodeArea,
        },
    }
}
