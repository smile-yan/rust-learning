// main 函数是程序入口，每个可执行程序必须有且只有一个
fn main() {
    // println! 是宏调用，注意末尾的 !
    println!("Hello, world!");
    println!("你好，Rust 学习之旅！");

    // 使用占位符输出变量
    // {} 会被后面的参数按顺序替换
    let name = "Rust";
    println!("欢迎学习 {}", name);

    // 多个占位符按顺序填充
    // let (x, y) = ... 是解构赋值，一次绑定两个变量
    let (x, y) = (42, 3.14);
    println!("坐标: x = {}, y = {}", x, y);

    // Debug 格式输出
    // 数组没有"美化显示"的方式，必须用 {:?} 打印
    let nums = [1, 2, 3];
    println!("数组: {:?}", nums);
}