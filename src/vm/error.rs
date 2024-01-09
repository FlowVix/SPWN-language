use itertools::Itertools;

use super::value::ValueType;
use crate::errors::diagnostic;
use crate::parser::operators::BinOp;
use crate::source::CodeSpan;

diagnostic! {
    #[title = "Runtime Error"]
    #[level = Error]
    RuntimeError {

        #[message = "Variable not initialized"]
        #[labels = [
            span => "This variable has not been initialized yet";
        ]]
        VarNotInitialized {
            span: CodeSpan,
        },

        #[message = "Invalid operands"]
        #[labels = [
            span => "Operator `{}` cannot be applied to {} and {}": op.to_str(), v1.0.runtime_display(), v2.0.runtime_display();
            v1.1 => "This is of type {}": v1.0.runtime_display();
            v2.1 => "This is of type {}": v2.0.runtime_display();
        ]]
        InvalidOperands {
            v1: (ValueType, CodeSpan),
            v2: (ValueType, CodeSpan),
            op: BinOp,
            span: CodeSpan,
        },

        #[message = "Type mismatch"]
        #[labels = [
            span => "Expected {}, found {}": {
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
            value: (ValueType, CodeSpan),
            expected: &'static [ValueType],
            span: CodeSpan,
        },
    }
}
