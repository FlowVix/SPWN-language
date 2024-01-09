use crate::errors::diagnostic;
use crate::lexer::token::Token;
use crate::source::CodeSpan;

// error_maker! {
//     #[title = "Syntax Error"]
//     SyntaxError {
//         #[msg = "Unexpected token"]
//         #[labels = [
//             area => "Expected `{}`, found `{}`": expected, found.name();
//         ]]
//         UnexpectedToken {
//             expected: String,
//             found: Token,
//             area: CodeArea,
//         },

//         #[msg = "Unknown character"]
//         #[labels = [
//             area => "Unknown character";
//         ]]
//         LexingError {
//             area: CodeArea,
//         },

//         #[msg = "Found `mut self`"] #[note = Some("`mut self` is unlikely the behaviour you want as it will clone `self`. Instead, to make `self` mutable, take a mutable reference: `&self`".into())]
//         #[labels = [
//             area => "Found here";
//         ]]
//         MutSelf {
//             area: CodeArea,
//         },

//         #[msg = "Unmatched token"]
//         #[labels = [
//             area => "Couldn't find matching `{}` for this `{}`": not_found.name(), for_tok.name();
//         ]]
//         UnmatchedToken {
//             for_tok: Token,
//             not_found: Token,
//             area: CodeArea,
//         },
//     }
// }

diagnostic! {
    #[title = "Syntax Error"]
    #[level = Error]
    SyntaxError {
        #[message = "Unexpected token"]
        #[labels = [
            span => "Expected `{}`, found `{}`": expected, found.name();
        ]]
        UnexpectedToken {
            expected: String,
            found: Token,
            span: CodeSpan,
        },

        #[message = "Unknown character"]
        #[labels = [
            span => "Unknown character";
        ]]
        LexingError {
            span: CodeSpan,
        },

        #[message = "Found `mut self`"]
        #[note = "`mut self` is unlikely the behaviour you want as it will clone `self`. Instead, to make `self` mutable, take a mutable reference: `&self`"]
        #[labels = [
            span => "Found here";
        ]]
        MutSelf {
            span: CodeSpan,
        },

        #[message = "Unmatched token"]
        #[labels = [
            span => "Couldn't find matching `{}` for this `{}`": not_found.name(), for_tok.name();
        ]]
        UnmatchedToken {
            for_tok: Token,
            not_found: Token,
            span: CodeSpan,
        },
    }
}
