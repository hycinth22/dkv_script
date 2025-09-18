// 词法分析器的标记类型
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // 关键字
    Let, Fn, If, Else, For, While, Return,
    // 类型
    Int, Float, Bool, String,
    // 运算符
    Plus, Minus, Multiply, Divide,
    Equal, Equals, NotEquals, LessThan, LessThanOrEqual, GreaterThan, GreaterThanOrEqual,
    And, Or, Not,
    Increment, Decrement,
    // 括号
    LParen, RParen, LBrace, RBrace,
    // 分隔符
    Semicolon, Comma, Colon,
    // 字面量
    Identifier(String),
    IntLiteral(i32),
    FloatLiteral(f32),
    BoolLiteral(bool),
    StringLiteral(String),
    // 特殊标记
    Eof,
}

// 标记结构
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: u32,
    pub column: u32,
}

impl Token {
    pub fn new(token_type: TokenType, line: u32, column: u32) -> Self {
        Token {
            token_type,
            line,
            column,
        }
    }
}