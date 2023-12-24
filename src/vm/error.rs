use crate::error::error_maker;
use crate::source::CodeArea;

error_maker! {
    #[title = "Runtime Error"]
    RuntimeError {
        #[msg = "Unexpected Cock"]
        #[labels = [
            area => "Death";
        ]]
        ShitPesticide {
            area: CodeArea,
        },


    }
}
