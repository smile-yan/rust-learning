fn main() {
    // let 声明的变量默认不可变，这是 Rust 的安全设计
    let x = 5;
    println!("x = {}", x);
    // x = 6; // 取消注释会报错，因为 x 不可变

    // 加上 mut 关键字，变量才能被重新赋值
    let mut y = 10;
    println!("修改前 y = {}", y);
    y = 20;
    println!("修改后 y = {}", y);

    // const 声明常量：必须标注类型，命名习惯全大写
    // 100_000 中的下划线只是可读性分隔符，不影响数值
    const MAX_POINTS: u32 = 100_000;
    println!("常量 MAX_POINTS = {}", MAX_POINTS);

    // 隐藏：用同名新变量覆盖旧变量
    // 新变量允许换类型，这是它与 mut 重新赋值的关键区别
    let spaces = "   ";
    let spaces = spaces.len();
    println!("隐藏后的 spaces = {}", spaces);

    // parse 方法需要显式类型标注
    // expect 在解析失败时直接让程序 panic 并显示这条提示
    let guess: u32 = "42".parse().expect("不是数字");
    println!("解析结果: {}", guess);
}