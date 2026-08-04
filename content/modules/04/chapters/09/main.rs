// 这个代码块展示了如何在 Rust 代码中使用 Cargo.toml 里声明的依赖。
// 实际运行时需要在 Cargo.toml 中添加对应依赖。

fn main() {
    // 假设 Cargo.toml 中有 serde 依赖
    // use serde::{Serialize, Deserialize};
    
    println!("Cargo.toml 示例:");
    println!("");
    // [package] 段：项目元信息（名称、版本、Rust 版本）
    println!("[package]");
    println!("name = \"my-project\"");
    println!("version = \"0.1.0\"");
    println!("edition = \"2021\"");
    println!("");
    // [dependencies] 段：声明第三方依赖及其版本要求
    println!("[dependencies]");
    println!("serde = \"1.0\"");
    println!("");
    println!("运行 cargo build 后，Cargo 会自动下载并编译依赖。");
}
