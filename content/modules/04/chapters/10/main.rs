fn main() {
    // 常见错误示例 1：所有权移动
    let s = String::from("hello");
    // 赋值即移动：所有权转给 _s2，s 从此失效
    let _s2 = s;
    // println!("{}", s); // borrow of moved value
    
    // 正确做法：克隆或借用
    let s3 = String::from("hello");
    let _s4 = s3.clone();
    println!("s3 仍然可用: {}", s3);
    
    // 常见错误示例 2：可变借用冲突
    let mut v = vec![1, 2, 3];
    // 不可变引用 _first 存活期间，不能对 v 做可变操作
    let _first = &v[0];
    // v.push(4); // 不能同时有不可变引用和可变引用
    
    println!("遇到编译错误不要慌，仔细阅读错误信息，Rust 会告诉你怎么修。");
}
