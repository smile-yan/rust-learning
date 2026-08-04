fn main() {
    // Rust 没有垃圾回收器：靠所有权在编译期确定内存何时释放
    // String 的数据在堆上，drop 时连同堆内存一起释放
    {
        // s 在这里创建
        let s = String::from("hello");
        println!("使用 s: {}", s);
    } // s 在这里离开作用域，drop 被自动调用，内存被释放
    
    println!("s 已经被释放，不存在内存泄漏");
    
    // 基本类型存储在栈上，离开作用域直接弹出
    let x = 42;
    println!("栈上的整数: {}", x);
}
