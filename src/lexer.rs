use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\r]+")]
pub enum Token {
    #[regex(r"[a-z][a-zA-Z0-9_]*", priority = 3)]
    Identifier,

    #[regex(r"[A-Z][a-zA-Z0-9]*", priority = 3)]
    Type,

    #[regex(r"[0-9]*.[0-9]+", priority = 4)]
    Float,

    #[regex(r"[0-9]+", priority = 5)]
    Integer,

    #[regex(".+", priority = 7)]
    String,

    #[token(";")]
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
}