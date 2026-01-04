use crate::{compiler::{CompileResult, Constant, FunctionInfo, OpCode}, SYSCALL};

// 运行时值类型
#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    Null,
}

// 比较函数
#[inline]
fn eq_values(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => x == y,
        (Value::Float(x), Value::Float(y)) => x == y,
        (Value::Bool(x), Value::Bool(y)) => x == y,
        (Value::String(x), Value::String(y)) => x == y,
        (Value::Null, Value::Null) => true,
        _ => false,
    }
}

#[inline]
fn ne_values(a: &Value, b: &Value) -> bool {
    !eq_values(a, b)
}

#[inline]
fn lt_values(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => x < y,
        (Value::Float(x), Value::Float(y)) => x < y,
        _ => panic!("Invalid types {:?} and {:?} for less than comparison", a, b),
    }
}

#[inline]
fn le_values(a: &Value, b: &Value) -> bool {
    lt_values(a, b) || eq_values(a, b)
}

#[inline]
fn gt_values(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => x > y,
        (Value::Float(x), Value::Float(y)) => x > y,
        _ => panic!("Invalid types for greater than comparison"),
    }
}

#[inline]
fn ge_values(a: &Value, b: &Value) -> bool {
    gt_values(a, b) || eq_values(a, b)
}

// 算术运算函数
#[inline]
fn add_values(a: &Value, b: &Value) -> Value {
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => Value::Int(x + y),
        (Value::Float(x), Value::Float(y)) => Value::Float(x + y),
        (Value::String(x), Value::String(y)) => Value::String(format!("{}{}", x, y)),
        _ => panic!("Invalid types for addition"),
    }
}

#[inline]
fn sub_values(a: &Value, b: &Value) -> Value {
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => Value::Int(x - y),
        (Value::Float(x), Value::Float(y)) => Value::Float(x - y),
        _ => panic!("Invalid types for subtraction"),
    }
}

#[inline]
fn inc_values(a: &Value) -> Value {
    match a {
        Value::Int(x) => Value::Int(x + 1),
        Value::Float(x) => Value::Float(x + 1.0),
        _ => panic!("Invalid type for increment"),
    }
}

#[inline]
fn not_values(a: &Value) -> Value {
    match a {
        Value::Bool(x) => Value::Bool(!x),
        _ => panic!("Invalid type for not operation"),
    }
}

#[inline]
fn neg_values(a: &Value) -> Value {
    match a {
        Value::Int(x) => Value::Int(-x),
        Value::Float(x) => Value::Float(-x),
        _ => panic!("Invalid type for negation"),
    }
}

#[inline]
fn dec_values(a: &Value) -> Value {
    match a {
        Value::Int(x) => Value::Int(x - 1),
        Value::Float(x) => Value::Float(x - 1.0),
        _ => panic!("Invalid type for decrement"),
    }
}

#[inline]
fn mul_values(a: &Value, b: &Value) -> Value {
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => Value::Int(x * y),
        (Value::Float(x), Value::Float(y)) => Value::Float(x * y),
        _ => panic!("Invalid types for multiplication"),
    }
}

#[inline]
fn div_values(a: &Value, b: &Value) -> Value {
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => {
            if *y == 0 {
                panic!("Division by zero");
            }
            Value::Int(x / y)
        },
        (Value::Float(x), Value::Float(y)) => {
            if *y == 0.0 {
                panic!("Division by zero");
            }
            Value::Float(x / y)
        },
        _ => panic!("Invalid types for division"),
    }
}

pub struct VM {
    constants: Vec<Constant>,
    global_vars: Vec<Value>,
    functions: Vec<FunctionInfo>,
    stack: Vec<Value>,
    entrypoint: u16,

    pc: usize, // 程序计数器
    fp: usize, // 栈帧指针
    
    // DKV command handler
    dkv_command_handler: Option<Box<dyn FnMut(&str) -> Result<String, String>>>,
}

