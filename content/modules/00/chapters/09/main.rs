fn main() {
    // Result 是枚举：Ok 装成功值，Err 装错误信息
    match divide(10, 2) {
        Ok(n) => println!("10 / 2 = {}", n),
        Err(e) => println!("错误: {}", e),
    }

    match divide(10, 0) {
        Ok(n) => println!("10 / 0 = {}", n),
        Err(e) => println!("错误: {}", e),
    }

    // 提供默认值
    // 出错时直接用 0 代替，错误被悄悄忽略
    let safe = divide(10, 0).unwrap_or(0);
    println!("unwrap_or 结果: {}", safe);

    // ? 操作符传播错误
    match safe_divide(10, 0) {
        Ok(n) => println!("传播后结果: {}", n),
        Err(e) => println!("传播错误: {}", e),
    }
}

// 返回值 Result<i32, String>：成功给 i32，失败给 String
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("除数不能为零"))
    } else {
        Ok(a / b)
    }
}

fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    // ? 遇到 Err 会立刻从当前函数返回，Ok 时取出里面的值
    let r = divide(a, b)?;
    Ok(r * 2)
}
