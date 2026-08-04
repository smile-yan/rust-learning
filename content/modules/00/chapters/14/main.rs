// 枚举的每个变体可以携带不同类型、不同数量的数据
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn main() {
    let msg = Message::Move { x: 10, y: 20 };

    // match 必须覆盖所有变体；每个分支可以解构出变体携带的数据
    match msg {
        Message::Quit => println!("退出"),
        Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
        Message::Write(text) => println!("写入: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("颜色: RGB({}, {}, {})", r, g, b);
        },
    }

    // ---- Option<T>：表示"可能有值" ----
    // 只有 Some(值) 和 None 两种

    // if let 是只关心某一种匹配时的简洁写法
    let some_number = Some(5);
    if let Some(n) = some_number {
        println!("值是: {}", n);
    }

    // None 表示没有值；类型标注不可少，编译器无法推断
    let absent: Option<i32> = None;
    println!("absent: {:?}", absent);

    // if n > 5 是匹配守卫，为分支附加额外条件
    let val = Some(7);
    match val {
        Some(n) if n > 5 => println!("大于 5: {}", n),
        Some(_) => println!("不大于 5"),
        None => println!("没有值"),
    }

    // unwrap_or：有值则取出，是 None 时返回给定的默认值
    let score: Option<u32> = Some(85);
    let final_score = score.unwrap_or(60);
    println!("最终分数: {}", final_score);
}