impl VM {
    pub fn new(compile_result: CompileResult) -> Self {
        let mut vm = VM {
            constants: compile_result.constants,
            global_vars: Vec::new(),
            functions: compile_result.functions,
            stack: Vec::new(),
            pc: 0,
            fp: 0,
            entrypoint: compile_result.entrypoint,
            dkv_command_handler: None,
        };

        // 初始化全局变量
        for global_var in compile_result.global_vars {
            if let Some(const_index) = global_var.const_index {
                let value = vm.get_constant(const_index);
                vm.global_vars.push(value);
            } else {
                vm.global_vars.push(Value::Null);
            }
        }
        vm
    }
    
    /// 设置DKV命令处理函数
    pub fn set_dkv_command_handler<F>(&mut self, handler: Option<F>) 
    where 
        F: FnMut(&str) -> Result<String, String> + 'static,
    {
        self.dkv_command_handler = handler.map(|h| Box::new(h) as Box<dyn FnMut(&str) -> Result<String, String>>);
    }

    pub fn run(&mut self) {
        // 调用主函数
        if self.entrypoint < self.functions.len() as u16 {
            self.call_function(self.entrypoint);
        } else {
            panic!("Entry point function not found");
        }
    }

    fn get_constant(&self, index: u16) -> Value {
        if index < self.constants.len() as u16 {
            match &self.constants[index as usize] {
                Constant::Nil => Value::Null,
                Constant::Int(value) => Value::Int(*value),
                Constant::Float(value) => Value::Float(*value),
                Constant::Bool(value) => Value::Bool(*value),
                Constant::String(value) => Value::String(value.clone()),
            }
        } else {
            panic!("Constant index out of bounds: {}", index);
        }
    }

    fn call_function(&mut self, func_index: u16) {
        if func_index >= self.functions.len() as u16 {
            panic!("Function index out of bounds: {}", func_index);
        }

        let func = &self.functions[func_index as usize];
        println!("Calling function: {}", func.name);
        let old_fp: usize = self.fp;
        let return_addr = self.pc;
        println!("Call function {} old_fp:{} return addr: {}", func.name, old_fp, return_addr);

        // 检查参数数量（栈上的参数数量必须大于等于参数数量）
        // stack为空时代表入口点函数，此时不必检查
        if !self.stack.is_empty() && func.param_count as usize > self.stack.len() - old_fp - 2 {
            panic!("Incorrect number of arguments for function {}: expected {}, got {}", func.name, func.param_count, self.stack.len() - old_fp - 2);
        }
        let args = (0..func.param_count).into_iter().map(|_i| {
            self.stack.pop().unwrap()
        }).collect::<Vec<_>>();

        // 保存返回地址和旧的帧指针
        self.stack.push(Value::Int(return_addr as i32));
        self.stack.push(Value::Int(old_fp as i32));

        // 设置新的帧指针
        self.fp = self.stack.len() - 2;

        // 设置程序计数器到函数的字节码开始位置
        self.pc = 0;

        // 压入参数
        self.stack.extend(args.into_iter());
        // 准备局部变量
        for _ in 0..func.local_count-func.param_count {
            self.stack.push(Value::Null);
        }
    
        // 执行函数
        self.execute_function(func_index);
        println!("Call function return");
    }

    fn execute_function(&mut self, func_index: u16) {
        if func_index >= self.functions.len() as u16 {
            panic!("Function index out of bounds: {}", func_index);
        }

        let func = self.functions[func_index as usize].clone();
        self.execute_bytecode(&func.bytecode);
    }

