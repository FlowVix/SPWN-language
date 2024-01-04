use crate::error::error_maker;
use crate::source::CodeArea;
use crate::util::ImmutStr;

error_maker! {
    #[title = "Compiler Error"]
    CompilerError {
        // #[msg = "Invalid operands"]
        // #[labels = [
        //     area => "Operator `{}` cannot be applied to `{}` and `{}`": op.to_str(), v1.0.runtime_display(), v2.0.runtime_display();
        //     v1.1 => "This is of type `{}`": v1.0.runtime_display();
        //     v2.1 => "This is of type `{}`": v2.0.runtime_display();
        // ]]
        // InvalidOperands {
        //     v1: (ValueType, CodeArea),
        //     v2: (ValueType, CodeArea),
        //     op: BinOp,
        //     area: CodeArea,
        // },
        #[msg = "Nonexistent variable"]
        #[labels = [
            area => "Variable `{}` does not exist": var;
        ]]
        NonexistentVariable {
            area: CodeArea,
            var: ImmutStr,
        },
    }
}
