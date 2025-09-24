use std::os::linux::raw::stat;

use dkv_script::{Lexer, Parser, ASTNode};

#[test]
fn test_parser_int_literal() {
    let source = "let x: int = 42;";
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    
    match *ast {
        ASTNode::Program(ref statements) => {
            let mut count = 0;
            statements.iter().filter_map(|stmt| {
                match stmt.as_ref() {
                    ASTNode::VariableDecl(name, _type, initializer) if name == "x" => Some(initializer),
                    _ => None,
                }
            }).for_each(|initializer| {
                count += 1;
                let initializer = &**initializer.as_ref().unwrap();
                match initializer {
                    ASTNode::IntLiteral(value) => assert_eq!(*value, 42),
                    _ => panic!("Expected IntLiteral"),
                }
            });
            assert_eq!(count, 1);
        },
        _ => panic!("Expected Program"),
    }
}

#[test]
fn test_parser_float_literal() {
    let source = "let x: float = 3.14;";
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    
    match *ast {
        ASTNode::Program(ref statements) => {
            let mut count = 0;
            statements.iter().filter_map(|stmt| {
                match stmt.as_ref() {
                    ASTNode::VariableDecl(name, _type, initializer) if name == "x" => Some(initializer),
                    _ => None,
                }
            }).for_each(|initializer| {
                count += 1;
                let initializer = &**initializer.as_ref().unwrap();
                match initializer {
                    ASTNode::FloatLiteral(value) => assert!((*value - 3.14).abs() < 0.001),
                    _ => panic!("Expected FloatLiteral"),
                }
            });
            assert_eq!(count, 1);
        },
        _ => panic!("Expected Program"),
    }
}

#[test]
fn test_parser_bool_literal() {
    let source = "let x: bool = true; let y: bool = false;";
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    
    match *ast {
        ASTNode::Program(ref statements) => {
            let mut count = 0;
            statements.iter().filter_map(|stmt| {
                match stmt.as_ref() {
                    ASTNode::VariableDecl(name, _type, initializer) if name == "x" => Some(initializer),
                    _ => None,
                }
            }).for_each(|initializer| {
                count += 1;
                let initializer = &**initializer.as_ref().unwrap();
                match initializer {
                    ASTNode::BoolLiteral(value) => assert_eq!(*value, true),
                    _ => panic!("Expected BoolLiteral"),
                }
            });
            statements.iter().filter_map(|stmt| {
                match stmt.as_ref() {
                    ASTNode::VariableDecl(name, _type, initializer) if name == "y" => Some(initializer),
                    _ => None,
                }
            }).for_each(|initializer| {
                count += 1;
                let initializer = &**initializer.as_ref().unwrap();
                match initializer {
                    ASTNode::BoolLiteral(value) => assert_eq!(*value, false),
                    _ => panic!("Expected BoolLiteral"),
                }
            });
            assert_eq!(count, 2);
        },
        _ => panic!("Expected Program"),
    }
}

#[test]
fn test_parser_string_literal() {
    let source = "let x: string = \"hello world\";";
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    
    match *ast {
        ASTNode::Program(ref statements) => {
            let mut count = 0;
            statements.iter().filter_map(|stmt| {
                match stmt.as_ref() {
                    ASTNode::VariableDecl(name, _type, initializer) if name == "x" => Some(initializer),
                    _ => None,
                }
            }).for_each(|initializer| {
                count += 1;
                let initializer = &**initializer.as_ref().unwrap();
                match initializer {
                    ASTNode::StringLiteral(ref value) => assert_eq!(value, "hello world"),
                    _ => panic!("Expected StringLiteral"),
                }
            });
            assert_eq!(count, 1);
        },
        _ => panic!("Expected Program"),
    }
}

#[test]
fn test_parser_variable_declaration() {
    let source = "let x: int = 42;";
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    
    match *ast {
        ASTNode::Program(ref statements) => {
            assert_eq!(statements.len(), 1);
            match statements[0].as_ref() {
                ASTNode::VariableDecl(ref name, ref type_name, Some(ref expr)) => {
                    assert_eq!(name, "x");
                    assert_eq!(type_name, "int");
                    match expr.as_ref() {
                        ASTNode::IntLiteral(value) => assert_eq!(*value, 42),
                        _ => panic!("Expected IntLiteral"),
                    }
                },
                _ => panic!("Expected VariableDecl"),
            }
        },
        _ => panic!("Expected Program"),
    }
}

