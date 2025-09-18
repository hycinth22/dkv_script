use crate::ast::ASTNode;
use core::panic;
use std::collections::HashMap;

const OPLEN: usize = 9;
const OP_ARGOFF: usize = 1;

// 字节码指令
#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    LoadConst = 0x01,
    LoadGlobal = 0x02,
    StoreGlobal = 0x03,
    LoadLocal = 0x04,
    StoreLocal = 0x05,

    Not = 0x10,
    Inc = 0x11,
    Dec = 0x12,
    Neg = 0x13,

    Add = 0x1A,
    Sub = 0x1B,
    Mul = 0x1C,
    Div = 0x1D,

    CmpEq = 0x20,
    CmpNe = 0x21,
    CmpLt = 0x22,
    CmpGt = 0x23,
    CmpLe = 0x24,
    CmpGe = 0x25,
    Jmp = 0x50,
    Jz = 0x51,
    Call = 0x60,
    Ret = 0x61,

    Syscall = 0xFE,
    Exit = 0xFF,
}
// 常量类型
#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    Nil,
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
}

// 函数信息
#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub name: String,
    pub param_count: u8,
    pub local_count: u8,
    pub bytecode: Vec<u8>,
}

// 全局变量信息
#[derive(Debug, Clone)]
pub struct GlobalVarInfo {
    pub name: String,
    pub const_index: Option<u16>,
}

// 局部变量信息
#[derive(Debug, Clone)]
pub struct LocalVarInfo {
    pub name: String,
    pub const_index: Option<u16>,
}

// 编译结果
#[derive(Debug, Clone)]
pub struct CompileResult {
    pub constants: Vec<Constant>,
    pub global_vars: Vec<GlobalVarInfo>,
    pub functions: Vec<FunctionInfo>,
    pub entrypoint: u16,
}

pub struct Compiler {
    constants: Vec<Constant>,
    global_vars: Vec<GlobalVarInfo>,
    functions: Vec<FunctionInfo>,
    entrypoint_function_index: u16,
    main_function_index: u16,

    // 符号表
    global_var_map: HashMap<String, usize>,
    function_map: HashMap<String, u16>,
    syscall_map: HashMap<String, u8>, // syscall函数列表，存储函数名和对应的系统调用编号

