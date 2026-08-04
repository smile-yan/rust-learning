// 引入标准库中的哈希表类型
use std::collections::HashMap;

fn main() {
    // Vec
    // vec! 宏创建动态数组；声明 mut 后才能 push
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("Vec: {:?}", v);

    // iter() 产生迭代器，map 对每个元素做变换，
    // collect 把结果收集成新的 Vec
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("翻倍: {:?}", doubled);

    // filter 按条件保留元素；闭包参数是 &&i32，用 *x 解引用再取余
    let evens: Vec<&i32> = v.iter()
        .filter(|x| *x % 2 == 0)
        .collect();
    println!("偶数: {:?}", evens);

    // String
    // String 可增长：push_str 追加字符串，push 追加单个字符
    let mut s = String::from("hello");
    s.push_str(", world");
    s.push('!');
    println!("String: {}", s);

    // HashMap
    // insert 插入键值对，键和值的类型由第一次插入推断
    let mut scores = HashMap::new();
    scores.insert("Alice", 95);
    scores.insert("Bob", 87);

    // 遍历 &scores 只是借用，不会夺走 HashMap 的所有权
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }

    // get 返回 Option：找到是 Some(分数)，找不到是 None，不会 panic
    match scores.get("Alice") {
        Some(score) => println!("Alice 的分数: {}", score),
        None => println!("未找到"),
    }

    // entry API
    // or_insert：键不存在时才插入默认值，常用于初始化或计数
    scores.entry("Charlie").or_insert(78);
    println!("Charlie: {}", scores["Charlie"]);
}
