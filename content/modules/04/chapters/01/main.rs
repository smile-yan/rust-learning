// 默认不可变让编译器帮你抓住意外修改，这是 Rust 的安全设计
fn main() {
    // 不可变：值不会改变
    let x = 5;
    println!("x = {}", x);

    // 可变：值会改变
    let mut count = 0;
    for _ in 0..3 {
        count += 1;
    }
    println!("count = {}", count);

    // 编译错误示例（取消注释查看）
    // x = 6;
}
