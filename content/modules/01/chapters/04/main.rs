fn main() {
    // 闭包：参数和返回类型可省略，类型由第一次调用推断并固定
    let add_one = |x| x + 1;
    println!("5 + 1 = {}", add_one(5));

    let offset = 10;
    let add_offset = |x| x + offset;  // 捕获环境变量
    println!("5 + offset = {}", add_offset(5));

    let v = vec![1, 2, 3, 4, 5];

    // 迭代器适配器是惰性的：map/filter 只是组装，
    // sum() 才真正消费求值
    let sum_of_squares: i32 = v
        .iter()
        .map(|x| x * x)
        .filter(|x| *x > 5)
        .sum();
    println!("大于 5 的平方和: {}", sum_of_squares);

    let words = vec!["hello", "rust", "world"];
    // collect 把迭代器收集成集合，目标类型由 Vec<String> 标注决定
    let upper: Vec<String> = words
        .iter()
        .map(|w| w.to_uppercase())
        .collect();
    println!("{:?}", upper);

    // fold 手动折叠迭代器：acc 是累积值，这里用来求最大值
    let max = v.iter()
        .fold(0, |acc, x| if *x > acc { *x } else { acc });
    println!("最大值: {}", max);

    // 自定义简单的迭代器
    let mut counter = Counter::new();
    println!("计数器: {:?}", counter.next());
    println!("计数器: {:?}", counter.next());
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// 只要实现 Iterator trait，自定义类型就能接入整个迭代器生态
impl Iterator for Counter {
    // 关联类型：声明每次迭代产出的元素类型
    type Item = u32;

    // next 返回 Some 表示还有值，返回 None 时迭代结束
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
