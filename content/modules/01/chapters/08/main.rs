use std::fs::File;
use std::io::{self, Read};

// ? 运算符：出错时提前返回并把错误交给调用者，成功时取出其中的值
fn read_username_from_file(
    path: &str,
) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

// 用 Result 显式表达「可能失败」，这里用 String 作错误类型
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("除数不能为零"))
    } else {
        Ok(a / b)
    }
}

// #[cfg(test)] 标注的模块只在 cargo test 时编译
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divide_normal() {
        assert_eq!(divide(10.0, 2.0).unwrap(), 5.0);
    }

    // 验证错误路径：除零应当返回 Err
    #[test]
    fn divide_by_zero() {
        assert!(divide(10.0, 0.0).is_err());
    }
}

fn main() {
    // 用 match 穷尽处理 Ok / Err 两种情况
    match divide(10.0, 2.0) {
        Ok(result) => println!("结果: {}", result),
        Err(e) => println!("错误: {}", e),
    }

    // 这个文件可能不存在，会返回错误
    match read_username_from_file("nonexistent.txt") {
        Ok(name) => println!("用户名: {}", name),
        Err(e) => println!("读取失败: {}", e),
    }
}
