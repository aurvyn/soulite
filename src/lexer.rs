use logos::{Lexer, Logos};

#[derive(Logos, Clone, Debug, PartialEq)]
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

    #[token("[<")]
    LeftList,

    #[token(">]")]
    RightList,

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

    #[token("=>")]
    FatArrow,

    #[token("_")]
    Underscore,

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

impl Token {
    pub fn get_precedence(&self) -> u8 {
        match self {
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
            Token::And | Token::Or => 3,
            Token::Pipe => 4,
            Token::Caret => 5,
            Token::Amp => 6,
            Token::LessThan
            | Token::GreaterThan
            | Token::LessThanOrEqual
            | Token::GreaterThanOrEqual
            | Token::Equal
            | Token::NotEqual => 7,
            Token::ShiftLeft | Token::ShiftRight | Token::PipeLeft | Token::PipeRight => 8,
            Token::Plus | Token::Minus => 9,
            Token::Multiply | Token::Divide | Token::Modulo => 10,
            Token::Exponent => 11,
            Token::Dot => 12,
            _ => 0,
        }
    }
}

pub trait CheckToken {
    fn is_arrow(&self) -> bool;
    fn is_assign(&self) -> bool;
    fn is_colon(&self) -> bool;
    fn is_identifier(&self) -> bool;
    fn is_newline(&self) -> bool;
    fn is_tab(&self) -> bool;
    fn is_type(&self) -> bool;
}

impl CheckToken for Option<Result<Token, ()>> {
    fn is_arrow(&self) -> bool {
        self == &Some(Ok(Token::Arrow))
    }

    fn is_assign(&self) -> bool {
        self == &Some(Ok(Token::Assign))
    }

    fn is_colon(&self) -> bool {
        self == &Some(Ok(Token::Colon))
    }

    fn is_identifier(&self) -> bool {
        self == &Some(Ok(Token::Identifier))
    }

    fn is_newline(&self) -> bool {
        self == &Some(Ok(Token::Newline))
    }

    fn is_tab(&self) -> bool {
        self == &Some(Ok(Token::Tab))
    }

    fn is_type(&self) -> bool {
        self == &Some(Ok(Token::Type)) || self == &Some(Ok(Token::LeftBracket))
    }
}

pub trait Lookahead {
    fn peek(&mut self) -> Option<Result<Token, ()>>;
}

impl<'source> Lookahead for Lexer<'source, Token> {
    fn peek(&mut self) -> Option<Result<Token, ()>> {
        self.clone().next()
    }
}
