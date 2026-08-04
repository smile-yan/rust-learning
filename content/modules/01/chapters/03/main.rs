// 生命周期参数 'a：表示返回值的有效期不超过 x、y 中较短的那个
// 标注不改变实际存活时间，只是向编译器描述引用之间的关系
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 返回的是输入字符串的切片，返回值借用 s，因此共享同一生命周期 'a
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    // enumerate 同时给出下标和元素，方便按空格位置切分
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    s
}

// 结构体持有引用时必须标注生命周期：
// Book 实例不能比它引用的 title 活得更久
struct Book<'a> {
    title: &'a str,
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    // result 的有效期受 string1 和 string2 中较短者约束
    let result = longest(string1.as_str(), string2);
    println!("更长的字符串是: {}", result);

    println!("第一个单词: {}", first_word("hello world"));

    let title = String::from("Rust 程序设计");
    // book 借用了 title，所以 title 必须先于 book 销毁
    //（同作用域下天然满足这一点）
    let book = Book { title: title.as_str() };
    println!("书名: {}", book.title);

    // 注意：result2 的生命周期不能超过 s 的生命周期
    // 取消下面注释会编译失败
    // let result2;
    // {
    //     let s = String::from("efghijk");
    //     result2 = longest(string1.as_str(), s.as_str());
    // }
    // println!("{}" , result2);
}
