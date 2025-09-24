use dkv_script::{Lexer, Parser, Compiler, VM};

#[test]
fn test_vm_constant() {
    let source = "let x: int = 42;";
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    
    let compiler = Compiler::new();
    let compile_result = compiler.compile(&ast);
    
    let mut vm = VM::new(compile_result);
    vm.run();
}

#[test]
fn test_vm_binary_operations() {
    // 测试加法
    let source = "let x: int = 1 + 2;";
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    
    let compiler = Compiler::new();
    let compile_result = compiler.compile(&ast);
    
    let mut vm = VM::new(compile_result);
    vm.run();
    
    // 测试减法
    let source = "let x: int = 5 - 3;";
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    
    let compiler = Compiler::new();
    let compile_result = compiler.compile(&ast);
    
    let mut vm = VM::new(compile_result);
    vm.run();

    // 测试乘法
    let source = "let x: int = 3 * 4;";
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    
    let compiler = Compiler::new();
    let compile_result = compiler.compile(&ast);
    
    let mut vm = VM::new(compile_result);
    vm.run();
    
    // 测试除法
    let source = "let x: int = 10 / 2;";
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    
    let compiler = Compiler::new();
    let compile_result = compiler.compile(&ast);
    
    let mut vm = VM::new(compile_result);
    vm.run();
}

#[test]
fn test_vm_variable_declaration_and_assignment() {
    let source = "let x: int = 42; x = x + 1;";
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    
    let compiler = Compiler::new();
    let compile_result = compiler.compile(&ast);
    
    let mut vm = VM::new(compile_result);
    vm.run();
}