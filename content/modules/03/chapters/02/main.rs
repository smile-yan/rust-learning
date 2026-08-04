fn main() {
    // 所有权移动
    let s1 = String::from("Rust");
    let s2 = s1;
    // s1 的所有权已移给 s2，再访问 s1 会直接编译错误
    println!("s2: {}", s2);

    // 不可变借用
    let s3 = String::from("borrow");
    // 不可变引用可以同时存在任意多个
    let r1 = &s3;
    let r2 = &s3;
    println!("r1: {}, r2: {}", r1, r2);

    // 可变借用
    let mut s4 = String::from("mut");
    {
        // 可变引用同一时刻只能有一个，用短作用域及时释放
        let r3 = &mut s4;
        r3.push_str("able");
    } // r3 在这里结束
    println!("s4: {}", s4);
}
