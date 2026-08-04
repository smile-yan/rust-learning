fn main() {
    greet("Rust");

    // Rust 中函数可以先调用后定义，编译器会处理整个文件
    let sum = add(5, 3);
    println!("5 + 3 = {}", sum);

    // 代码块作为表达式
    let y = {
        let x = 3;
        x + 1  // 没有分号，这是返回值
    };
    println!("y = {}", y);

    // 函数体最后一个表达式就是返回值
    let value = with_side_effect();
    println!("返回值: {}", value);

    // 值传递：i32 实现了 Copy，传参时复制副本，原变量不受影响
    let a = 5;
    plus_one_value(a);
    println!("值传递后 a 仍然是 {}", a);

    // 引用传递：&mut 借出原值，函数内修改直接作用于原变量
    let mut b = 5;
    plus_one_ref(&mut b);
    println!("引用传递后 b 变成了 {}", b);
}

// 参数必须标注类型；&str 是字符串切片类型
fn greet(name: &str) {
    println!("你好, {}!", name);
}

// -> i32 声明返回值类型
fn add(a: i32, b: i32) -> i32 {
    a + b  // 返回值，没有分号
}

fn with_side_effect() -> i32 {
    println!("函数被调用了");
    // 若给 42 加上分号就变成语句，函数返回值类型就对不上了
    42
}

// 值传递：x 是原值的副本，修改副本不影响调用方
fn plus_one_value(mut x: i32) {
    x += 1;
    println!("函数内副本加一后 x = {}", x);
}

// 引用传递：x 是对原变量的可变借用，*x 解引用后修改的是原值
fn plus_one_ref(x: &mut i32) {
    *x += 1;
}