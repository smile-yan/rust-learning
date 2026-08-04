// 这个文件演示了模块系统的核心语法。
// 在真实项目中，这些模块通常分别放在
// my_module.rs 或 my_module/mod.rs 中。

// mod 定义模块；模块内的项默认私有，加 pub 才对外可见
mod my_module {
    pub fn public_fn() {
        println!("这是公共函数");
    }

    // 没有 pub，只能在 my_module 内部调用
    fn _private_fn() {
        println!("这是私有函数");
    }

    // 模块可以嵌套，内层模块同样用 pub 控制可见性
    pub mod nested {
        pub fn greet() {
            println!("来自嵌套模块的问候");
        }
    }
}

mod utils {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

// use 把深层路径引入当前作用域，之后可直接写 greet()
use my_module::nested::greet;

fn main() {
    my_module::public_fn();
    greet();

    let result = utils::add(2, 3);
    println!("utils::add(2, 3) = {}", result);

    // self 指当前模块，crate 指当前 crate 根
    let _local = self::utils::add(1, 1);
    println!("通过 self 调用: {}", _local);
}
