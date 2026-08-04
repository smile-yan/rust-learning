// 尽可能展示多个关键字的使用方式
use std::fmt::Debug;

// const 是编译期常量；static 是有固定内存地址的全局变量
const GREETING: &str = "Rust";
// static mut 是可变全局变量，读写都必须放在 unsafe 块中
static mut COUNTER: usize = 0;

// trait 定义一组能力（类似其他语言的接口）
trait Speak {
    fn speak(&self) -> String;
}

// impl 某 trait for 某类型：为类型实现该能力
struct Person;
impl Speak for Person {
    fn speak(&self) -> String {
        String::from("Hello")
    }
}

// unsafe fn：调用者必须用 unsafe 块包裹，自行保证安全前提
unsafe fn increment_counter() {
    COUNTER += 1;
}

fn main() {
    // loop 是无限循环，break 退出，continue 跳到下一轮
    let mut count = 0;
    loop {
        count += 1;
        if count == 2 {
            break;
        } else {
            continue;
        }
    }

    let person = Person;
    let speech = person.speak();

    // for / in / while
    let mut total = 0;
    for i in 0..5 {
        total += i;
    }
    while total > 0 {
        total -= 1;
    }

    // match / true / false
    let is_ready = true;
    match is_ready {
        true => println!("准备就绪"),
        false => println!("尚未就绪"),
    }

    // dyn trait 对象
    let speaker: &dyn Speak = &person;
    println!("{}", speaker.speak());

    // unsafe 调用
    unsafe {
        increment_counter();
        println!("COUNTER = {}", COUNTER);
    }

    println!("{}", GREETING);
}
