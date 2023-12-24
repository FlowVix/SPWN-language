use delve::EnumDisplay;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumDisplay)]
pub enum LexerError {
    #[delve(display = "Invalid type indicator")]
    InvalidTypeIndicator,
    #[delve(display = "Invalid hex literal")]
    InvalidHexLiteral,
    #[delve(display = "Invalid octal literal")]
    InvalidOctalLiteral,
    #[delve(display = "Invalid binary literal")]
    InvalidBinaryLiteral,
    #[delve(display = "Invalid seximal literal")]
    InvalidSeximalLiteral,
    #[delve(display = "Invalid dozenal literal")]
    InvalidDozenalLiteral,
    #[delve(display = "Invalid base-φ literal")]
    InvalidGoldenLiteral,
    #[delve(display = "Unknown character")]
    UnknownCharacter,
    #[delve(display = "Unterminated block comment")]
    UnterminatedBlockComment,
    #[delve(display = "Unterminated string")]
    UnterminatedString,
    #[delve(display = "Invalid character for raw string")]
    InvalidCharacterForRawString,
}
