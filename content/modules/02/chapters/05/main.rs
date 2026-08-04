use serde::{Deserialize, Serialize};

// derive 宏自动生成序列化/反序列化所需的 trait 实现
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

fn main() {
    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    // to_string_pretty 把结构体序列化为带缩进的 JSON 字符串
    let json = serde_json::to_string_pretty(&user).unwrap();
    println!("序列化结果:");
    println!("{}", json);

    // 标注目标类型 User，from_str 按该类型把 JSON 解析回结构体
    let parsed: User = serde_json::from_str(&json).unwrap();
    println!("反序列化结果: {:?}", parsed);

    // 模拟 SQLx 查询结果映射
    let rows = vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        },
    ];

    println!("\n用户列表:");
    // 遍历 &rows 只借用元素，不会消耗 Vec
    for u in &rows {
        println!("{} - {} - {}", u.id, u.name, u.email);
    }
}
