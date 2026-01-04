mod ast;
mod bin_format;
mod compiler;
mod ffi;
mod lexer;
mod parser;
mod token;
mod vm;

// 公共 API 导出
pub use ast::*;
pub use bin_format::{load_from_file, save_to_file};
pub use compiler::{CompileResult, Compiler, Constant, GlobalVarInfo, FunctionInfo, OpCode};
pub use lexer::Lexer;
pub use parser::Parser;
pub use token::TokenType;
pub use vm::VM;
pub use ffi::{DkvScriptCompileResult, DkvScriptVM}; // （不需要 pub use FFI 函数，因为已经用 #[no_mangle] 标记）

#[derive(Debug, Clone, Copy)]
#[repr(u16)]
enum SYSCALL {
    PRINT = 0x01,
    DKVCOMMAND = 0x02,
}

impl From<u16> for SYSCALL {
    fn from(value: u16) -> Self {
        match value {
            0x01 => SYSCALL::PRINT,
            0x02 => SYSCALL::DKVCOMMAND,
            _ => panic!("Invalid syscall id"),
        }
    }
}

/// 编译源代码的便捷函数
pub fn do_compile(source: &str) -> Result<CompileResult, Box<dyn std::error::Error>> {
    // 词法分析
    let lexer = Lexer::new(source.to_string());
    
    // 语法分析
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    
    // 编译
    let compiler = Compiler::new();
    let compile_result = compiler.compile(&ast);
    
    Ok(compile_result)
}
