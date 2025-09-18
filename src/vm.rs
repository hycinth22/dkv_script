use crate::compiler::{CompileResult, Constant, FunctionInfo, OpCode};

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

fn ne_values(a: &Value, b: &Value) -> bool {
    !eq_values(a, b)
}

fn lt_values(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => x < y,
        (Value::Float(x), Value::Float(y)) => x < y,
        _ => panic!("Invalid types for less than comparison"),
    }
}

fn le_values(a: &Value, b: &Value) -> bool {
    lt_values(a, b) || eq_values(a, b)
}

fn gt_values(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => x > y,
        (Value::Float(x), Value::Float(y)) => x > y,
        _ => panic!("Invalid types for greater than comparison"),
    }
}

fn ge_values(a: &Value, b: &Value) -> bool {
    gt_values(a, b) || eq_values(a, b)
}

// 算术运算函数
fn add_values(a: &Value, b: &Value) -> Value {
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => Value::Int(x + y),
        (Value::Float(x), Value::Float(y)) => Value::Float(x + y),
        (Value::String(x), Value::String(y)) => Value::String(format!("{}{}", x, y)),
        _ => panic!("Invalid types for addition"),
    }
}

fn sub_values(a: &Value, b: &Value) -> Value {
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => Value::Int(x - y),
        (Value::Float(x), Value::Float(y)) => Value::Float(x - y),
        _ => panic!("Invalid types for subtraction"),
    }
}

fn inc_values(a: &Value) -> Value {
    match a {
        Value::Int(x) => Value::Int(x + 1),
        Value::Float(x) => Value::Float(x + 1.0),
        _ => panic!("Invalid type for increment"),
    }
}

fn dec_values(a: &Value) -> Value {
    match a {
        Value::Int(x) => Value::Int(x - 1),
        Value::Float(x) => Value::Float(x - 1.0),
        _ => panic!("Invalid type for decrement"),
    }
}

