// 显式生命周期标注：'a 表示返回值引用最多活到 x、y 中较短的那个
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        // 这里合法：result 使用时 string1 和 string2 都还活着
        let result = longest(string1.as_str(), string2.as_str());
        println!("更长的字符串是: {}", result);
    }
    
    // 下面会编译错误：s 在内层作用域结束时被释放，
    // result2 活得比它久就是悬垂引用
    // let result2;
    // {
    //     let s = String::from("short");
    //     result2 = longest(string1.as_str(), s.as_str());
    // }
    // println!("{}", result2);
}
