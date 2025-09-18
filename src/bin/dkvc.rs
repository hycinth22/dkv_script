use dkv_script::*;
use std::env;
use std::fs;
use std::path::Path;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: dkvc <command> <file>");
        println!("Commands:");
        println!("  compile    Compile DKV script to binary");
        println!("  run        Run DKV script file");
        println!("  execute    Execute compiled binary file");
        println!("  tokenize   Display token sequence for debugging");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "compile" => {
            if let Err(err) = compile_file(file_path) {
                eprintln!("Error compiling file: {}", err);
            }
        },
        "run" => {
            if let Err(err) = run_file(file_path) {
                eprintln!("Error running file: {}", err);
            }
        },
        "execute" => {
            if let Err(err) = execute_file(file_path) {
                eprintln!("Error executing file: {}", err);
            }
        },
        "tokenize" => {
            if let Err(err) = tokenize_file(file_path) {
                eprintln!("Error tokenizing file: {}", err);
            }
        },
        _ => {
            println!("Unknown command: {}", command);
        },
    }
}

fn tokenize_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string(file_path)?;
    let mut lexer = Lexer::new(source);
    
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        tokens.push(token.clone());
        if let TokenType::Eof = token.token_type {
            break;
        }
    }
    
    println!("Tokens:");
    for token in tokens {
        println!("{:?} at line {}, column {}", token.token_type, token.line, token.column);
    }
    
    Ok(())
}

fn compile_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 读取源文件
    let source = fs::read_to_string(file_path)?;
    
    // 创建输出文件路径
    let output_path = Path::new(file_path).with_extension("cdkvs");
    
    // 执行编译
    let compile_result = do_compile(&source)?;
    
    // 保存编译结果
    save_to_file(&compile_result, &output_path.to_string_lossy())?;
    
    println!("Compilation successful. Output: {}", output_path.to_string_lossy());
    Ok(())
}

fn run_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 读取源文件
    let source = fs::read_to_string(file_path)?;
    
    // 执行编译
    let compile_result = do_compile(&source)?;
    
    // 运行程序
    let mut vm = VM::new(compile_result);
    vm.run();
    
    Ok(())
}

fn execute_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 加载编译后的二进制文件
    let compile_result = load_from_file(file_path)?;
    
    // 运行程序
    let mut vm = VM::new(compile_result);
    vm.run();
    
    Ok(())
}
