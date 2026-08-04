fn main() {
    // String 拥有数据：存于堆上，可增长、可修改
    let mut s = String::from("hello");
    s.push_str(" world");
    println!("String: {}", s);

    // &str 是借用：只读视图，不拿走所有权，s 之后仍可使用
    let slice: &str = &s;
    println!("&str: {}", slice);

    // 字符串字面量是 '&static str'
    let literal: &str = "Rust";
    println!("字面量: {}", literal);

    // &str -> String
    let owned = literal.to_string();
    println!("转换后: {}", owned);
}
