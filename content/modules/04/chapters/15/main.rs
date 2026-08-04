fn main() {
    // 完全不需要手动 free
    let s = String::from("Rust 自动管理内存");
    println!("{}", s);
} // s 在这里自动释放

// 与 C 交互时才可能需要手动管理（不安全代码）
unsafe fn manual_memory_example() {
    // into_raw 把 Box 转为裸指针，放弃自动释放的责任转移给调用者
    let ptr = Box::into_raw(Box::new(42));
    println!("裸指针: {:?}", ptr);
    // 必须手动释放：from_raw 还原为 Box 再由 drop 释放，漏掉则泄漏
    drop(Box::from_raw(ptr));
}

fn main2() {
    unsafe {
        manual_memory_example();
    }
    println!("手动管理内存只用于 unsafe 或 FFI 场景");
}