    fn execute_bytecode(&mut self, bytecode: &[u8]) {
        while self.pc < bytecode.len() {
            println!("Stack: {:?}", self.stack);
            println!("PC: {}", self.pc);
            let opcode = OpCode::from_byte(bytecode[self.pc]);
            println!("Executing opcode: {:02x}", opcode);
            self.pc += 1;

            match opcode {
                OpCode::LoadConst => {
                    let const_index = self.read_u16(bytecode);
                    let value = self.get_constant(const_index);
                    self.stack.push(value);
                },
                OpCode::LoadGlobal => {
                    let var_index = self.read_u16(bytecode);
                    if var_index < self.global_vars.len() as u16 {
                        let value = self.global_vars[var_index as usize].clone();
                        self.stack.push(value);
                    } else {
                        panic!("Global variable index out of bounds: {}", var_index);
                    }
                },
                OpCode::StoreGlobal => {
                    let var_index = self.read_u16(bytecode);
                    if var_index < self.global_vars.len() as u16 {
                        if let Some(value) = self.stack.pop() {
                            self.global_vars[var_index as usize] = value;
                        } else {
                            panic!("Stack underflow");
                        }
                    } else {
                        panic!("Global variable index out of bounds: {}", var_index);
                    }
                },
                OpCode::LoadLocal => {
                    let local_index = self.read_u16(bytecode);
                    println!("Load local {} fp:{}", local_index, self.fp);
                    let stack_index = self.fp + 2 + local_index as usize;
                    if stack_index < self.stack.len() {
                        let value = self.stack[stack_index].clone();
                        self.stack.push(value);
                    } else {
                        panic!("Local variable index out of bounds: {}", local_index);
                    }
                },
                OpCode::StoreLocal => {
                    let local_index = self.read_u16(bytecode);
                    println!("Store local {} fp:{}", local_index, self.fp);
                    let stack_index = self.fp + 2 + local_index as usize;
                    if stack_index < self.stack.len() {
                        if let Some(value) = self.stack.pop() {
                            self.stack[stack_index] = value;
                        } else {
                            panic!("Stack underflow");
                        }
                    } else {
                        panic!("Local variable index out of bounds: {}", local_index);
                    }
                },
                OpCode::Add => self.binary_operation(add_values),
                OpCode::Sub => self.binary_operation(sub_values),
                OpCode::Mul => self.binary_operation(mul_values),
                OpCode::Div => self.binary_operation(div_values),
                OpCode::Not => self.unary_operation(not_values),
                OpCode::Inc => self.unary_operation(inc_values),
                OpCode::Dec => self.unary_operation(dec_values),
                OpCode::Neg => self.unary_operation(neg_values),
                OpCode::CmpEq => self.comparison_operation(eq_values),
                OpCode::CmpNe => self.comparison_operation(ne_values),
                OpCode::CmpLt => self.comparison_operation(lt_values),
                OpCode::CmpLe => self.comparison_operation(le_values),
                OpCode::CmpGt => self.comparison_operation(gt_values),
                OpCode::CmpGe => self.comparison_operation(ge_values),
                OpCode::Jmp => {
                    let offset = self.read_i16(bytecode) as isize;
                    self.pc = (self.pc as isize -1 + offset) as usize;
                    continue;
                },
                OpCode::Jz => {
                    let offset = self.read_i16(bytecode) as isize;
                    if let Some(Value::Bool(x)) = self.stack.pop() {
                        if x {
                            // do nothing
                        } else {
                            self.pc = (self.pc as isize -1 + offset) as usize;
                            continue;
                        }
                    } else {
                        panic!("Jz operator applied to non-bool value");
                    }
                },
                OpCode::Call => {
                    let func_index = self.read_u16(bytecode);
                    self.pc -= 1;
                    self.call_function(func_index);
                    self.pc += 1;
                },
                OpCode::Ret => {
                    // 弹出返回值
                    let Some(rv) = self.stack.pop() else {
                        panic!("ret: return value missing in stack")
                    };
                    // 保存程序计数器和帧指针
                    let Some(Value::Int(return_addr)) = self.stack.get(self.fp).cloned() else {
                        panic!("invalid return_addr. fp: {}", self.fp);
                    };
                    let Some(Value::Int(old_fp)) = self.stack.get(self.fp + 1).cloned() else {
                        panic!("invalid old_fp. fp: {}", self.fp);
                    };
                    // 清理栈帧
                    self.stack.truncate(self.fp);
                    // 压入返回值
                    self.stack.push(rv);
                    // 恢复程序计数器和帧指针
                    self.pc = return_addr as usize;
                    self.fp = old_fp as usize;
                    return;
                },
                OpCode::Syscall => {
                    let syscall_id = self.read_u16(bytecode);
                    match SYSCALL::from(syscall_id) {
                        SYSCALL::PRINT => {
                            if let Some(value) = self.stack.pop() {
                                self.print_value(&value);
                            } else {
                                panic!("Stack underflow in syscall PRINT");
                            }
                        },
                        SYSCALL::DKVCOMMAND => {
                            if let Some(Value::String(command)) = self.stack.pop() {
                                let result = if let Some(ref mut handler) = self.dkv_command_handler {
                                    match handler(&command) {
                                        Ok(output) => Value::String(output),
                                        Err(err) => Value::String(format!("Error: {}", err)),
                                    }
                                } else {
                                    Value::String(format!("Error: No DKV command handler set"))
                                };
                                self.stack.push(result);
                            } else {
                                panic!("Stack underflow or invalid value type in syscall DKVCOMMAND");
                            }
                        },
                        _ => panic!("Unknown syscall ID: 0x{:02x}", syscall_id),
                    }
                },
                OpCode::Exit => {
                    // 退出程序执行
                    return;
                },
            }
            self.pc += 8;
        }
    }