fn mul_values(a: &Value, b: &Value) -> Value {
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => Value::Int(x * y),
        (Value::Float(x), Value::Float(y)) => Value::Float(x * y),
        _ => panic!("Invalid types for multiplication"),
    }
}

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
    pc: usize, // 程序计数器
    fp: usize, // 栈帧指针
    entrypoint: u16,
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

        // 复制函数信息以避免可变借用冲突
        let func = self.functions[func_index as usize].clone();
        let old_fp = self.fp;
        let return_addr = self.pc;

        // 保存返回地址和旧的帧指针
        self.stack.push(Value::Int(return_addr as i32));
        self.stack.push(Value::Int(old_fp as i32));

        // 设置新的帧指针
        self.fp = self.stack.len() - 2;

        // 设置程序计数器到函数的字节码开始位置
        self.pc = 0;

        // 执行函数
        self.execute_bytecode(&func.bytecode);

        // 函数返回后恢复状态
        self.fp = old_fp;
    }

    fn execute_bytecode(&mut self, bytecode: &[u8]) {
        while self.pc < bytecode.len() {
            let opcode = OpCode::from_byte(bytecode[self.pc]);
            self.pc += 2;

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
                OpCode::Not => {
                    if let Value::Bool(b) = self.stack.pop().unwrap() {
                        self.stack.push(Value::Bool(!b));
                    } else {
                        panic!("Not operator applied to non-bool value");
                    }
                },
                OpCode::Neg => {
                    if let Value::Int(i) = self.stack.pop().unwrap() {
                        self.stack.push(Value::Int(-i));
                    } else {
                        panic!("Neg operator applied to non-int value");
                    }
                }
                OpCode::Add => self.binary_operation(add_values),
                OpCode::Sub => self.binary_operation(sub_values),
                OpCode::Mul => self.binary_operation(mul_values),
                OpCode::Div => self.binary_operation(div_values),
                OpCode::Inc => {
                    let var_index = self.read_u16(bytecode);
                    if var_index < self.global_vars.len() as u16 {
                        let current_value = self.global_vars[var_index as usize].clone();
                        self.global_vars[var_index as usize] = inc_values(&current_value);
                    } else {
                        panic!("Global variable index out of bounds for increment: {}", var_index);
                    }
                },
                OpCode::Dec => {
                    let var_index = self.read_u16(bytecode);
                    if var_index < self.global_vars.len() as u16 {
                        let current_value = self.global_vars[var_index as usize].clone();
                        self.global_vars[var_index as usize] = dec_values(&current_value);
                    } else {
                        panic!("Global variable index out of bounds for decrement: {}", var_index);
                    }
                },
                OpCode::CmpEq => self.comparison_operation(eq_values),
                OpCode::CmpNe => self.comparison_operation(ne_values),
                OpCode::CmpLt => self.comparison_operation(lt_values),
                OpCode::CmpLe => self.comparison_operation(le_values),
                OpCode::CmpGt => self.comparison_operation(gt_values),
                OpCode::CmpGe => self.comparison_operation(ge_values),
                OpCode::Jmp => {
                    let offset = self.read_i16(bytecode) as isize;
                    self.pc = (self.pc as isize + offset) as usize;
                },
                OpCode::Jz => {
                    let offset = self.read_i16(bytecode) as isize;
                    if let Some(Value::Bool(false)) = self.stack.last() {
                        self.pc = (self.pc as isize + offset) as usize;
                    }
                },
                OpCode::Call => {
                    let func_index = self.read_u16(bytecode);
                    self.call_function(func_index);
                },
                OpCode::Ret => {
                    // 恢复程序计数器和帧指针
                    if let Some(Value::Int(fp)) = self.stack.get(self.fp + 1) {
                        self.fp = *fp as usize;
                    }
                    if let Some(Value::Int(return_addr)) = self.stack.get(self.fp) {
                        self.pc = *return_addr as usize;
                    }
                    // 弹出返回值（如果有）
                    break;
                },
                OpCode::Syscall => {
                    // 系统调用（简化版）
                },
                OpCode::Exit => {
                    // 退出程序执行
                    break;
                },
            }
        }
    }

    fn read_u16(&mut self, bytecode: &[u8]) -> u16 {
        let value = u16::from_le_bytes([bytecode[self.pc], bytecode[self.pc + 1]]);
        self.pc += 8; // 跳过整个 8 字节操作数
        value
    }

    fn read_i16(&mut self, bytecode: &[u8]) -> i16 {
        let value = i16::from_le_bytes([bytecode[self.pc], bytecode[self.pc + 1]]);
        self.pc += 8; // 跳过整个 8 字节操作数
        value
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

    fn add_values(&self, a: &Value, b: &Value) -> Value {
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => Value::Int(x + y),
            (Value::Float(x), Value::Float(y)) => Value::Float(x + y),
            (Value::String(x), Value::String(y)) => Value::String(format!("{}{}", x, y)),
            _ => panic!("Invalid types for addition"),
        }
    }

    fn sub_values(&self, a: &Value, b: &Value) -> Value {
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => Value::Int(x - y),
            (Value::Float(x), Value::Float(y)) => Value::Float(x - y),
            _ => panic!("Invalid types for subtraction"),
        }
    }

    fn mul_values(&self, a: &Value, b: &Value) -> Value {
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => Value::Int(x * y),
            (Value::Float(x), Value::Float(y)) => Value::Float(x * y),
            _ => panic!("Invalid types for multiplication"),
        }
    }

    fn div_values(&self, a: &Value, b: &Value) -> Value {
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

impl OpCode {
    fn from_byte(byte: u8) -> OpCode {
        match byte {
            0x01 => OpCode::LoadConst,
            0x02 => OpCode::LoadGlobal,
            0x03 => OpCode::StoreGlobal,
            0x04 => OpCode::LoadLocal,
            0x05 => OpCode::StoreLocal,
            0x10 => OpCode::Add,
            0x11 => OpCode::Sub,
            0x12 => OpCode::Mul,
            0x13 => OpCode::Div,
            0x14 => OpCode::Inc,
            0x15 => OpCode::Dec,
            0x20 => OpCode::CmpEq,
            0x21 => OpCode::CmpNe,
            0x22 => OpCode::CmpLt,
            0x23 => OpCode::CmpGt,
            0x24 => OpCode::CmpLe,
            0x25 => OpCode::CmpGe,
            0x50 => OpCode::Jmp,
            0x51 => OpCode::Jz,
            0x60 => OpCode::Call,
            0x61 => OpCode::Ret,
            0xFE => OpCode::Syscall,
            0xFF => OpCode::Exit,
            _ => panic!("Unknown opcode: {:#02x}", byte),
        }
    }
}