    // 用于跟踪当前函数
    current_local_vars: Vec<LocalVarInfo>,
    current_local_vars_map: HashMap<String, usize>,
    in_global_scope: bool,
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Compiler {
    pub fn new() -> Self {
        let mut syscall_map = HashMap::new();
        syscall_map.insert("print".to_string(), 1);
        syscall_map.insert("input".to_string(), 2);
        syscall_map.insert("time".to_string(), 3);
        syscall_map.insert("exit".to_string(), 4);
        syscall_map.insert("sleep".to_string(), 5);
        
        Compiler {
            constants: Vec::new(),
            global_vars: Vec::new(),
            functions: Vec::new(),
            entrypoint_function_index: 0,
            main_function_index: 0,

            global_var_map: HashMap::new(),
            function_map: HashMap::new(),
            syscall_map,
            current_local_vars: Vec::new(),
            current_local_vars_map: HashMap::new(),
            in_global_scope: true,
        }
    }

    pub fn compile(mut self, ast: &ASTNode) -> CompileResult {
        self.add_constant(Constant::Nil);
        let mut entrypoint_bytecode = Vec::new();
        self.visit_ast_with_bytecode(ast, &mut entrypoint_bytecode);
        self.emit_opcode_with_arg(&mut entrypoint_bytecode, OpCode::Call, self.main_function_index as u64);
        self.emit_opcode(&mut entrypoint_bytecode, OpCode::Exit);
        self.functions.push(FunctionInfo {
            name: "_entrypoint".to_string(),
            param_count: 0,
            local_count: 0,
            bytecode: entrypoint_bytecode,
        });
        let entrypoint_function_index = self.functions.len() as u16 - 1;
        CompileResult {
            constants: self.constants,
            global_vars: self.global_vars,
            functions: self.functions,
            entrypoint: entrypoint_function_index,
        }
    }
    
    fn visit_ast_with_bytecode(&mut self, ast: &ASTNode, bytecode: &mut Vec<u8>) {
        if let ASTNode::Program(statements) = ast {
                for stmt in statements {
                    self.visit_statement(stmt, bytecode);
                }
        } else {
            panic!("ROOT node is not ASTNode::Program");
        }
    }

    fn visit_block(&mut self, block: &ASTNode, bytecode: &mut Vec<u8>) {
        if let ASTNode::Block(statements) = block {
            for stmt in statements {
                self.visit_statement(stmt, bytecode);
            }
        }
    }

    fn visit_assignment(&mut self, name: &str, expr: &ASTNode, bytecode: &mut Vec<u8>) {
        self.visit_expression(expr, bytecode);

        if let Some(var_index_value) = self.lookup_local(name) {
            self.emit_store_local(bytecode, var_index_value);
        } else if let Some(var_index_value) = self.lookup_global(name) {
            self.emit_store_global(bytecode, var_index_value);
        } else {
            panic!("Unknown variable: {}", name);
        }
    }

    fn visit_increment(&mut self, name: &str, bytecode: &mut Vec<u8>) {
       if let Some(var_index_value) = self.lookup_local(name) {
            self.emit_load_local(bytecode, var_index_value);
            self.emit_opcode(bytecode, OpCode::Inc);
            self.emit_store_local(bytecode, var_index_value);
        } else if let Some(var_index_value) = self.lookup_global(name) {
            self.emit_load_global(bytecode, var_index_value);
            self.emit_opcode(bytecode, OpCode::Inc);
            self.emit_store_global(bytecode, var_index_value);
        } else {
            panic!("Unknown variable: {}", name);
        }
    }

    fn visit_decrement(&mut self, name: &str, bytecode: &mut Vec<u8>) {
        if let Some(var_index_value) = self.lookup_local(name) {
            self.emit_load_local(bytecode, var_index_value);
            self.emit_opcode(bytecode, OpCode::Dec);
            self.emit_store_local(bytecode, var_index_value);
        } else if let Some(var_index_value) = self.lookup_global(name) {
            self.emit_load_global(bytecode, var_index_value);
            self.emit_opcode(bytecode, OpCode::Dec);
            self.emit_store_global(bytecode, var_index_value);
        } else {
            panic!("Unknown variable: {}", name);
        }
    }

    fn visit_statement(&mut self, stmt: &ASTNode, bytecode: &mut Vec<u8>) {
        match stmt {
            ASTNode::Block(statements) => {
                for stmt in statements {
                    self.visit_statement(stmt, bytecode);
                }
            },
            ASTNode::VariableDecl(name, _type, initializer) => {
                if let Some(expr) = initializer {
                    // 根据表达式生成初始化字节码
                    self.visit_expression(expr, bytecode);
                } else {
                    // 默认值
                    match _type.as_str() {
                        "int" => self.add_constant(Constant::Int(0)),
                        "float" => self.add_constant(Constant::Float(0.0)),
                        "bool" => self.add_constant(Constant::Bool(false)),
                        "string" => self.add_constant(Constant::String("".to_string())),
                        _ => panic!("Unknown type: {}", _type),
                    };
                };

                if self.in_global_scope {
                    self.global_vars.push(GlobalVarInfo {
                        name: name.clone(),
                        const_index: None,
                    });
                    let global_index = (self.global_vars.len() - 1) as u16;
                    self.global_var_map.insert(name.clone(), global_index as usize);
                    // 为全局变量生成初始化字节码
                    self.emit_store_global(bytecode, global_index);
                } else {
                    self.current_local_vars.push(LocalVarInfo {
                        name: name.clone(),
                        const_index: None,
                    });
                    let local_index = self.current_local_vars.len() as u8;
                    self.current_local_vars_map.insert(name.clone(), local_index as usize);
                    // 为局部变量生成初始化字节码
                    self.emit_store_local(bytecode, local_index);
                }
            },
            ASTNode::Assignment(name, expr) => self.visit_assignment(name, expr, bytecode),
            ASTNode::Increment(var_name) => self.visit_increment(var_name, bytecode),
            ASTNode::Decrement(var_name) => self.visit_decrement(var_name, bytecode),
            ASTNode::IfStatement(condition, then_branch, else_branch) => {
                // 生成求值字节码
                self.visit_expression(condition, bytecode);

                // 为 JZ 预留空间。该JZ负责条件为false则跳转到else分支或if结束
                let jz_pos = bytecode.len();
                self.emit_opcode_with_arg(bytecode, OpCode::Jz, 0);

                self.visit_block(then_branch, bytecode);

                // 为 JMP 预留空间。该JMP负责then_branch结束后跳转到if结束
                let jmp_pos = bytecode.len();
                self.emit_opcode_with_arg(bytecode, OpCode::Jmp, 0);

                // 填充 JZ 的偏移量
                let jz_offset = bytecode.len() - jz_pos - OPLEN;
                self.set_arg_at(bytecode, jz_pos, jz_offset as u64);
                // 写入else分支
                if let Some(else_block) = else_branch {
                    self.visit_block(else_block, bytecode);
                }

                // 填充 JMP 的偏移量
                let jmp_offset = bytecode.len() - jmp_pos - OPLEN;
                self.set_arg_at(bytecode, jmp_pos, jmp_offset as u64);
            },
            ASTNode::ForLoop(init, condition, update, body) => {
                // 初始化循环变量
                if let Some(init) = init {
                    if let ASTNode::Assignment(name, expr) = &**init {
                        self.visit_assignment(name, expr, bytecode);
                    } else {
                        panic!("For loop init must be an assignment");
                    }
                }
                let loop_start = bytecode.len();
                // 生成求值字节码
                if let Some(condition) = condition {
                    self.visit_expression(condition, bytecode);
                } else {
                    let const_idx = self.add_constant(Constant::Bool(true));
                    self.emit_load_const(bytecode, const_idx);
                }
                // 为 JZ 预留空间
                let jz_pos = bytecode.len();
                self.emit_opcode_with_arg(bytecode, OpCode::Jz, 0);

                self.visit_block(body, bytecode);

                if let Some(update) = update {
                    match &**update {
                        ASTNode::Assignment(name, expr) => self.visit_assignment(name, expr, bytecode),
                        ASTNode::Increment(var_name) => {
                            self.visit_increment(var_name, bytecode);
                        },
                        ASTNode::Decrement(var_name) => {
                            self.visit_decrement(var_name, bytecode);
                        },
                        _ => panic!("For loop update must be an assignment"),
                    }
                }

                // 跳回循环开始
                let jmp_offset = loop_start as i64 - bytecode.len() as i64 - OPLEN as i64;
                self.emit_opcode_with_arg(bytecode, OpCode::Jmp, jmp_offset as u64);

                // 填充 JZ 的偏移量
                let jz_offset = bytecode.len() - jz_pos - OPLEN;
                self.set_arg_at(bytecode, jz_pos, jz_offset as u64);
            },
            ASTNode::FunctionDef(name, params, body) => {
                let func_index = self.functions.len() as u16;
                self.function_map.insert(name.clone(), func_index);
                if name == "main" {
                    self.entrypoint_function_index = func_index;
                }

                // 记录参数作为局部变量
                self.current_local_vars.reserve(params.len());
                self.current_local_vars_map.reserve(params.len());

                let param_count = params.len() as u8;
                for (i, (param_name, _)) in params.iter().enumerate() {
                    self.current_local_vars.push(LocalVarInfo {
                        name: param_name.clone(),
                        const_index: None,
                    });
                    self.current_local_vars_map.insert(param_name.clone(), i);
                }
                
                // 编译函数体到字节码，同时分析局部变量
                let mut bytecode = Vec::new();
                self.in_global_scope = false;
                self.visit_block(body, &mut bytecode);
                self.in_global_scope = true;

                let const_idx= self.add_constant(Constant::Nil);
                self.emit_load_const(&mut bytecode, const_idx);
                self.emit_opcode(&mut bytecode, OpCode::Ret);

                // 计算总局部变量数
                let local_count = self.current_local_vars.len() as u8;

                self.functions.push(FunctionInfo {
                    name: name.clone(),
                    param_count,
                    local_count,
                    bytecode,
                });
                
                // 清空当前函数的局部变量映射，准备处理下一个函数
                self.current_local_vars.clear();
                self.current_local_vars_map.clear();
            },
            ASTNode::FunctionCall(name, args) => {
                for arg in args {
                    self.visit_expression(arg, bytecode);
                }

                let syscall_num = self.syscall_map.get(name).copied();
                if let Some(syscall_num) = syscall_num {
                    // 是系统调用，生成Syscall指令
                    self.emit_opcode_with_arg(bytecode, OpCode::Syscall, syscall_num as u64);
                } else {
                    // 不是系统调用，继续使用Call指令
                    let func_index = if let Some(index) = self.function_map.get(name) {
                        *index
                    } else {
                        panic!("Unknown function: {}", name);
                    };
                    self.emit_opcode_with_arg(bytecode, OpCode::Call, func_index as u64);
                }
            },
            ASTNode::Return(expr_opt) => {
                if let Some(expr) = expr_opt {
                    self.visit_expression(expr, bytecode);
                } else {
                    // 返回空值
                    let const_idx= self.add_constant(Constant::Nil);
                    self.emit_load_const(bytecode, const_idx);
                }
                self.emit_opcode(bytecode, OpCode::Ret);
            },
            _ => {
                panic!("Unsupported statement type {:?}", *stmt);
            },
        }
    }

    fn visit_expression(&mut self, expr: &ASTNode, bytecode: &mut Vec<u8>) {
        match expr {
            ASTNode::IntLiteral(value) => {
                let const_idx = self.add_constant(Constant::Int(*value));
                self.emit_load_const(bytecode, const_idx);
            },
            ASTNode::FloatLiteral(value) => {
                let const_idx = self.add_constant(Constant::Float(*value));
                self.emit_load_const(bytecode, const_idx);
            }
            ASTNode::BoolLiteral(value) => {
                let const_idx = self.add_constant(Constant::Bool(*value));
                self.emit_load_const(bytecode, const_idx);
            }
            ASTNode::StringLiteral(value) => {
                let const_idx = self.add_constant(Constant::String(value.clone()));
                self.emit_load_const(bytecode, const_idx);
            }
            ASTNode::Identifier(name) => {
                if let Some(local_index) = self.lookup_local(name) {
                    // 局部变量
                    self.emit_load_local(bytecode, local_index);
                } else if let Some(global_index) = self.lookup_global(name) {
                   self.emit_load_global(bytecode, global_index);
                } else {
                    panic!("Unknown identifier: {}", name);
                }
            },
            ASTNode::BinaryExpr(left, op, right) => {
                // 从左到右求值
                self.visit_expression(left, bytecode);
                self.visit_expression(right, bytecode);

                // 将操作符映射到对应的 OpCode
                let opcode = match op.as_str() {
                    "+" => OpCode::Add,
                    "-" => OpCode::Sub,
                    "*" => OpCode::Mul,
                    "/" => OpCode::Div,
                    "==" => OpCode::CmpEq,
                    "!=" => OpCode::CmpNe,
                    "<" => OpCode::CmpLt,
                    ">" => OpCode::CmpGt,
                    "<=" => OpCode::CmpLe,
                    ">=" => OpCode::CmpGe,
                    _ => panic!("Unknown binary operator: {}", op),
                };
                
                // 执行运算
                self.emit_opcode(bytecode, opcode);
            },
            ASTNode::UnaryExpr(op, expr) => {
                self.visit_expression(expr, bytecode);

                // 根据操作符类型，生成相应的字节码
                match op.as_str() {
                    "-" => {
                        self.emit_opcode(bytecode, OpCode::Neg);
                    },
                    "!" => {
                        self.emit_opcode(bytecode, OpCode::Not);
                    },
                    _ => panic!("Unknown unary operator: {}", op),
                }
            },
            _ => panic!("Unexpected expression type {:?}", *expr),
        }
    }

    fn add_constant(&mut self, constant: Constant) -> u16 {
        // 检查常量是否已存在
        for (i, c) in self.constants.iter().enumerate() {
            if c == &constant {
                return i as u16;
            }
        }
        
        // 添加新常量
        self.constants.push(constant);
        (self.constants.len() - 1) as u16
    }

    fn emit_opcode(&mut self, bytecode: &mut Vec<u8>, opcode: OpCode) {
        self.emit_opcode_with_arg(bytecode, opcode, 0)
    }

    fn emit_opcode_with_arg(&mut self, bytecode: &mut Vec<u8>, opcode: OpCode, arg: u64) {
        println!("emit_opcode_with_arg: {:?} {}", opcode, arg);
        bytecode.push(opcode as u8);
        bytecode.extend_from_slice(&arg.to_le_bytes());
    }

    fn emit_load_const(&mut self, bytecode: &mut Vec<u8>, const_index: u16) {
        self.emit_opcode_with_arg(bytecode, OpCode::LoadConst, const_index as u64);
    }

    fn emit_load_local(&mut self, bytecode: &mut Vec<u8>, local_index: u8) {
        self.emit_opcode_with_arg(bytecode, OpCode::LoadLocal, local_index as u64);
    }

    fn emit_load_global(&mut self, bytecode: &mut Vec<u8>, global_index: u16) {
        self.emit_opcode_with_arg(bytecode, OpCode::LoadGlobal, global_index as u64);
    }

    fn emit_store_local(&mut self, bytecode: &mut Vec<u8>, local_index: u8) {
        self.emit_opcode_with_arg(bytecode, OpCode::StoreLocal, local_index as u64);
    }

    fn emit_store_global(&mut self, bytecode: &mut Vec<u8>, global_index: u16) {
        self.emit_opcode_with_arg(bytecode, OpCode::StoreGlobal, global_index as u64);
    }

    fn set_arg_at(&mut self, bytecode: &mut [u8], pc: usize, arg: u64) {
        assert!(pc + OPLEN <= bytecode.len(), "pc + OP_ARGOFF out of bounds {} {}", pc, bytecode.len());
        let arg_bytes = arg.to_le_bytes();
        for i in 0..OPLEN-OP_ARGOFF {
            bytecode[pc + OP_ARGOFF + i] = arg_bytes[i];
        }
    }

    fn lookup_local(&self, name: &str) -> Option<u8> {
        self.current_local_vars_map.get(name).map(|v| *v as u8)
    }

    fn lookup_global(&self, name: &str) -> Option<u16> {
        self.global_var_map.get(name).map(|v| *v as u16)
    }
}