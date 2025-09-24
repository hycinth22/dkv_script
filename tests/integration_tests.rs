use dkv_script::{do_compile, VM};

#[test]
fn test_integration_simple_program() {
    let source = "let a: int = 10; let b: int = 20; let c: int = a + b;";
    let compile_result = do_compile(source).unwrap();
    
    let mut vm = VM::new(compile_result);
    vm.run();
    
    // 由于VM没有提供直接访问局部变量的方法，我们无法直接验证变量值
    // 但至少我们验证了程序能够正常执行而不崩溃
}

#[test]
fn test_integration_function_call() {
    let source = "fn add(a int, b int) { return a + b; } let result: int = add(10, 20);";
    let compile_result = do_compile(source).unwrap();
    
    let mut vm = VM::new(compile_result);
    vm.run();
    
    // 由于VM没有提供直接访问局部变量的方法，我们无法直接验证变量值
    // 但至少我们验证了程序能够正常执行而不崩溃
}

#[test]
fn test_integration_while_loop() {
    let source = "let i: int = 0; let sum: int = 0; while i < 5 { sum = sum + i; i = i + 1; }";
    let compile_result = do_compile(source).unwrap();
    
    let mut vm = VM::new(compile_result);
    vm.run();
    
    // 由于VM没有提供直接访问局部变量的方法，我们无法直接验证变量值
    // 但至少我们验证了程序能够正常执行而不崩溃
}

#[test]
fn test_integration_nested_expressions() {
    let source = "let a: int = 10; let b: int = 20; let c: int = 30; let result: int = a + (b * c) - 5;";
    let compile_result = do_compile(source).unwrap();
    
    let mut vm = VM::new(compile_result);
    vm.run();
    
    // 由于VM没有提供直接访问局部变量的方法，我们无法直接验证变量值
    // 但至少我们验证了程序能够正常执行而不崩溃
}