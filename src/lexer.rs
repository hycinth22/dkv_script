use crate::token::{Token, TokenType};

pub struct Lexer {
    source: Vec<char>,
    current: usize,
    line: u32,
    column: u32,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source: source.chars().collect(),
            current: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.is_at_end() {
            return Token::new(TokenType::Eof, self.line, self.column);
        }

        let c = self.advance();

        match c {
            '(' => Token::new(TokenType::LParen, self.line, self.column - 1),
            ')' => Token::new(TokenType::RParen, self.line, self.column - 1),
            '{' => Token::new(TokenType::LBrace, self.line, self.column - 1),
            '}' => Token::new(TokenType::RBrace, self.line, self.column - 1),
            ';' => Token::new(TokenType::Semicolon, self.line, self.column - 1),
            ',' => Token::new(TokenType::Comma, self.line, self.column - 1),
            ':' => Token::new(TokenType::Colon, self.line, self.column - 1),
            '+' => {
                if self.match_char('+') {
                    Token::new(TokenType::Increment, self.line, self.column - 2)
                } else {
                    Token::new(TokenType::Plus, self.line, self.column - 1)
                }
            },
            '-' => {
                if self.match_char('-') {
                    Token::new(TokenType::Decrement, self.line, self.column - 2)
                } else {
                    Token::new(TokenType::Minus, self.line, self.column - 1)
                }
            },
            '*' => Token::new(TokenType::Multiply, self.line, self.column - 1),
            '/' => Token::new(TokenType::Divide, self.line, self.column - 1),
            '!' => {
                if self.match_char('=') {
                    Token::new(TokenType::NotEquals, self.line, self.column - 2)
                } else {
                    Token::new(TokenType::Not, self.line, self.column - 1)
                }
            },
            '=' => {
                if self.match_char('=') {
                    Token::new(TokenType::Equals, self.line, self.column - 2)
                } else {
                    Token::new(TokenType::Equal, self.line, self.column - 1)
                }
            },
            '<' => {
                if self.match_char('=') {
                    Token::new(TokenType::LessThanOrEqual, self.line, self.column - 2)
                } else {
                    Token::new(TokenType::LessThan, self.line, self.column - 1)
                }
            },
            '>' => {
                if self.match_char('=') {
                    Token::new(TokenType::GreaterThanOrEqual, self.line, self.column - 2)
                } else {
                    Token::new(TokenType::GreaterThan, self.line, self.column - 1)
                }
            },
            '&' => {
                if self.match_char('&') {
                    Token::new(TokenType::And, self.line, self.column - 2)
                } else {
                    panic!("Unexpected character '&' at line {}, column {}", self.line, self.column - 1)
                }
            },
            '|' => {
                if self.match_char('|') {
                    Token::new(TokenType::Or, self.line, self.column - 2)
                } else {
                    panic!("Unexpected character '|' at line {}, column {}", self.line, self.column - 1)
                }
            },
            '"' => self.string(),
            c if c.is_ascii_digit() => self.number(),
            c if c.is_alphabetic() || c == '_' => self.identifier(),
            _ => panic!("Unexpected character '{}' at line {}, column {}", c, self.line, self.column - 1),
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            if self.is_at_end() {
                break;
            }

            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                    self.column += 1;
                },
                '\n' => {
                    self.advance();
                    self.line += 1;
                    self.column = 1;
                },
                '/' => {
                    if self.peek_next() == '/' {
                        // 跳过注释
                        while !self.is_at_end() && self.peek() != '\n' {
                            self.advance();
                            self.column += 1;
                        }
                    } else {
                        break;
                    }
                },
                _ => break,
            }
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            false
        } else {
            self.current += 1;
            self.column += 1;
            true
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn identifier(&mut self) -> Token {
        let start = self.current - 1;
        let start_column = self.column - 1;

        while !self.is_at_end() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            self.advance();
            self.column += 1;
        }

        let text: String = self.source[start..self.current].iter().collect();
        let token_type = self.identifier_type(&text);

        Token::new(token_type, self.line, start_column)
    }

    fn identifier_type(&self, text: &str) -> TokenType {
        match text {
            "let" => TokenType::Let,
            "fn" => TokenType::Fn,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "for" => TokenType::For,
            "while" => TokenType::While,
            "return" => TokenType::Return,
            "int" => TokenType::Int,
            "float" => TokenType::Float,
            "bool" => TokenType::Bool,
            "string" => TokenType::String,
            "true" => TokenType::BoolLiteral(true),
            "false" => TokenType::BoolLiteral(false),
            _ => TokenType::Identifier(text.to_string()),
        }
    }

    fn number(&mut self) -> Token {
        let start = self.current - 1;
        let start_column = self.column - 1;

        while !self.is_at_end() && self.peek().is_ascii_digit() {
            self.advance();
            self.column += 1;
        }

        if !self.is_at_end() && self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            self.column += 1;
            while !self.is_at_end() && self.peek().is_ascii_digit() {
                self.advance();
                self.column += 1;
            }

            let text: String = self.source[start..self.current].iter().collect();
            let value: f32 = text.parse().unwrap();
            Token::new(TokenType::FloatLiteral(value), self.line, start_column)
        } else {
            let text: String = self.source[start..self.current].iter().collect();
            let value: i32 = text.parse().unwrap();
            Token::new(TokenType::IntLiteral(value), self.line, start_column)
        }
    }

    fn string(&mut self) -> Token {
        let start_line = self.line;
        let start_column = self.column - 1;
        let start_pos = self.current;

        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            panic!("Unterminated string at line {}, column {}", start_line, start_column);
        }

        self.advance(); // 跳过右引号
        self.column += 1;
        let end_pos = self.current;

        let text: String = self.source[start_pos + 1..end_pos - 1].iter().collect();
        Token::new(TokenType::StringLiteral(text), self.line, start_column)
    }
}