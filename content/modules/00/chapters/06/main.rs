// ---- 所有权与借用：引用不拿走所有权 ----
fn main() {
    // 不可变借用
    // &s1 传的是引用，所有权不转移，后面还能继续用 s1
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("'{}' 的长度是 {}。", s1, len);

    // 可变借用
    // 同一作用域内可变借用最多一个，且不能和不可变借用共存
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("修改后: {}", s2);

    // 所有权转移与 clone
    // 若写 let s3 = s1 会移动所有权使 s1 失效；clone 是显式深拷贝
    let s3 = s1.clone();
    println!("s1 = {}, s3 = {}", s1, s3);

    // 切片不拥有数据
    // &text[..5] 等价于 &text[0..5]，只是对原字符串的借用
    let text = String::from("hello world");
    let first = &text[..5];
    let second = &text[6..];
    println!("{} | {}", first, second);
}

// 参数 &String 表示借用，函数用完不释放原数据
fn calculate_length(s: &String) -> usize {
    s.len()
}

// &mut String 表示可变借用，允许在函数内修改内容
fn change(s: &mut String) {
    s.push_str(", world");
}