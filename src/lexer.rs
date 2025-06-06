use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r" +")]
pub enum Token {
    #[regex(r"\p{Ll}[_\p{L}]*")]
    Identifier,

    #[regex(r"\p{Lu}\p{L}*")]
    Type,

    #[regex(r"(?:\d+\.\d*|\.\d+)")]
    Float,

    #[regex(r"\d+")]
    Integer,

    #[regex(r#""(?:[^"]|\\")*""#)]
    String,

    #[regex(r";[^\n]*")]
    Comment,

    #[token("=")]
    Assign,

    #[token("..")]
    Range,

    #[token("&&")]
    And,

    #[token("||")]
    Or,

    #[token("&")]
    Amp,

    #[token("|")]
    Pipe,

    #[token("<")]
    LessThan,

    #[token("<=")]
    LessThanOrEqual,

    #[token(">")]
    GreaterThan,

    #[token(">=")]
    GreaterThanOrEqual,

    #[token("==")]
    Equal,

    #[token("!=")]
    NotEqual,

    #[token("<<")]
    ShiftLeft,

    #[token(">>")]
    ShiftRight,

    #[token("<|")]
    PipeLeft,

    #[token("|>")]
    PipeRight,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Multiply,

    #[token("/")]
    Divide,

    #[token("%")]
    Modulo,

    #[token("**")]
    Exponent,

    #[token(".")]
    Dot,

    #[token("!")]
    Not,

    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("{")]
    LeftBrace,

    #[token("}")]
    RightBrace,

    #[token("[")]
    LeftBracket,

    #[token("]")]
    RightBracket,

    #[token(",")]
    Comma,

    #[token(":")]
    Colon,

    #[token("~")]
    Tilde,

    #[token("++")]
    Increment,

    #[token("--")]
    Decrement,

    #[token("^")]
    Caret,

    #[token("#")]
    Hash,

    #[token("@")]
    At,

    #[token("$")]
    Dollar,

    #[token("'")]
    Apostrophe,

    #[token("->")]
    Arrow,

    #[token("+=")]
    PlusAssign,

    #[token("-=")]
    MinusAssign,

    #[token("*=")]
    MultiplyAssign,

    #[token("/=")]
    DivideAssign,

    #[token("%=")]
    ModuloAssign,

    #[token("**=")]
    ExponentAssign,

    #[token("&=")]
    AndAssign,

    #[token("|=")]
    OrAssign,

    #[token("~=")]
    InverseAssign,

    #[token("^=")]
    XorAssign,

    #[token("<<=")]
    ShiftLeftAssign,

    #[token("<|=")]
    PipeLeftAssign,

    #[token("\t")]
    Tab,

    #[token("\n")]
    Newline,
}

pub fn get_precedence(tok: &Token) -> u8 {
    match tok {
        Token::Assign
        | Token::PlusAssign
        | Token::MinusAssign
        | Token::MultiplyAssign
        | Token::DivideAssign
        | Token::ModuloAssign
        | Token::AndAssign
        | Token::OrAssign
        | Token::InverseAssign
        | Token::XorAssign
        | Token::ExponentAssign
        | Token::ShiftLeftAssign
        | Token::PipeLeftAssign => 1,
        Token::Range => 2,
        Token::And
        | Token::Or => 3,
        Token::Pipe => 4,
        Token::Caret => 5,
        Token::Amp => 6,
        Token::LessThan
        | Token::GreaterThan
        | Token::LessThanOrEqual
        | Token::GreaterThanOrEqual
        | Token::Equal
        | Token::NotEqual => 7,
        Token::ShiftLeft
        | Token::ShiftRight
        | Token::PipeLeft
        | Token::PipeRight => 8,
        Token::Plus
        | Token::Minus => 9,
        Token::Multiply
        | Token::Divide
        | Token::Modulo => 10,
        Token::Exponent => 11,
        Token::Dot => 12,
        _ => 0,
    }
}