    fn read_u16(&mut self, bytecode: &[u8]) -> u16 {
        let value = u16::from_le_bytes([bytecode[self.pc], bytecode[self.pc + 1]]);
        value
    }

    fn read_i16(&mut self, bytecode: &[u8]) -> i16 {
        let value = i16::from_le_bytes([bytecode[self.pc], bytecode[self.pc + 1]]);
        value
    }

    fn read_u64(&mut self, bytecode: &[u8]) -> u64 {
        let value = u64::from_le_bytes([bytecode[self.pc], bytecode[self.pc + 1], bytecode[self.pc + 2], bytecode[self.pc + 3], bytecode[self.pc + 4], bytecode[self.pc + 5], bytecode[self.pc + 6], bytecode[self.pc + 7]]);
        value
    }

    fn read_i64(&mut self, bytecode: &[u8]) -> i64 {
        let value = i64::from_le_bytes([bytecode[self.pc], bytecode[self.pc + 1], bytecode[self.pc + 2], bytecode[self.pc + 3], bytecode[self.pc + 4], bytecode[self.pc + 5], bytecode[self.pc + 6], bytecode[self.pc + 7]]);
        value
    }

    fn unary_operation(&mut self, op: fn(&Value) -> Value) {
        if let Some(a) = self.stack.pop() {
            let result = op(&a);
            self.stack.push(result);
        } else {
            panic!("Stack underflow in unary operation");
        }
    }

    fn binary_operation(&mut self, op: fn(&Value, &Value) -> Value) {
        if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
            let result = op(&a, &b);
            self.stack.push(result);
        } else {
            panic!("Stack underflow in binary operation");
        }
    }

    fn comparison_operation(&mut self, op: fn(&Value, &Value) -> bool) {
        if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
            let result = op(&a, &b);
            self.stack.push(Value::Bool(result));
        } else {
            panic!("Stack underflow in comparison operation");
        }
    }

    fn print_value(&self, value: &Value) {
        match value {
            Value::Int(x) => println!("{}", x),
            Value::Float(x) => println!("{}", x),
            Value::Bool(x) => println!("{}", x),
            Value::String(x) => println!("{}", x),
            Value::Null => println!("null"),
        }
    }
}

use num_traits::FromPrimitive;
impl OpCode {
    fn from_byte(byte: u8) -> OpCode {
        match OpCode::from_u8(byte) {
            Some(op) => op,
            _ => panic!("Unknown opcode: {:#02x}", byte),
        }
    }
}

impl core::convert::Into<u8> for OpCode {
    fn into(self) -> u8 {
        self as u8
    }
}

impl std::fmt::Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}(0x{:02})", self, *self as u8)
    }
}

impl std::fmt::LowerHex for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}(0x{:02x})", self, *self as u8)
    }
}
