fn main() {
    // vec! 宏创建向量（可增长数组）
    let domains = vec![
        "系统编程",
        "Web 后端",
        "命令行工具",
        "区块链",
        "游戏开发",
        "云原生",
    ];
    
    println!("Rust 适合的应用领域:");
    // for 直接逐个取出向量中的元素
    for domain in domains {
        println!("  - {}", domain);
    }
    
    println!("\n无论哪个领域，Rust 都能提供 C/C++ 级别的性能，同时保证内存安全。");
}
