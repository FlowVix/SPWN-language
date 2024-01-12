use super::{ParseResult, Parser};

// impl<'a> Parser<'a> {
//     pub fn parse_int(&mut self, s: &str, base: usize) -> i64 {

//     }
// }

#[macro_export]
macro_rules! list_helper {
    ($self:ident, $closing_tok:ident $code:block) => {
        while !$self.next_is(Token::$closing_tok) {
            $code;
            if !$self.skip_tok(Token::Comma) {
                break;
            }
        }
        $self.expect_tok_recover(Token::$closing_tok);
    };

    ($self:ident, $first:ident, $closing_tok:ident $code:block) => {
        let mut $first = true;
        while !$self.next_is(Token::$closing_tok) {
            $code;
            $first = false;
            if !$self.skip_tok(Token::Comma) {
                break;
            }
        }
        $self.expect_tok_recover(Token::$closing_tok);
    };
}
