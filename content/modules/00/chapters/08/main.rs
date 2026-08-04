// 枚举的每个变体可以携带不同类型、不同数量的数据
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn main() {
    let msg = Message::Move { x: 10, y: 20 };

    // match 必须穷尽所有变体，漏一个就编译报错
    match msg {
        Message::Quit => println!("退出"),
        Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
        Message::Write(text) => println!("写入: {}", text),
    }

    // Option 的基本使用
    // Some(5) 的类型是 Option<i32>，Rust 用 Option 代替 null
    let some_number = Some(5);
    if let Some(n) = some_number {
        println!("值是: {}", n);
    }

    // None 无法推断具体类型，需要显式标注 Option<i32>
    let absent: Option<i32> = None;
    println!("absent: {:?}", absent);

    // 处理 Option 的常用模式
    let val = Some(7);
    match val {
        // if n > 5 是匹配守卫，给分支附加额外条件
        Some(n) if n > 5 => println!("大于 5: {}", n),
        Some(_) => println!("不大于 5"),
        None => println!("没有值"),
    }

    // unwrap_or 提供默认值
    let score: Option<u32> = None;
    let final_score = score.unwrap_or(60);
    println!("最终分数: {}", final_score);
}
