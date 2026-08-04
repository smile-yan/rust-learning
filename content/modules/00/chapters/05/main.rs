fn main() {
    let number = 7;
    // if 的条件必须是 bool，Rust 不会把数字自动当作真值
    if number % 2 == 0 {
        println!("偶数");
    } else {
        println!("奇数");
    }

    // loop 可以返回值
    // break 后面跟的值就是整个 loop 表达式的结果
    let mut count = 0;
    let result = loop {
        count += 1;
        if count >= 3 {
            break count * 2;
        }
    };
    println!("loop 结果: {}", result);

    // while 循环
    let mut n = 3;
    while n > 0 {
        println!("while: {}", n);
        n -= 1;
    }

    // for 循环遍历范围
    // 1..4 不包含 4，即打印 1、2、3
    for i in 1..4 {
        println!("for: {}", i);
    }

    // for 循环遍历集合并获取索引
    // enumerate() 让每次迭代同时拿到 (索引, 值)
    let arr = [10, 20, 30];
    for (idx, val) in arr.iter().enumerate() {
        println!("arr[{}] = {}", idx, val);
    }

    // if let 简化模式匹配
    // 只关心 Some 这一种情况时，比写完整 match 更简洁
    let some_value = Some(5);
    if let Some(v) = some_value {
        println!("if let 匹配到: {}", v);
    }
}