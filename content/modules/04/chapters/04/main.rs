// 返回 Result 表示可能失败：Ok 装成功值，Err 装错误信息
fn may_fail(input: i32) -> Result<i32, String> {
    if input > 0 {
        Ok(input * 2)
    } else {
        Err(String::from("输入必须大于 0"))
    }
}

// 只有返回 Result 或 Option 的函数内部才能使用 ?
fn double_positive(input: i32) -> Result<i32, String> {
    // ? 会在 Err 时提前返回错误
    let value = may_fail(input)?;
    Ok(value + 1)
}

fn main() {
    // unwrap：成功则取值，失败直接 panic，仅在确信不会失败时使用
    let good = may_fail(5).unwrap();
    println!("unwrap 结果: {}", good);
    
    // ? 示例：传播错误，传入 -1 时 may_fail 返回 Err，
    // double_positive 随之提前返回
    match double_positive(-1) {
        Ok(n) => println!("成功: {}", n),
        Err(e) => println!("错误: {}", e),
    }
    
    match double_positive(5) {
        Ok(n) => println!("成功: {}", n),
        Err(e) => println!("错误: {}", e),
    }
}
