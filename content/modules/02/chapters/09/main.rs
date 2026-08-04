// 这是一个 no_std 风格的伪代码示例，展示嵌入式 Rust 的核心思想
// 实际项目需要目标板和对应 crate

// #![no_std] 不链接标准库，#![no_main] 没有常规 main，
// 入口由 #[entry] 指定
// #![no_std]
// #![no_main]

// use cortex_m_rt::entry;

// #[entry]
// fn main() -> ! {
//     // 初始化硬件
//     // 配置 GPIO
//     // 主循环
//     loop {
//         // 闪烁 LED
//     }
// }

fn main() {
    println!("嵌入式 Rust 示例");
    println!("在真实嵌入式项目中，你会使用 #![no_std] 和硬件抽象层 crate。");
    println!("Rust 的类型系统可以帮助你在编译期发现硬件配置错误。");
    
    // 模拟主循环
    for i in 0..3 {
        println!("循环 {}: LED 状态切换", i);
    }
}
