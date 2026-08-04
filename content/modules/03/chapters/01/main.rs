fn main() {
    // 这行代码在编译期就会被阻止，而不是运行时崩溃
    // let s = String::from("hello");
    // let r = &s;
    // drop(s); // ❌ 编译错误：不能在使用引用后释放所有者
    // println!("{}", r);

    // 正确的做法：引用只在所有者有效期间使用
    let s = String::from("hello");
    let r = &s;
    println!("引用: {}", r);
    // NLL（非词法生命周期）：r 在最后一次使用后借用即结束，
    // s 则在其作用域结束时自动释放
}
