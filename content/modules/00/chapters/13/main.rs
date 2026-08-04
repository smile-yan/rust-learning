fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 迭代器是惰性的：filter 筛选偶数、map 取出值，
    // 直到 collect 才真正执行
    // |&&x| 用两个 & 解构双重引用：
    // iter 产生 &i32，filter 又借了一层
    let evens: Vec<i32> = nums
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x)
        .collect();
    println!("偶数: {:?}", evens);

    // sum() 直接对整个迭代器求和
    let sum: i32 = nums.iter().sum();
    println!("总和: {}", sum);

    // fold 从初始值 1 开始，把每个元素依次乘进累加器 acc
    let product = nums.iter().fold(1, |acc, x| acc * x);
    println!("乘积: {}", product);

    // any 判断是否存在满足条件的元素，找到一个就立即返回
    let has_even = nums.iter().any(|&x| x % 2 == 0);
    println!("包含偶数: {}", has_even);

    // chars() 按字符遍历字符串，enumerate() 附带下标
    for (idx, val) in "Rust".chars().enumerate() {
        println!("chars[{}]: {}", idx, val);
    }

    // take(3) 只取前 3 个元素；cloned 把迭代器里的 &i32 转成 i32
    let first_three: Vec<i32> = nums.iter()
        .take(3)
        .cloned()
        .collect();
    println!("前三个: {:?}", first_three);

    // skip(5) 跳过前 5 个元素，取剩余部分
    let skipped: Vec<i32> = nums.iter().skip(5).cloned().collect();
    println!("跳过前五个: {:?}", skipped);
}