#[test]
fn test_parser_assignment() {
    let source = "let x: int; x = 42;";
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    
    match *ast {
        ASTNode::Program(ref statements) => {
            for stat in statements {
                match stat.as_ref() {
                    ASTNode::Assignment(ref name, ref expr) => {
                        assert_eq!(name, "x");
                        match expr.as_ref() {
                            ASTNode::IntLiteral(value) => assert_eq!(*value, 42),
                            _ => panic!("Expected IntLiteral"),
                        }
                    },
                    _ => {},
                }
            }
        },
        _ => panic!("Expected Program"),
    }
}

#[test]
fn test_parser_binary_expression() {
    let source = "let x: int = 1 + 2 * 3;";
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    
    match *ast {
        ASTNode::Program(ref statements) => {
            let mut count = 0;
            statements.iter().filter_map(|stmt| {
                match stmt.as_ref() {
                    ASTNode::VariableDecl(name, _type, initializer) if name == "x" => Some(initializer),
                    _ => None,
                }
            }).for_each(|expr| {
                count += 1;
                let expr = &**expr.as_ref().unwrap();
                match expr {
                    ASTNode::BinaryExpr(ref left, ref op, ref right) => {
                        assert_eq!(op, "+");
                        match left.as_ref() {
                            ASTNode::IntLiteral(value) => assert_eq!(*value, 1),
                            _ => panic!("Expected IntLiteral"),
                        }
                        match right.as_ref() {
                            ASTNode::BinaryExpr(ref nested_left, ref nested_op, ref nested_right) => {
                                assert_eq!(nested_op, "*");
                                match nested_left.as_ref() {
                                    ASTNode::IntLiteral(value) => assert_eq!(*value, 2),
                                    _ => panic!("Expected IntLiteral"),
                                }
                                match nested_right.as_ref() {
                                    ASTNode::IntLiteral(value) => assert_eq!(*value, 3),
                                    _ => panic!("Expected IntLiteral"),
                                }
                            },
                            _ => panic!("Expected BinaryExpr"),
                        }
                    },
                    _ => panic!("Expected BinaryExpr"),
                }
            });
            assert_eq!(count, 1);
        },
        _ => panic!("Expected Program"),
    }
}

#[test]
fn test_parser_function_definition() {
    let source = "fn add(a int, b int) { return a + b; }";
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    
    match *ast {
        ASTNode::Program(ref statements) => {
            assert_eq!(statements.len(), 1);
            match statements[0].as_ref() {
                ASTNode::FunctionDef(ref name, ref params, ref body) => {
                    assert_eq!(name, "add");
                    assert_eq!(params.len(), 2);
                    assert_eq!(params[0].0, "a");
                    assert_eq!(params[0].1, "int");
                    assert_eq!(params[1].0, "b");
                    assert_eq!(params[1].1, "int");
                    
                    match body.as_ref() {
                        ASTNode::Block(ref block_statements) => {
                            assert_eq!(block_statements.len(), 1);
                            match block_statements[0].as_ref() {
                                ASTNode::Return(Some(ref expr)) => {
                                    match expr.as_ref() {
                                        ASTNode::BinaryExpr(ref left, ref op, ref right) => {
                                            assert_eq!(op, "+");
                                            match left.as_ref() {
                                                ASTNode::Identifier(ref id) => assert_eq!(id, "a"),
                                                _ => panic!("Expected Identifier"),
                                            }
                                            match right.as_ref() {
                                                ASTNode::Identifier(ref id) => assert_eq!(id, "b"),
                                                _ => panic!("Expected Identifier"),
                                            }
                                        },
                                        _ => panic!("Expected BinaryExpr"),
                                    }
                                },
                                _ => panic!("Expected Return"),
                            }
                        },
                        _ => panic!("Expected Block"),
                    }
                },
                _ => panic!("Expected FunctionDef"),
            }
        },
        _ => panic!("Expected Program"),
    }
}