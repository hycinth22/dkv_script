use dkv_script::{Lexer, Parser, Compiler, OpCode};

#[test]
fn test_compiler_constant() {
    let source = "let x: int = 42;";
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    
    let mut compiler = Compiler::new();
    
    let compiled_chunk = compiler.compile(&ast);
    let compiled_fn = &compiled_chunk.functions[0];
    
    // 验证函数名
    assert_eq!(compiled_fn.name, "_entrypoint");
    
    // 验证字节码长度
    assert!(compiled_fn.bytecode.len() >= 2);
    // 验证第一个指令是 LoadConst
    assert_eq!(compiled_fn.bytecode[0], OpCode::LoadConst as u8);
    // 验证常量池中有值
    assert!(!compiled_chunk.constants.is_empty());
}

#[test]
fn test_compiler_variable_declaration() {
    let source = "let x: int = 42;";
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    
    let mut compiler = Compiler::new();
    let compiled_chunk = compiler.compile(&ast);
    
    // 验证函数被编译
    assert!(!compiled_chunk.functions.is_empty());
    let func = &compiled_chunk.functions[0];
    // 验证字节码包含相应指令
    assert!(func.bytecode.contains(&(OpCode::LoadConst as u8)));
    assert!(func.bytecode.contains(&(OpCode::StoreGlobal as u8)));
    // 验证常量池中有值
    assert!(!compiled_chunk.constants.is_empty());
}

#[test]
fn test_compiler_assignment() {
    let source = "let x: int; x = 42;";
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    
    let mut compiler = Compiler::new();
    let compiled_chunk = compiler.compile(&ast);
    
    // 验证函数被编译
    assert!(!compiled_chunk.functions.is_empty());
    let func = &compiled_chunk.functions[0];
    // 验证字节码包含相应指令
    assert!(func.bytecode.contains(&(OpCode::LoadConst as u8)));
    assert!(func.bytecode.contains(&(OpCode::StoreGlobal as u8)));
}

#[test]
fn test_compiler_binary_expression() {
    let source = "let x: int = 1 + 2;";
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    
    let mut compiler = Compiler::new();
    let compiled_chunk = compiler.compile(&ast);
    
    // 验证函数被编译
    assert!(!compiled_chunk.functions.is_empty());
    let func = &compiled_chunk.functions[0];
    // 验证字节码包含相应指令
    assert!(func.bytecode.contains(&(OpCode::LoadConst as u8)));
    assert!(func.bytecode.contains(&(OpCode::Add as u8)));
    // 验证常量池中有两个值
    assert!(compiled_chunk.constants.len() >= 2);
}

#[test]
fn test_compiler_function_definition() {
    let source = "fn add(a int, b int) { return a + b; }";
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    
    let mut compiler = Compiler::new();
    let compiled_chunk = compiler.compile(&ast);
    
    // 验证函数被编译
    assert!(!compiled_chunk.functions.is_empty());
    // 验证函数包含参数
    let func = &compiled_chunk.functions[0];
    assert_eq!(func.param_count, 2);
}