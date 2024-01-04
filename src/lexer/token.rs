use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
#[logos(skip r#"[ \t\r\f]+|//.*"#)]
pub enum Token {
    // LITERALS ====================================
    #[regex(r#"\d+"#)]
    Int,
    #[regex(r#"0x[a-fA-F0-9]+"#)]
    HexInt,
    #[regex(r#"0o[0-7]+"#)]
    OctalInt,
    #[regex(r#"0b[0-1]+"#)]
    BinaryInt,

    #[regex(r#"\d*\.\d+"#)]
    Float,

    #[token("true")]
    True,
    #[token("false")]
    False,

    #[regex(r#"\d+g"#)]
    GroupID,
    #[regex(r#"\d+c"#)]
    ColorID,
    #[regex(r#"\d+i"#)]
    ItemID,
    #[regex(r#"\d+b"#)]
    BlockID,
    #[regex(r#"\d+t"#)]
    TimerID,
    #[regex(r#"\d+e"#)]
    EffectID,
    #[regex(r#"\d+ch"#)]
    ChannelID,
    #[regex(r#"\d+m"#)]
    MaterialID,

    #[regex(r#"\?g"#)]
    ArbitraryGroupID,
    #[regex(r#"\?c"#)]
    ArbitraryColorID,
    #[regex(r#"\?i"#)]
    ArbitraryItemID,
    #[regex(r#"\?b"#)]
    ArbitraryBlockID,
    #[regex(r#"\?t"#)]
    ArbitraryTimerID,
    #[regex(r#"\?e"#)]
    ArbitraryEffectID,
    #[regex(r#"\?ch"#)]
    ArbitraryChannelID,
    #[regex(r#"\?m"#)]
    ArbitraryMaterialID,

    // IDENTS ====================================
    #[regex(r#"[a-zA-Z_]\w*"#)]
    Ident,
    #[regex(r#"@[a-zA-Z_]\w*"#)]
    Type,
    #[token("self")]
    Slf,

    // KEYWORDS ====================================
    #[token("mut")]
    Mut,
    #[token("type")]
    TypeDecl,
    #[token("_")]
    Any,

    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("for")]
    For,
    #[token("while")]
    While,
    #[token("match")]
    Match,

    #[token("extract")]
    Extract,
    #[token("import")]
    Import,

    #[token("unsafe")]
    Unsafe,

    #[token("try")]
    Try,
    #[token("catch")]
    Catch,
    #[token("throw")]
    Throw,

    #[token("return")]
    Return,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,

    #[token("dbg")]
    Dbg,

    // PUNCT ====================================
    #[token("\n")]
    Newline,
    #[token(";")]
    Semicolon,

    #[token(":")]
    Colon,
    #[token("::")]
    DoubleColon,
    #[token("->")]
    Arrow,
    #[token("=>")]
    FatArrow,
    #[token(".")]
    Dot,
    #[token("..")]
    Range,
    #[token("...")]
    Ellipsis,

    #[token(",")]
    Comma,

    #[token("(")]
    OpenParen,
    #[token(")")]
    ClosedParen,
    #[token("[")]
    OpenSqBracket,
    #[token("]")]
    ClosedSqBracket,
    #[token("{")]
    OpenBracket,
    #[token("}")]
    ClosedBracket,
    #[token("!{")]
    TriggerFnBracket,

    #[token("&")]
    Ampersand,
    #[token("|")]
    Pipe,

    #[token("?")]
    QMark,

    // OPERATORS ====================================
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Mult,
    #[token("/")]
    Div,
    #[token("%")]
    Mod,
    #[token("**")]
    Pow,

    #[token("+=")]
    PlusEq,
    #[token("-=")]
    MinusEq,
    #[token("*=")]
    MultEq,
    #[token("/=")]
    DivEq,
    #[token("%=")]
    ModEq,
    #[token("**=")]
    PowEq,

    #[token("=")]
    Assign,

    #[token("==")]
    Eq,
    #[token("!=")]
    NEq,
    #[token(">")]
    Gt,
    #[token(">=")]
    GtE,
    #[token("<")]
    Lt,
    #[token("<=")]
    LtE,

    #[token("!")]
    ExclMark,

    #[token("||")]
    Or,
    #[token("&&")]
    And,
    #[token("in")]
    In,

    // OTHER ====================================
    #[logos(skip)]
    Eof,
}

impl Token {
    pub fn name(&self) -> &'static str {
        match self {
            Token::Int => "int literal",
            Token::HexInt => "hex int literal",
            Token::OctalInt => "octal int literal",
            Token::BinaryInt => "binary int literal",
            Token::Float => "float literal",
            Token::True => "true",
            Token::False => "false",
            Token::GroupID => "group id",
            Token::ColorID => "color id",
            Token::ItemID => "item id",
            Token::BlockID => "block id",
            Token::TimerID => "timer id",
            Token::EffectID => "effect id",
            Token::ChannelID => "channel id",
            Token::MaterialID => "material id",
            Token::ArbitraryGroupID => "arbitrary group id",
            Token::ArbitraryColorID => "arbitrary color id",
            Token::ArbitraryItemID => "arbitrary item id",
            Token::ArbitraryBlockID => "arbitrary block id",
            Token::ArbitraryTimerID => "arbitrary timer id",
            Token::ArbitraryEffectID => "arbitrary effect id",
            Token::ArbitraryChannelID => "arbitrary channel id",
            Token::ArbitraryMaterialID => "arbitrary material id",
            Token::Ident => "identifier",
            Token::Type => "type indicator",
            Token::Slf => "self",
            Token::Any => "_",
            Token::Mut => "mut",
            Token::TypeDecl => "type",
            Token::If => "if",
            Token::Else => "else",
            Token::For => "for",
            Token::While => "while",
            Token::Match => "match",
            Token::Extract => "extract",
            Token::Import => "import",
            Token::Unsafe => "unsafe",
            Token::Try => "try",
            Token::Catch => "catch",
            Token::Throw => "throw",
            Token::Return => "return",
            Token::Break => "break",
            Token::Continue => "continue",
            Token::Dbg => "dbg",
            Token::Newline => "\n",
            Token::Semicolon => ";",
            Token::Colon => ":",
            Token::DoubleColon => "::",
            Token::Arrow => "->",
            Token::FatArrow => "=>",
            Token::Dot => ".",
            Token::Range => "..",
            Token::Ellipsis => "...",
            Token::Comma => ",",
            Token::OpenParen => "(",
            Token::ClosedParen => ")",
            Token::OpenSqBracket => "[",
            Token::ClosedSqBracket => "]",
            Token::OpenBracket => "{",
            Token::ClosedBracket => "}",
            Token::TriggerFnBracket => "!{",
            Token::Ampersand => "&",
            Token::Pipe => "|",
            Token::QMark => "?",
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Mult => "*",
            Token::Div => "/",
            Token::Mod => "%",
            Token::Pow => "**",
            Token::PlusEq => "+=",
            Token::MinusEq => "-=",
            Token::MultEq => "*=",
            Token::DivEq => "/=",
            Token::ModEq => "%=",
            Token::PowEq => "**=",
            Token::Assign => "=",
            Token::Eq => "==",
            Token::NEq => "!=",
            Token::Gt => ">",
            Token::GtE => ">=",
            Token::Lt => "<",
            Token::LtE => "<=",
            Token::ExclMark => "!",
            Token::Or => "||",
            Token::And => "&&",
            Token::In => "in",
            Token::Eof => "end of file",
        }
    }
}
