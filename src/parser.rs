use crate::ast::ASTNode;
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::new(TokenType::Eof, 0, 0),
        };
        parser.current_token = parser.lexer.next_token();
        parser
    }

    pub fn parse(&mut self) -> Box<ASTNode> {
        dbg!(self.parse_program())
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn expect_token(&mut self, expected_type: TokenType) {
        // 对于带有关联数据的TokenType，我们只比较枚举变体类型，不比较具体值
        let tokens_match = match (&self.current_token.token_type, &expected_type) {
            (TokenType::Identifier(_), TokenType::Identifier(_)) => true,
            (TokenType::IntLiteral(_), TokenType::IntLiteral(_)) => true,
            (TokenType::FloatLiteral(_), TokenType::FloatLiteral(_)) => true,
            (TokenType::BoolLiteral(_), TokenType::BoolLiteral(_)) => true,
            (TokenType::StringLiteral(_), TokenType::StringLiteral(_)) => true,
            _ => self.current_token.token_type == expected_type
        };
        
        if !tokens_match {
            panic!("Expected token {:?}, but got {:?} at line {}, column {}", 
                   expected_type, 
                   self.current_token.token_type, 
                   self.current_token.line, 
                   self.current_token.column);
        }
        self.advance();
    }

    fn parse_program(&mut self) -> Box<ASTNode> {
        let mut statements = Vec::new();
        while self.current_token.token_type != TokenType::Eof {
            statements.push(dbg!(self.parse_statement()));
        }
        Box::new(ASTNode::Program(statements))
    }

    fn parse_statement(&mut self) -> Box<ASTNode> {
        println!("parse_statement, current token: {:?}", self.current_token.token_type);
        match &self.current_token.token_type {
            TokenType::Let => self.parse_variable_decl(),
            TokenType::If => self.parse_if_statement(),
            TokenType::For => self.parse_for_loop(),
            TokenType::While => self.parse_while_loop(),
            TokenType::Fn => self.parse_function_def(),
            TokenType::Return => self.parse_return(),
            TokenType::LBrace => self.parse_block(),
            TokenType::LParen => self.parse_expression(),
            TokenType::Semicolon => {
                self.expect_token(TokenType::Semicolon); // 跳过分号
                Box::new(ASTNode::Block(Vec::new())) // 返回一个空的代码块
            },
            TokenType::Identifier(_) => self.parse_assignment_or_function_call(),
            _ => panic!("Unexpected token in statement: {:?}", self.current_token.token_type),
        }
    }

    fn parse_variable_decl(&mut self) -> Box<ASTNode> {
        self.expect_token(TokenType::Let); // 跳过 let
        
        let name = if let TokenType::Identifier(name) = &self.current_token.token_type {
            name.clone()
        } else {
            panic!("Expected identifier after 'let'")
        };
        self.advance();

        // 检查是否有冒号
        if let TokenType::Colon = self.current_token.token_type {
            self.expect_token(TokenType::Colon); // 跳过冒号
        } else {
            panic!("Expected colon after variable name")
        }

        // 处理类型关键字
        let type_name = match &self.current_token.token_type {
            TokenType::Int => "int".to_string(),
            TokenType::Float => "float".to_string(),
            TokenType::Bool => "bool".to_string(),
            TokenType::String => "string".to_string(),
            TokenType::Identifier(type_name) => type_name.clone(),
            _ => panic!("Expected valid type after colon: {:?}", self.current_token.token_type)
        };
        self.advance();

        let initializer = if let TokenType::Equal = self.current_token.token_type {
              self.expect_token(TokenType::Equal); // 跳过等号
              Some(self.parse_expression())
          } else {
              None
          };

        self.expect_token(TokenType::Semicolon); // 跳过分号

        Box::new(ASTNode::VariableDecl(name, type_name, initializer))
    }

    fn parse_if_statement(&mut self) -> Box<ASTNode> {
        self.expect_token(TokenType::If); // 跳过 if

        let condition = self.parse_expression();

        let then_branch = self.parse_block();

        let else_branch = if let TokenType::Else = self.current_token.token_type {
              self.expect_token(TokenType::Else); // 跳过 else
              Some(self.parse_block())
          } else {
              None
          };

        Box::new(ASTNode::IfStatement(condition, then_branch, else_branch))
    }

    fn parse_for_loop(&mut self) -> Box<ASTNode> {
        self.expect_token(TokenType::For); // 跳过 for

        let initializer = if let TokenType::Identifier(_) = &self.current_token.token_type {
            let assignment = self.parse_assignment();
            self.expect_token(TokenType::Semicolon); // 跳过 ;
            Some(assignment)
        } else {
            None
        };
        println!("parse_for_loop, initializer: {:?}", initializer);

        let condition = if self.current_token.token_type != TokenType::Semicolon {
              let expr = self.parse_expression();
              self.expect_token(TokenType::Semicolon); // 跳过 ;
              Some(expr)
          } else {
              self.expect_token(TokenType::Semicolon); // 跳过 ;
              None
          };
        println!("parse_for_loop, condition: {:?}", condition);

        let increment = if self.current_token.token_type != TokenType::RParen {
            let expr = self.parse_assignment();
            Some(expr)
        } else {
            None
        };
        println!("parse_for_loop, increment: {:?}", increment);

        let body = self.parse_block();

        Box::new(ASTNode::ForLoop(initializer, condition, increment, body))
    }

    fn parse_while_loop(&mut self) -> Box<ASTNode> {
        println!("Entering parse_while_loop, current token: {:?}", self.current_token.token_type);
        self.expect_token(TokenType::While); // 跳过 while
 
        let condition = self.parse_expression();

        println!("parse_while_loop, condition: {:?}", condition);
        let body = self.parse_block();

        println!("Exiting parse_while_loop, current token: {:?}", self.current_token.token_type);
        Box::new(ASTNode::WhileLoop(condition, body))
    }

    fn parse_function_def(&mut self) -> Box<ASTNode> {
        self.expect_token(TokenType::Fn); // 跳过 fn

        let name = if let TokenType::Identifier(name) = &self.current_token.token_type {
            name.clone()
        } else {
            panic!("Expected function name")
        };
        self.advance();

        self.expect_token(TokenType::LParen); // 跳过 (
        let params = self.parse_params();
        self.expect_token(TokenType::RParen); // 跳过 )

        let body = self.parse_block();

        Box::new(ASTNode::FunctionDef(name, params, body))
    }

    fn parse_params(&mut self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if self.current_token.token_type != TokenType::RParen {
            loop {
                let name = if let TokenType::Identifier(name) = &self.current_token.token_type {
                    name.clone()
                } else {
                    panic!("Expected parameter name")
                };                   
                self.advance(); // 跳过参数名


                let type_name = match &self.current_token.token_type {
                    TokenType::Int => "int".to_string(),
                    TokenType::Float => "float".to_string(),
                    TokenType::Bool => "bool".to_string(),
                    TokenType::String => "string".to_string(),
                    TokenType::Identifier(type_name) => type_name.clone(),
                    _ => panic!("Expected valid type after colon: {:?}", self.current_token.token_type)
                };
                self.advance(); // 跳过参数类型

                params.push((name, type_name));

                if self.current_token.token_type != TokenType::Comma {
                    break;
                }
                self.expect_token(TokenType::Comma); // 跳过 ,
            }
        }

        params
    }

    fn parse_function_call(&mut self, name: String) -> Box<ASTNode> {
        println!("Entering parse_function_call, name: {:?}, current token: {:?}", name, self.current_token.token_type);
        self.expect_token(TokenType::LParen); // 跳过 (
        let args = self.parse_args();
        self.expect_token(TokenType::RParen); // 跳过 )
        println!("Exiting parse_function_call, current token: {:?}", self.current_token.token_type);
        Box::new(ASTNode::FunctionCall(name, args))
    }

    fn parse_args(&mut self) -> Vec<Box<ASTNode>> {
        println!("Entering parse_args, current token: {:?}", self.current_token.token_type);
        let mut args = Vec::new();

        if self.current_token.token_type != TokenType::RParen {
            loop {
                args.push(self.parse_expression());

                if self.current_token.token_type != TokenType::Comma {
                    break;
                }
                self.expect_token(TokenType::Comma); // 跳过 ,
            }
        }
        println!("Exiting parse_args, current token: {:?}", self.current_token.token_type);
        args
    }

    fn parse_return(&mut self) -> Box<ASTNode> {
        self.expect_token(TokenType::Return); // 跳过 return

        let expr = if self.current_token.token_type != TokenType::Semicolon {
            Some(self.parse_expression())
        } else {
            None
        };

        self.expect_token(TokenType::Semicolon); // 跳过分号

        Box::new(ASTNode::Return(expr))
    }

    fn parse_block(&mut self) -> Box<ASTNode> {
        println!("Entering parse_block, current token: {:?}", self.current_token.token_type);
        self.expect_token(TokenType::LBrace); // 跳过 {
        let mut statements = Vec::new();

        while self.current_token.token_type != TokenType::RBrace {
            statements.push(dbg!(self.parse_statement()));
        }

        self.expect_token(TokenType::RBrace); // 跳过 }

        println!("Exiting parse_block, current token: {:?}", self.current_token.token_type);
        Box::new(ASTNode::Block(statements))
    }

    fn parse_expression(&mut self) -> Box<ASTNode> {
        println!("Entering parse_expression, current token: {:?}", self.current_token.token_type);
        let x= self.parse_logical_or();
        println!("Exiting parse_expression, current token: {:?}", self.current_token.token_type);
        x
    }

    fn parse_logical_or(&mut self) -> Box<ASTNode> {
        let mut left = self.parse_logical_and();

        while let TokenType::Or = self.current_token.token_type {
            self.advance();
            let right = self.parse_logical_and();
            left = Box::new(ASTNode::BinaryExpr(left, "||".to_string(), right));
        }

        left
    }

    fn parse_logical_and(&mut self) -> Box<ASTNode> {
        let mut left = self.parse_equality();

        while let TokenType::And = self.current_token.token_type {
            self.advance();
            let right = self.parse_equality();
            left = Box::new(ASTNode::BinaryExpr(left, "&&".to_string(), right));
        }

        left
    }

    fn parse_equality(&mut self) -> Box<ASTNode> {
        println!("Entering parse_equality, current token: {:?}", self.current_token.token_type);
        let mut left = self.parse_relational();

        while matches!(
            self.current_token.token_type,
            TokenType::Equals | TokenType::NotEquals
        ) {
            let operator = match self.current_token.token_type {
                TokenType::Equals => "==",
                TokenType::NotEquals => "!=",
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_relational();
            left = Box::new(ASTNode::BinaryExpr(left, operator.to_string(), right));
        }

        println!("Exiting parse_equality, left: {:?}", left);
        left
    }

    fn parse_relational(&mut self) -> Box<ASTNode> {
        println!("Entering parse_relational, current token: {:?}", self.current_token.token_type);
        let mut left = self.parse_additive();
        println!("After parse_additive, current token: {:?}", self.current_token.token_type);

        while matches!(
            self.current_token.token_type,
            TokenType::LessThan | TokenType::LessThanOrEqual | TokenType::GreaterThan | TokenType::GreaterThanOrEqual
        ) {
            println!("Found relational operator: {:?}", self.current_token.token_type);
            let operator = match self.current_token.token_type {
                TokenType::LessThan => "<",
                TokenType::LessThanOrEqual => "<=",
                TokenType::GreaterThan => ">",
                TokenType::GreaterThanOrEqual => ">=",
                _ => unreachable!(),
            };
            self.advance();
            println!("After advancing, current token: {:?}", self.current_token.token_type);
            let right = self.parse_additive();
            left = Box::new(ASTNode::BinaryExpr(left, operator.to_string(), right));
        }

        println!("Exiting parse_relational, left: {:?}", left);
        left
    }

    fn parse_additive(&mut self) -> Box<ASTNode> {
        let mut left = self.parse_multiplicative();

        while matches!(
            self.current_token.token_type,
            TokenType::Plus | TokenType::Minus
        ) {
            let operator = match self.current_token.token_type {
                TokenType::Plus => "+",
                TokenType::Minus => "-",
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_multiplicative();
            left = Box::new(ASTNode::BinaryExpr(left, operator.to_string(), right));
        }

        left
    }

    fn parse_multiplicative(&mut self) -> Box<ASTNode> {
        let mut left = self.parse_unary();

        while matches!(
            self.current_token.token_type,
            TokenType::Multiply | TokenType::Divide
        ) {
            let operator = match self.current_token.token_type {
                TokenType::Multiply => "*",
                TokenType::Divide => "/",
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_unary();
            left = Box::new(ASTNode::BinaryExpr(left, operator.to_string(), right));
        }

        left
    }

    fn parse_unary(&mut self) -> Box<ASTNode> {
        if matches!(
            self.current_token.token_type,
            TokenType::Not | TokenType::Minus
        ) {
            let operator = match self.current_token.token_type {
                TokenType::Not => "!",
                TokenType::Minus => "-",
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_unary();
            return Box::new(ASTNode::UnaryExpr(operator.to_string(), right));
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Box<ASTNode> {
        let token_type = &self.current_token.token_type;
        println!("Entering parse_primary, current token: {:?}", token_type);
        match token_type {
            TokenType::IntLiteral(value) => {
                let cloned_value = *value;
                self.advance();
                Box::new(ASTNode::IntLiteral(cloned_value))
            },
            TokenType::FloatLiteral(value) => {
                let cloned_value = *value;
                self.advance();
                Box::new(ASTNode::FloatLiteral(cloned_value))
            },
            TokenType::BoolLiteral(value) => {
                let cloned_value = *value;
                self.advance();
                Box::new(ASTNode::BoolLiteral(cloned_value))
            },
            TokenType::StringLiteral(value) => {
                let cloned_value = value.clone();
                self.advance();
                Box::new(ASTNode::StringLiteral(cloned_value))
            },
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.advance();
                println!("parse_primary, identifier: {:?}", name);
                println!("parse_primary, current token: {:?}", self.current_token.token_type);
                if let TokenType::LParen = self.current_token.token_type {
                    self.parse_function_call(name)
                } else {
                    Box::new(ASTNode::Identifier(name))
                }
            },
            TokenType::LParen => {
                self.advance();
                let expr = self.parse_expression();
                self.expect_token(TokenType::RParen); // 跳过 )
                expr
            },
            TokenType::LBrace => {
                // LBrace 应该在 parse_statement 方法中被处理
                // 但如果它到达了这里，我们需要适当处理
                self.parse_block()
            },
            _ => panic!("Unexpected token in primary expression: {:?}", token_type),
        }
    }

    fn parse_assignment_with_identifier(&mut self, identifier: String) -> Box<ASTNode> {
        match self.current_token.token_type {
            TokenType::Equal => {
                self.advance(); // 跳过=
                let expr = self.parse_expression();
                Box::new(ASTNode::Assignment(identifier, expr))
            }
            TokenType::Increment => {
                self.advance(); // 跳过++
                Box::new(ASTNode::Increment(identifier))
            }
            TokenType::Decrement => {
                self.advance(); // 跳过--
                Box::new(ASTNode::Decrement(identifier))
            }
            _ => {
                panic!("Unexpected token in assignment: {:?}", self.current_token.token_type)
            }
        }
    }

    fn parse_assignment(&mut self) -> Box<ASTNode> {
        println!("parse_assignment, current token: {:?}", self.current_token.token_type);
        if let TokenType::Identifier(name) = self.current_token.token_type.clone() {
            self.advance(); // 跳过标识符
            self.parse_assignment_with_identifier(name)
        } else {
            panic!("Expected identifier in assignment")
        }
    }

    fn parse_assignment_or_function_call(&mut self) -> Box<ASTNode> {
        println!("parse_statement, identifier token: {:?}", self.current_token.token_type);
        // 保存当前token以便后续使用
        let identifier_token = self.current_token.clone();
        if let TokenType::Identifier(name) = &identifier_token.token_type {
            // 查看下一个token
            self.advance();

            match &self.current_token.token_type {
                TokenType::Equal |
                TokenType::Increment |
                TokenType::Decrement => self.parse_assignment_with_identifier(name.clone()),
                TokenType::LParen => self.parse_function_call(name.clone()),
                _ => panic!("Unexpected token after identifier: {:?}", self.current_token.token_type),
            }
        } else {
            panic!("Expected identifier in assignment or function call")
        }
    }
}