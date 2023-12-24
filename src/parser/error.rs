use crate::error::error_maker;
use crate::lexer::error::LexerError;
use crate::lexer::tokens::Token;
use crate::source::CodeArea;

error_maker! {
    #[title = "Syntax Error"]
    SyntaxError {
        #[msg = "Unexpected Token"]
        #[labels = [
            area => "Expected `{}`, found `{}`": expected, found.to_str();
        ]]
        UnexpectedToken {
            expected: String,
            found: Token,
            area: CodeArea,
        },

        #[msg = "Lexer error"]
        #[labels = [
            area => "{}": err;
        ]]
        LexingError {
            err: LexerError,
            area: CodeArea,
        },

        // ==================================================================

        #[msg = "Unmatched token"] #[note = None]
        #[labels = [
            area => "Couldn't find matching `{}` for this `{}`": not_found.to_str(), for_char.to_str();
        ]]
        UnmatchedToken {
            for_char: Token,
            not_found: Token,
            area: CodeArea,
        },

        // ==================================================================

        #[msg = "Unexpected character"] #[note = None]
        #[labels = [
            area => "Expected `{}`, found `{}`": expected.to_string(), found;
        ]]
        UnexpectedCharacter {
            expected: char,
            found: String,
            area: CodeArea,
        },

        // ==================================================================

        #[msg = "Unexpected string flag"] #[note = None]
        #[labels = [
            area => "Expected valid string flag, found `{}`": flag;
        ]]
        UnexpectedFlag {
            flag: String,
            area: CodeArea,
        },

        // ==================================================================

        #[msg = "Error parsing escape sequence"] #[note = None]
        #[labels = [
            area => "Unknown escape sequence \\`{}`": character;
        ]]
        InvalidEscape {
            character: char,
            area: CodeArea,
        },

        // ==================================================================

        #[msg = "Error parsing unicode escape sequence"] #[note = None]
        #[labels = [
            area => "Invalid unicode sequence `{}`": sequence;
        ]]
        InvalidUnicode {
            sequence: String,
            area: CodeArea,
        },

        // ==================================================================

        #[msg = "Cannot have multiple spread arguments"] #[note = None]
        #[labels = [
            area => "Second spread argument provided here";
            prev_area => "First spread argument provided here";
        ]]
        MultipleSpreadArguments {
            area: CodeArea,
            prev_area: CodeArea,
        },

        // ==================================================================

        #[msg = "Positional argument after keyword argument"] #[note = None]
        #[labels = [
            area => "This positional argument was provided after keyword arguments";
            keyword_area => "First keyword argument provided here";
        ]]
        PositionalArgAfterKeyword {
            area: CodeArea,
            keyword_area: CodeArea,
        },

        // ==================================================================

        #[msg = "Duplicate keyword argument"] #[note = None]
        #[labels = [
            area => "Keyword argument `{}` was provided twice": name;
            prev_area => "Argument previously provided here";
        ]]
        DuplicateKeywordArg {
            name: String,
            area: CodeArea,
            prev_area: CodeArea,
        },

        // ==================================================================

        #[msg = "Duplicate attribute field"] #[note = None]
        #[labels = [
            first_used => "Field `{}` first used here": field;
            used_again => "Used again here";
        ]]
        DuplicateAttributeField {
            used_again: CodeArea,
            field: String,
            first_used: CodeArea,
        },

        // ==================================================================

        #[msg = "Invalid string type used for dictionary key"] #[note = Some("f-strings and byte strings are not allowed as keys".into())]
        #[labels = [
            area => "Invalid string here";
        ]]
        InvalidDictStringKey {
            area: CodeArea,
        },

        // ==================================================================

        #[msg = "Unbalanced block in format string"] #[note = None]
        #[labels = [
            area => "Expected `{}`": expected;
        ]]
        UnbalancedFormatStringBlock {
            expected: &'static str,
            area: CodeArea,
        },

        // ==================================================================

        #[msg = "Invalid `self` argument position"] #[note = None]
        #[labels = [
            area => "Argument is at position {}": pos;
        ]]
        SelfArgumentNotFirst {
            pos: usize,
            area: CodeArea,
        },

        // ==================================================================

        #[msg = "`self` argument cannot be spread"] #[note = None]
        #[labels = [
            area => "Spread occurs on this `self`";
        ]]
        SelfArgumentCannotBeSpread {
            area: CodeArea,
        },

        // ==================================================================

        #[msg = "Unknown attribute namespace"] #[note = None]
        #[labels = [
            area => "Namespace `{}` does not exist": namespace;
        ]]
        UnknownAttributeNamespace {
            namespace: String,
            area: CodeArea,
        },

        // ==================================================================

        #[msg = "Unknown attribute"] #[note = None]
        #[labels = [
            area => "Attribute `{}` does not exist": attribute;
        ]]
        UnknownAttribute {
            attribute: String,
            area: CodeArea,
        },

        // ==================================================================

        // #[msg = "Mismatched attribute style"] #[note = Some("`#![...]` in an inner attribute and `#[...]` is an outer attribute".into())]
        // #[labels = [
        //     area => "Attribute does not exist as an {} attribute": style;
        // ]]
        // MismatchedAttributeStyle {
        //     style: AttrStyle,
        //     area: CodeArea,
        // },

        // ==================================================================

        #[msg = "Duplicate attribute"]
        #[note = None]
        #[labels = [
            old_area => "Attribute `{}` originally specified here": attribute;
            current_area => "Attribute also specified here";
        ]]
        DuplicateAttribute {
            attribute: String,
            current_area: CodeArea,
            old_area: CodeArea,
        },

        // ==================================================================

        // #[msg = "No arguments provided to attribute"] #[note = Some("A `word` attribute is an attribute without values (E.G. `#[debug_bytecode]`)".into())]
        // #[labels = [
        //     attribute_area => "Attribute `{}` expected to take value(s)": attribute;
        //     attribute_area => "Attribute used as a word here";
        // ]]
        // NoArgumentsProvidedToAttribute {
        //     attribute: String,
        //     attribute_area: CodeArea,
        // },

        // ==================================================================

        #[msg = "Unknown argument in attribute"] #[note = None]
        #[labels = [
            attribute_area => "Unknown argument for attribute `{}`": attribute;
            arg_area => "Argument provided here";
        ]]
        UnknownAttributeArgument {
            attribute: String,
            attribute_area: CodeArea,
            arg_area: CodeArea,
        },

        // ==================================================================

        #[msg = "Unexpected value for attribute"] #[note = None]
        #[labels = [
            attribute_area => "Unexpected value provided to attribute `{}`": attribute;
            value_area => "Argument provided here";
        ]]
        UnexpectedValueForAttribute {
            attribute: String,
            attribute_area: CodeArea,
            value_area: CodeArea,
        },

        // ==================================================================

        #[msg = "Missing required arguments for attribute"] #[note = Some(format!("The missing arguments may be: {}", list_join(missing)))]
        #[labels = [
            attribute_area => "Expected {} required arguments for attribute `{}`": expected, attribute;
            args_area => "Found only {} args here": found;
        ]]
        MissingRequiredArgumentsForAttribute {
            attribute: String,
            expected: usize,
            found: usize,
            attribute_area: CodeArea,
            args_area: CodeArea,
            missing: Vec<String>,
        },

        // ==================================================================

        #[msg = "Mismatched attribute target"] #[note = None]
        #[labels = [
            target_area => "Attribute `{}` cannot be added to this element": attribute;
        ]]
        MismatchedAttributeTarget {
            target_area: CodeArea,
            attribute: String,
        },

        // ==================================================================

        #[msg = "Found `mut self`"] #[note = Some("`mut self` is unlikely the behaviour you want as it will clone `self`. Instead, to make `self` mutable, take a mutable reference: `&self`".into())]
        #[labels = [
            area => "Found here";
        ]]
        MutSelf {
            area: CodeArea,
        },

        // ==================================================================

        #[msg = "Integer too large"] #[note = Some(format!("The maximum size for an integer is: {}", i64::MAX))]
        #[labels = [
            area => "Found integer here";
        ]]
        IntegerTooLarge {
            area: CodeArea,
        },

        // ==================================================================

        #[msg = "Unknown object key"] #[note = None]
        #[labels = [
            area => "Found key here";
        ]]
        UnknownObjectKey {
            area: CodeArea,
        },
    }
}
