// 返回字符串切片 &str：借用输入字符串的一部分，不产生新的 String
fn first_word(s: &str) -> &str {
    // as_bytes 把字符串按字节查看，方便逐字节查找空格
    let bytes = s.as_bytes();

    // enumerate() 同时给出下标和元素；&item 通过模式解构取出字节值
    for (i, &item) in bytes.iter().enumerate() {
        // b' ' 是字节字面量；遇到空格就返回开头到下标 i 的切片
        if item == b' ' {
            return &s[..i];
        }
    }

    // 没找到空格就返回整个字符串的切片
    &s[..]
}

// 参数 &[i32] 是切片：数组和 Vec 都能传入，比固定数组更通用
fn sum_slice(nums: &[i32]) -> i32 {
    let mut total = 0;
    for n in nums {
        total += n;
    }
    total
}

fn main() {
    let text = String::from("hello world");
    // &String 会自动转换成 &str（deref 强制转换）
    let word = first_word(&text);
    println!("第一个单词: {}", word);

    let arr = [10, 20, 30, 40, 50];
    // arr[1..4] 取下标 1 到 3 的元素，含头不含尾
    println!("arr[1..4] 的和: {}", sum_slice(&arr[1..4]));
    // &arr 整个数组也能直接当切片传入
    println!("arr 全部元素的和: {}", sum_slice(&arr));

    // Vec 同样可以切片后传给同一个函数
    let v = vec![1, 2, 3, 4, 5];
    println!("Vec 前三个元素的和: {}", sum_slice(&v[..3]));

    // 字符串字面量本身就是 &str
    let literal: &str = "Rust 编程";
    println!("字符串切片: {}", literal);
}
