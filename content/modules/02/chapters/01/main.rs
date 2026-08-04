fn main() {
    let mut num = 5;

    // 创建裸指针：*const 只读、*mut 可写，创建本身不需要 unsafe
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 解引用裸指针、调用 unsafe 函数都必须放在 unsafe 块中，
    // 安全性由程序员自行保证
    unsafe {
        println!("r1 指向: {}", *r1);
        // 通过可变裸指针直接修改内存中的值
        *r2 += 1;
        println!("r2 指向: {}", *r2);
        dangerous();
    }

    // 任意地址的裸指针，仅作演示，不要解引用
    let address = 0x012345usize;
    let _r = address as *const i32;

    println!("unsafe 代码块结束");
}

// unsafe fn 表示该函数包含不安全操作，调用它也必须使用 unsafe 块
unsafe fn dangerous() {
    println!("调用 unsafe 函数");
}
