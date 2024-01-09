use crate::errors::diagnostic;
use crate::source::CodeSpan;
use crate::util::ImmutStr;

diagnostic! {
    #[title = "Compiler Error"]
    #[level = Error]
    CompilerError {
        // #[msg = "Invalid operands"]
        // #[labels = [
        //     span => "Operator `{}` cannot be applied to `{}` and `{}`": op.to_str(), v1.0.runtime_display(), v2.0.runtime_display();
        //     v1.1 => "This is of type `{}`": v1.0.runtime_display();
        //     v2.1 => "This is of type `{}`": v2.0.runtime_display();
        // ]]
        // InvalidOperands {
        //     v1: (ValueType, CodeSpan),
        //     v2: (ValueType, CodeSpan),
        //     op: BinOp,
        //     span: CodeSpan,
        // },
        #[message = "Nonexistent variable"]
        #[labels = [
            span => "Variable `{}` does not exist": var;
        ]]
        NonexistentVariable {
            span: CodeSpan,
            var: ImmutStr,
        },
    }
}
