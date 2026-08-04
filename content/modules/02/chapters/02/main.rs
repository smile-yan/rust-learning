// macro_rules! 定义声明宏：按模式匹配输入，再展开为代码
macro_rules! say_hello {
    // 无参数模式：匹配 say_hello!() 这种空调用
    () => {
        println!("Hello!");
    };
}

// $func_name:ident 捕获一个标识符（这里是函数名）
macro_rules! create_function {
    ($func_name:ident) => {
        // 宏展开后生成一个名为 $func_name 的函数
        fn $func_name() {
            // stringify! 把捕获的代码原样转成字符串
            println!("调用了 {:?}", stringify!($func_name));
        }
    };
}

// $expression:expr 捕获任意一个表达式
macro_rules! print_result {
    ($expression:expr) => {
        println!(
            "{:?} = {:?}",
            stringify!($expression),
            $expression
        );
    };
}

// $(...),* 重复匹配：零个或多个表达式，以逗号分隔
macro_rules! vec_of_strings {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            // 对每个捕获到的表达式重复执行一次 push
            $(
                temp_vec.push($x.to_string());
            )*
            // 末尾表达式作为宏展开后的结果值
            temp_vec
        }
    };
}

// 调用宏在编译期展开，实际生成 foo 和 bar 两个函数
create_function!(foo);
create_function!(bar);

fn main() {
    say_hello!();
    foo();
    bar();
    print_result!(1u32 + 1);
    print_result!({
        let x = 2u32;
        x * x
    });

    let strings = vec_of_strings!["a", "b", "c"];
    println!("{:?}", strings);
}
