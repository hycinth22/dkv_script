use dkv_script::{Lexer, TokenType};

#[test]
fn test_lexer_basic_tokens() {
    let source = "( ) { } ; , :";
    let mut lexer = Lexer::new(source.to_string());
    
    assert_eq!(lexer.next_token().token_type, TokenType::LParen);
    assert_eq!(lexer.next_token().token_type, TokenType::RParen);
    assert_eq!(lexer.next_token().token_type, TokenType::LBrace);
    assert_eq!(lexer.next_token().token_type, TokenType::RBrace);
    assert_eq!(lexer.next_token().token_type, TokenType::Semicolon);
    assert_eq!(lexer.next_token().token_type, TokenType::Comma);
    assert_eq!(lexer.next_token().token_type, TokenType::Colon);
    assert_eq!(lexer.next_token().token_type, TokenType::Eof);
}

#[test]
fn test_lexer_operators() {
    let source = "= + - * / == != < > <= >= !";
    let mut lexer = Lexer::new(source.to_string());
    
    assert_eq!(lexer.next_token().token_type, TokenType::Equal);
    assert_eq!(lexer.next_token().token_type, TokenType::Plus);
    assert_eq!(lexer.next_token().token_type, TokenType::Minus);
    assert_eq!(lexer.next_token().token_type, TokenType::Multiply);
    assert_eq!(lexer.next_token().token_type, TokenType::Divide);
    assert_eq!(lexer.next_token().token_type, TokenType::Equals);
    assert_eq!(lexer.next_token().token_type, TokenType::NotEquals);
    assert_eq!(lexer.next_token().token_type, TokenType::LessThan);
    assert_eq!(lexer.next_token().token_type, TokenType::GreaterThan);
    assert_eq!(lexer.next_token().token_type, TokenType::LessThanOrEqual);
    assert_eq!(lexer.next_token().token_type, TokenType::GreaterThanOrEqual);
    assert_eq!(lexer.next_token().token_type, TokenType::Not);
    assert_eq!(lexer.next_token().token_type, TokenType::Eof);
}

#[test]
fn test_lexer_keywords_and_identifiers() {
    let source = "fn let if else for while return true false int float bool string";
    let mut lexer = Lexer::new(source.to_string());
    
    assert_eq!(lexer.next_token().token_type, TokenType::Fn);
    assert_eq!(lexer.next_token().token_type, TokenType::Let);
    assert_eq!(lexer.next_token().token_type, TokenType::If);
    assert_eq!(lexer.next_token().token_type, TokenType::Else);
    assert_eq!(lexer.next_token().token_type, TokenType::For);
    assert_eq!(lexer.next_token().token_type, TokenType::While);
    assert_eq!(lexer.next_token().token_type, TokenType::Return);
    match lexer.next_token().token_type {
        TokenType::BoolLiteral(value) => assert_eq!(value, true),
        _ => panic!("Expected BoolLiteral(true)"),
    }
    match lexer.next_token().token_type {
        TokenType::BoolLiteral(value) => assert_eq!(value, false),
        _ => panic!("Expected BoolLiteral(false)"),
    }
    assert_eq!(lexer.next_token().token_type, TokenType::Int);
    assert_eq!(lexer.next_token().token_type, TokenType::Float);
    assert_eq!(lexer.next_token().token_type, TokenType::Bool);
    assert_eq!(lexer.next_token().token_type, TokenType::String);
    assert_eq!(lexer.next_token().token_type, TokenType::Eof);
}

#[test]
fn test_lexer_literals() {
    let source = "42 3.14 true false \"hello\"";
    let mut lexer = Lexer::new(source.to_string());
    
    match lexer.next_token().token_type {
        TokenType::IntLiteral(value) => assert_eq!(value, 42),
        _ => panic!("Expected IntLiteral"),
    }
    
    match lexer.next_token().token_type {
        TokenType::FloatLiteral(value) => assert!((value - 3.14).abs() < 0.001),
        _ => panic!("Expected FloatLiteral"),
    }
    
    match lexer.next_token().token_type {
        TokenType::BoolLiteral(value) => assert_eq!(value, true),
        _ => panic!("Expected BoolLiteral(true)"),
    }
    match lexer.next_token().token_type {
        TokenType::BoolLiteral(value) => assert_eq!(value, false),
        _ => panic!("Expected BoolLiteral(false)"),
    }
    
    match lexer.next_token().token_type {
        TokenType::StringLiteral(value) => assert_eq!(value, "hello"),
        _ => panic!("Expected StringLiteral"),
    }
    
    assert_eq!(lexer.next_token().token_type, TokenType::Eof);
}

#[test]
fn test_lexer_increment_decrement() {
    let source = "count++ count-- i = i + 1; j = j - 2;";
    let mut lexer = Lexer::new(source.to_string());
    
    match lexer.next_token().token_type {
        TokenType::Identifier(name) => assert_eq!(name, "count"),
        _ => panic!("Expected Identifier"),
    }
    assert_eq!(lexer.next_token().token_type, TokenType::Increment);
    
    match lexer.next_token().token_type {
        TokenType::Identifier(name) => assert_eq!(name, "count"),
        _ => panic!("Expected Identifier"),
    }
    assert_eq!(lexer.next_token().token_type, TokenType::Decrement);
    
    match lexer.next_token().token_type {
        TokenType::Identifier(name) => assert_eq!(name, "i"),
        _ => panic!("Expected Identifier"),
    }
    assert_eq!(lexer.next_token().token_type, TokenType::Equal);

    match lexer.next_token().token_type {
        TokenType::Identifier(name) => assert_eq!(name, "i"),
        _ => panic!("Expected Identifier"),
    }
    assert_eq!(lexer.next_token().token_type, TokenType::Plus);
    
    match lexer.next_token().token_type {
        TokenType::IntLiteral(value) => assert_eq!(value, 1),
        _ => panic!("Expected IntLiteral"),
    }
    assert_eq!(lexer.next_token().token_type, TokenType::Semicolon);
    
    match lexer.next_token().token_type {
        TokenType::Identifier(name) => assert_eq!(name, "j"),
        _ => panic!("Expected Identifier"),
    }
    assert_eq!(lexer.next_token().token_type, TokenType::Equal);
    match lexer.next_token().token_type {
        TokenType::Identifier(name) => assert_eq!(name, "j"),
        _ => panic!("Expected Identifier"),
    }
    assert_eq!(lexer.next_token().token_type, TokenType::Minus);
    match lexer.next_token().token_type {
        TokenType::IntLiteral(value) => assert_eq!(value, 2),
        _ => panic!("Expected IntLiteral"),
    }
    assert_eq!(lexer.next_token().token_type, TokenType::Semicolon);

    
    assert_eq!(lexer.next_token().token_type, TokenType::Eof);
}