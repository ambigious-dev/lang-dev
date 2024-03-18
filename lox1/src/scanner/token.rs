#[rustfmt::skip]

#[derive(Debug, Default)]
pub enum TokenType {
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, FullStop, Minus, Plus, Semi, Slash, Star,
  
    Bang, BangEq,
    Equal, EqualEq,
    Greater, GreaterEq,
    Less, LessEq,

    Ident, Str, Num,
  
    Print, Class, Super, This,
    Fun, Return, If, Else, For, While, Var,
    And, Or, True, False, Nil,

    #[default]
    EOF,
}

#[derive(Debug)]
pub enum LiteralType {
    Number(i64),
    String(String),
    Bool(bool),
    Nil,
}

#[derive(Debug, Default)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<LiteralType>,
    pub line: u32,
    pub col: u32,
}
