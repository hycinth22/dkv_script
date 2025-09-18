use crate::compiler::{CompileResult, Constant, FunctionInfo, GlobalVarInfo};
use std::fs::File;
use std::io::{Read, Write};

// 魔数 "SBYT"
const MAGIC_NUMBER: [u8; 4] = [0x53, 0x42, 0x59, 0x54];
// 版本号 1.0
const VERSION: [u8; 2] = [0x01, 0x00];

// 常量类型
const CONST_TYPE_NIL: u8 = 0;
const CONST_TYPE_INT: u8 = 1;
const CONST_TYPE_FLOAT: u8 = 2;
const CONST_TYPE_BOOL: u8 = 3;
const CONST_TYPE_STRING: u8 = 4;

// 保存编译结果到二进制文件
pub fn save_to_file(compile_result: &CompileResult, file_path: &str) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;

    // 写入文件头
    file.write_all(&MAGIC_NUMBER)?;
    file.write_all(&VERSION)?;
    file.write_all(&compile_result.entrypoint.to_le_bytes())?;

    // 写入常量池
    let const_count = compile_result.constants.len() as u16;
    file.write_all(&const_count.to_le_bytes())?;
    for constant in &compile_result.constants {
        match constant {
            Constant::Nil => {
                file.write_all(&[CONST_TYPE_NIL])?;
            }
            Constant::Int(value) => {
                file.write_all(&[CONST_TYPE_INT])?;
                file.write_all(&value.to_le_bytes())?;
            },
            Constant::Float(value) => {
                file.write_all(&[CONST_TYPE_FLOAT])?;
                file.write_all(&value.to_le_bytes())?;
            },
            Constant::Bool(value) => {
                file.write_all(&[CONST_TYPE_BOOL])?;
                file.write_all(&[if *value { 1 } else { 0 }])?;
            },
            Constant::String(value) => {
                file.write_all(&[CONST_TYPE_STRING])?;
                let len = value.len() as u16;
                file.write_all(&len.to_le_bytes())?;
                file.write_all(value.as_bytes())?;
            },
        }
    }

    // 写入全局变量
    let global_count = compile_result.global_vars.len() as u16;
    file.write_all(&global_count.to_le_bytes())?;
    for global_var in &compile_result.global_vars {
        // 这里简化处理，我们假设所有全局变量都是整数类型
        // 实际上应该根据变量的实际类型写入
        file.write_all(&[CONST_TYPE_INT])?;
        let name_len = global_var.name.len() as u16;
        file.write_all(&name_len.to_le_bytes())?;
        file.write_all(global_var.name.as_bytes())?;
        let const_val = if let Some(const_index) = global_var.const_index {
            const_index.to_le_bytes()
        } else {
            0u16.to_le_bytes()
        };
        file.write_all(&const_val)?;
    }

    // 写入函数表
    let func_count = compile_result.functions.len() as u16;
    file.write_all(&func_count.to_le_bytes())?;
    for func in &compile_result.functions {
        let name_len = func.name.len() as u16;
        file.write_all(&name_len.to_le_bytes())?;
        file.write_all(func.name.as_bytes())?;
        file.write_all(&[func.param_count])?;
        file.write_all(&[func.local_count])?;
        let bytecode_len = func.bytecode.len() as u16;
        file.write_all(&bytecode_len.to_le_bytes())?;
        file.write_all(&func.bytecode)?;
    }

    Ok(())
}

// 从二进制文件加载程序
pub fn load_from_file(file_path: &str) -> std::io::Result<CompileResult> {
    let mut file = File::open(file_path)?;

    // 读取文件头
    let mut magic_number = [0u8; 4];
    file.read_exact(&mut magic_number)?;
    if magic_number != MAGIC_NUMBER {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid file format",
        ));
    }

    let mut version = [0u8; 2];
    file.read_exact(&mut version)?;
    if version != VERSION {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Unsupported version",
        ));
    }

    let mut entrypoint = [0u8; 2];
    file.read_exact(&mut entrypoint)?;
    let entrypoint = u16::from_le_bytes(entrypoint);

    // 读取常量池
    let mut const_count = [0u8; 2];
    file.read_exact(&mut const_count)?;
    let const_count = u16::from_le_bytes(const_count);
    let mut constants = Vec::new();
    for _ in 0..const_count {
        let mut const_type = [0u8; 1];
        file.read_exact(&mut const_type)?;
        match const_type[0] {
            CONST_TYPE_NIL => {
                constants.push(Constant::Nil);
            },
            CONST_TYPE_INT => {
                let mut value = [0u8; 4];
                file.read_exact(&mut value)?;
                constants.push(Constant::Int(i32::from_le_bytes(value)));
            },
            CONST_TYPE_FLOAT => {
                let mut value = [0u8; 4];
                file.read_exact(&mut value)?;
                constants.push(Constant::Float(f32::from_le_bytes(value)));
            },
            CONST_TYPE_BOOL => {
                let mut value = [0u8; 1];
                file.read_exact(&mut value)?;
                constants.push(Constant::Bool(value[0] != 0));
            },
            CONST_TYPE_STRING => {
                let mut len = [0u8; 2];
                file.read_exact(&mut len)?;
                let len = u16::from_le_bytes(len);
                let mut value = vec![0u8; len as usize];
                file.read_exact(&mut value)?;
                let value = String::from_utf8(value).map_err(|_| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8 string")
                })?;
                constants.push(Constant::String(value));
            },
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Unknown constant type",
                ));
            },
        }
    }

    // 读取全局变量
    let mut global_count = [0u8; 2];
    file.read_exact(&mut global_count)?;
    let global_count = u16::from_le_bytes(global_count);
    let mut global_vars = Vec::new();
    for _ in 0..global_count {
        let mut var_type = [0u8; 1];
        file.read_exact(&mut var_type)?;
        let mut name_len = [0u8; 2];
        file.read_exact(&mut name_len)?;
        let name_len = u16::from_le_bytes(name_len);
        let mut name = vec![0u8; name_len as usize];
        file.read_exact(&mut name)?;
        let name = String::from_utf8(name).map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8 string")
        })?;
        let mut const_index = [0u8; 2];
        file.read_exact(&mut const_index)?;
        let const_index = u16::from_le_bytes(const_index);
        let const_index = if const_index == 0 {
            None
        } else {
            Some(const_index)
        };
        global_vars.push(GlobalVarInfo {
            name,
            const_index,
        });
    }

    // 读取函数表
    let mut func_count = [0u8; 2];
    file.read_exact(&mut func_count)?;
    let func_count = u16::from_le_bytes(func_count);
    let mut functions = Vec::new();
    for _ in 0..func_count {
        let mut name_len = [0u8; 2];
        file.read_exact(&mut name_len)?;
        let name_len = u16::from_le_bytes(name_len);
        let mut name = vec![0u8; name_len as usize];
        file.read_exact(&mut name)?;
        let name = String::from_utf8(name).map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8 string")
        })?;
        let mut param_count = [0u8; 1];
        file.read_exact(&mut param_count)?;
        let mut local_count = [0u8; 1];
        file.read_exact(&mut local_count)?;
        let mut bytecode_len = [0u8; 2];
        file.read_exact(&mut bytecode_len)?;
        let bytecode_len = u16::from_le_bytes(bytecode_len);
        let mut bytecode = vec![0u8; bytecode_len as usize];
        file.read_exact(&mut bytecode)?;

        functions.push(FunctionInfo {
            name,
            param_count: param_count[0],
            local_count: local_count[0],
            bytecode,
        });
    }

    Ok(CompileResult {
        constants,
        global_vars,
        functions,
        entrypoint,
    })
}