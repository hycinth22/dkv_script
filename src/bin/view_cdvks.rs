use std::env;
use dkv_script::{load_from_file};
use dkv_script::Constant;

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: view_cdvks <file.cdkvs>");
        return;
    }

    let file_path = &args[1];
    let compile_result = match load_from_file(file_path) {
        Ok(result) => result,
        Err(err) => {
            println!("Error loading file {}: {}", file_path, err);
            return;
        },
    };

    println!("=== DKV Binary File Info ===");
    println!("File: {}", file_path);
    println!("Version: 1.0");
    println!("Entry Point: Function #{}", compile_result.entrypoint);
    println!();

    // 打印常量池
    println!("=== Constant Pool ({}) ===", compile_result.constants.len());
    for (i, constant) in compile_result.constants.iter().enumerate() {
        match constant {
            Constant::Nil => {
                println!("#{}: NIL", i);
            }
            Constant::Int(value) => {
                println!("#{}: INT = {}", i, value);
            },
            Constant::Float(value) => {
                println!("#{}: FLOAT = {}", i, value);
            },
            Constant::Bool(value) => {
                println!("#{}: BOOL = {}", i, value);
            },
            Constant::String(value) => {
                println!("#{}: STRING = \"{}\"", i, value);
            },
        }
    }
    println!();

    // 打印全局变量
    println!("=== Global Variables ({}) ===", compile_result.global_vars.len());
    for global_var in &compile_result.global_vars {
        println!("{}: (const #{:?})
", global_var.name, global_var.const_index);
    }
    println!();

    // 打印函数表
    println!("=== Functions ({}) ===", compile_result.functions.len());
    for (i, func) in compile_result.functions.iter().enumerate() {
        println!("Function #{}: {}", i, func.name);
        println!("  Parameters: {}", func.param_count);
        println!("  Local Variables: {}", func.local_count);
        println!("  Bytecode Size: {} bytes", func.bytecode.len());
        
        // 打印字节码（可选）
        if !func.bytecode.is_empty() {
            println!("  Bytecode:");
            print_bytecode(&func.bytecode);
        }
        println!();
    }
    
    println!("=== End of File Info ===");
}

// 打印字节码的辅助函数
fn print_bytecode(bytecode: &[u8]) {
    println!("{:4}  {:10}\t\tARG", "  PC", "OPCODE");
    let mut i = 0;
    while i < bytecode.len() {
        
        let opcode = bytecode[i];
        let opcode_name = match opcode {
            0x01 => "LoadConst",
            0x02 => "LoadGlobal",
            0x03 => "StoreGlobal",
            0x04 => "LoadLocal",
            0x05 => "StoreLocal",

            0x10 => "Not",
            0x11 => "Inc",
            0x12 => "Dec",
            0x13 => "Neg",

            0x1A => "Add",
            0x1B => "Sub",
            0x1C => "Mul",
            0x1D => "Div",

            0x20 => "CmpEq",
            0x21 => "CmpNe",
            0x22 => "CmpLt",
            0x23 => "CmpLe",
            0x24 => "CmpGt",
            0x25 => "CmpGe",

            0x50 => "Jmp",
            0x51 => "Jz",

            0x60 => "Call",
            0x61 => "Ret",

            0xFE => "Syscall",
            0xFF => "Exit",
            _ => "Unknown",
        };
        
        // 打印pc计数器
        print!("{:4}  ", i);
        
        // 打印指令
        print!("0x{:04X}({})\t\t", opcode, opcode_name);
        
        // 读取并打印操作数
        let mut operand_bytes = [0u8; 8];
        if i < bytecode.len() - 8 {
            operand_bytes.copy_from_slice(&bytecode[i+1..i+9]);
            let operand = u64::from_le_bytes(operand_bytes);
            print!("{:016X}", operand);
        }
        
        // 每个指令一行
        println!();
        
        i += 1;
        // 跳过操作数
        i += 8;
    }
    if bytecode.len() % 10 != 0 {
        println!();
    }
}