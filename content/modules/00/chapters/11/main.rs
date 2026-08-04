/// 计算两个整数的和
///
/// # Examples
///
/// ```
/// assert_eq!(add(2, 3), 5);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    // 这是单行注释
    let x = 10;
    let y = 3;

    /*
     * 这是多行注释，
     * 常用于临时禁用一段代码或详细说明。
     */

    println!("算术: {} + {} = {}", x, y, add(x, y));
    // 整数除法会截断小数部分：10 / 3 结果是 3
    println!("整除: {} / {} = {}", x, y, x / y);
    // % 取余数：10 % 3 结果是 1
    println!("取余: {} % {} = {}", x, y, x % y);

    let a = true;
    let b = false;
    // 逻辑运算符 &&、||、!，其中 && 和 || 会短路求值
    println!("逻辑与: {}", a && b);
    println!("逻辑或: {}", a || b);
    println!("逻辑非: {}", !a);

    // 0b 前缀是二进制字面量，下划线只是方便阅读的分隔符
    let n: u8 = 0b1010_1010;
    // {:08b} 按 8 位二进制格式化输出，不足位补 0
    println!("位与: {:08b}", n & 0b1111_0000);
    println!("左移: {:08b}", n << 2);

    // += 是复合赋值，等价于 total = total + x
    let mut total = 0;
    total += x;
    total += y;
    println!("复合赋值结果: {}", total);
}
