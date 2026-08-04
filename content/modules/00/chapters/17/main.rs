// pub 表示对外公开，这些函数构成库对外的接口
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Result<T, E> 表示可能失败的操作：Ok(结果) 或 Err(错误)
pub fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("除数不能为零")
    } else {
        Ok(a / b)
    }
}

/// 返回一个问候语
///
/// # Examples
///
/// ```
/// assert_eq!(rust_projects::greet("Rust"), "Hello, Rust!");
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    println!("{}", greet("Rust"));
    println!("2 + 3 = {}", add(2, 3));
    // {:?} 以调试格式打印，可以直接输出 Result
    println!("10 / 2 = {:?}", divide(10, 2));
    println!("10 / 0 = {:?}", divide(10, 0));
}

// #[cfg(test)] 表示这个模块只在 cargo test 时才编译
#[cfg(test)]
mod tests {
    // 引入外层模块的全部内容，测试里才能直接调用 add 等函数
    use super::*;

    // #[test] 把一个函数标记为测试用例
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    // unwrap 在遇到 Err 时会 panic，从而让测试失败
    #[test]
    fn test_divide_ok() {
        assert_eq!(divide(10, 2).unwrap(), 5);
    }

    // is_err() 判断结果是否为 Err 变体
    #[test]
    fn test_divide_by_zero() {
        assert!(divide(10, 0).is_err());
    }

    #[test]
    fn test_greet() {
        assert_eq!(greet("Rust"), "Hello, Rust!");
    }
}
