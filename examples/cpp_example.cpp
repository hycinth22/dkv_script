#include "../include/dkv_script.hpp"
#include <iostream>
#include <fstream>

// 读取文件内容的辅助函数
std::string readFile(const std::string& filename) {
    std::ifstream file(filename);
    if (!file) {
        throw std::runtime_error("Failed to open file: " + filename);
    }
    
    std::string content((std::istreambuf_iterator<char>(file)),
                       std::istreambuf_iterator<char>());
    
    return content;
}

int main() {
    try {
        // 创建 DkvScript 对象
        dkv_script::DkvScript script;
        
        std::cout << "=== Testing DKV Script C++ Interface ===\n";
        
        // 示例 1: 执行一段简单的脚本代码
        std::cout << "\n1. Executing simple script code:\n";
        std::string simple_script = "print(\"Hello from DKV Script!\");";
        script.execute(simple_script);
        
        // 示例 2: 读取并执行 hello.dkvs 文件
        std::cout << "\n2. Executing hello.dkvs script file:\n";
        std::string hello_script = readFile("examples/hello.dkvs");
        script.execute(hello_script);
        
        // 示例 3: 执行 expr.dkvs 文件
        std::cout << "\n3. Executing expr.dkvs script file:\n";
        std::string expr_script = readFile("examples/expr.dkvs");
        script.execute(expr_script);
        
        std::cout << "\n=== All tests completed successfully! ===\n";
        
        return 0;
    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
        return 1;
    }
}
