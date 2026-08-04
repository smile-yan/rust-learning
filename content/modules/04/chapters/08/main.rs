fn main() {
    // vec! 宏创建向量，存放学习路径的每一步
    let steps = vec![
        "阅读 Rust Book",
        "完成 Rustlings 练习",
        "学习本教程",
        "阅读优秀开源项目",
        "动手写小项目",
    ];
    
    println!("Rust 学习路径:");
    // enumerate 遍历时同时产生从 0 开始的序号
    for (i, step) in steps.iter().enumerate() {
        println!("{}. {}", i + 1, step);
    }
    
    let days = 100;
    println!("\n坚持 {} 天，你会发现 Rust 并不可怕。", days);
}
