// 抽象语法树（AST）节点类型
#[derive(Debug, Clone)]
pub enum ASTNode {
    // 程序
    Program(Vec<Box<ASTNode>>),
    Block(Vec<Box<ASTNode>>),
    FunctionDef(String, Vec<(String, String)>, Box<ASTNode>),
    // 语句
    VariableDecl(String, String, Option<Box<ASTNode>>),
    Assignment(String, Box<ASTNode>),
    IfStatement(Box<ASTNode>, Box<ASTNode>, Option<Box<ASTNode>>),
    ForLoop(Option<Box<ASTNode>>, Option<Box<ASTNode>>, Option<Box<ASTNode>>, Box<ASTNode>),
    WhileLoop(Box<ASTNode>, Box<ASTNode>),
    FunctionCall(String, Vec<Box<ASTNode>>),
    Return(Option<Box<ASTNode>>),
    // 表达式
    BinaryExpr(Box<ASTNode>, String, Box<ASTNode>),
    UnaryExpr(String, Box<ASTNode>),
    // 自增自减表达式
    Increment(String),
    Decrement(String),
    // 字面量
    IntLiteral(i32),
    FloatLiteral(f32),
    BoolLiteral(bool),
    StringLiteral(String),
    Identifier(String),
}