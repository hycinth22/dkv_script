// FFI 接口部分
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};

use crate::{CompileResult, VM};

// 错误类型
type ResultCode = i32;
const SUCCESS: ResultCode = 0;
const ERROR: ResultCode = 1;

// C 兼容的 DKV 命令处理函数指针类型
type DkvCommandHandlerFn = unsafe extern "C" fn(command: *const c_char, user_data: *mut c_void) -> *mut c_char;

// C 兼容的结构体
#[repr(C)]
pub struct DkvScriptVM {
    vm: VM,
    handler: Option<DkvCommandHandlerFn>,
    user_data: *mut c_void,
}

#[repr(C)]
pub struct DkvScriptCompileResult {
    result: CompileResult,
}

// 暴露给 C 的函数
#[no_mangle]
pub extern "C" fn dkv_script_compile(source: *const c_char, result: *mut *mut DkvScriptCompileResult) -> ResultCode {
    unsafe {
        if source.is_null() || result.is_null() {
            return ERROR;
        }
        
        let source_str = match CStr::from_ptr(source).to_str() {
            Ok(s) => s,
            Err(_) => return ERROR,
        };
        
        match crate::do_compile(source_str) {
            Ok(compile_result) => {
                let c_result = Box::new(DkvScriptCompileResult {
                    result: compile_result,
                });
                *result = Box::into_raw(c_result);
                SUCCESS
            },
            Err(_) => ERROR,
        }
    }
}

#[no_mangle]
pub extern "C" fn dkv_script_create_vm(compile_result: *mut DkvScriptCompileResult, vm: *mut *mut DkvScriptVM) -> ResultCode {
    unsafe {
        if compile_result.is_null() || vm.is_null() {
            return ERROR;
        }
        
        let c_result = &mut *compile_result;
        let vm_instance = VM::new(c_result.result.clone());
        
        // 创建 C 兼容的 VM 实例
        let c_vm = Box::new(DkvScriptVM {
            vm: vm_instance,
            handler: None,
            user_data: std::ptr::null_mut(),
        });
        
        *vm = Box::into_raw(c_vm);
        SUCCESS
    }
}

#[no_mangle]
pub extern "C" fn dkv_script_set_dkv_command_handler(
    vm: *mut DkvScriptVM,
    handler: Option<DkvCommandHandlerFn>,
    user_data: *mut c_void
) -> ResultCode {
    unsafe {
        if vm.is_null() {
            return ERROR;
        }
        
        let c_vm = &mut *vm;
        c_vm.handler = handler;
        c_vm.user_data = user_data;
        
        // 将 C 风格的处理函数转换为 Rust 闭包
        let handler_closure = if let Some(c_handler) = handler {
            // 创建一个捕获 c_handler 和 user_data 的闭包
            Some(move |command: &str| -> Result<String, String> {
                // 将 Rust 字符串转换为 C 字符串
                let c_command = CString::new(command).map_err(|e| e.to_string())?;
                
                // 调用 C 处理函数
                let c_result = c_handler(c_command.as_ptr(), user_data);
                if c_result.is_null() {
                    return Err("C handler returned null pointer".to_string());
                }
                
                // 将 C 字符串转换为 Rust 字符串
                let result_str = CStr::from_ptr(c_result)
                    .to_str()
                    .map_err(|e| e.to_string())?
                    .to_string();
                
                // 释放 C 字符串（假设 C 处理函数返回的字符串需要被释放）
                libc::free(c_result as *mut libc::c_void);
                
                Ok(result_str)
            })
        } else {
            None
        };
        
        // 设置处理函数
        c_vm.vm.set_dkv_command_handler(handler_closure);
        SUCCESS
    }
}

#[no_mangle]
pub extern "C" fn dkv_script_run_vm(vm: *mut DkvScriptVM) -> ResultCode {
    unsafe {
        if vm.is_null() {
            return ERROR;
        }
        
        let c_vm = &mut *vm;
        c_vm.vm.run();
        SUCCESS
    }
}

#[no_mangle]
pub extern "C" fn dkv_script_free_compile_result(result: *mut DkvScriptCompileResult) {
    unsafe {
        if !result.is_null() {
            drop(Box::from_raw(result));
        }
    }
}

#[no_mangle]
pub extern "C" fn dkv_script_free_vm(vm: *mut DkvScriptVM) {
    unsafe {
        if !vm.is_null() {
            drop(Box::from_raw(vm));
        }
    }
}
