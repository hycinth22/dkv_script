use dkv_script::*;
use std::env;
use std::fs;
use std::io::{self, BufRead, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        1 => {
            // 没有参数，启动交互式解释器
            run_interactive_mode();
        },
        2 => {
            // 有一个参数，执行指定文件
            if let Err(err) = run_file(&args[1]) {
                eprintln!("Error running file: {}", err);
            }
        },
        _ => {
            println!("Usage:");
            println!("  interpreter_cdvks         Start interactive DKV script interpreter");
            println!("  interpreter_cdvks <file>  Run DKV script from file");
        }
    }
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

fn run_interactive_mode() {
    println!("DKV Script Interactive Interpreter");
    println!("Type 'exit' to quit.");
    println!("\nNote: Interactive mode only supports single expressions per line.");
    println!("For complex scripts, please use a file.");
    
    let stdin = io::stdin();
    
    loop {
        print!("dkv> ");
        std::io::stdout().flush().unwrap();
        
        let mut line = String::new();
        if stdin.lock().read_line(&mut line).unwrap_or(0) == 0 {
            break;
        }
        
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        if line == "exit" || line == "quit" {
            break;
        }
        
        // 在交互式模式下，我们需要将单行表达式包装成一个完整的程序
        // 简单地添加一个 print 语句来显示结果
        let program = format!("print({});", line);
        
        match do_compile(&program) {
            Ok(compile_result) => {
                let mut vm = VM::new(compile_result);
                vm.run();
            },
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }
    
    println!("Exiting interpreter.");
}