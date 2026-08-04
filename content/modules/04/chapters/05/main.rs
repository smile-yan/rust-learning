// Rust 用 Option 代替 null：Some 装值，None 表示无值
fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Alice"))
    } else {
        None
    }
}

fn main() {
    let user1 = find_user(1);
    let user2 = find_user(99);
    
    // match 强制处理两种可能：编译器保证不会漏掉 None 分支
    match user1 {
        Some(name) => println!("找到用户: {}", name),
        None => println!("未找到用户"),
    }
    
    // unwrap_or 提供默认值
    let name = user2.unwrap_or(String::from("访客"));
    println!("用户2: {}", name);
    
    // map：仅在 Some 时执行闭包转换值，None 则原样跳过
    let upper = find_user(1).map(|n| n.to_uppercase());
    println!("大写: {:?}", upper);
}
