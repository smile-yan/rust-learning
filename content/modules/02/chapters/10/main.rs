use std::time::Instant;

// O(n) 迭代求和：逐个累加，耗时随 n 线性增长
fn sum_iterative(n: u64) -> u64 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
}

// O(1) 公式求和：等差数列求和公式，耗时与 n 无关
fn sum_formula(n: u64) -> u64 {
    n * (n + 1) / 2
}

fn main() {
    // 计时对比请用 release 模式运行，debug 模式未做优化，结果会失真
    let n = 10_000_000;

    // Instant::now() 记录时间点，elapsed() 返回至今经过的时长
    let start = Instant::now();
    let r1 = sum_iterative(n);
    let t1 = start.elapsed();

    // 同样的方式单独为公式法计时，便于对比
    let start = Instant::now();
    let r2 = sum_formula(n);
    let t2 = start.elapsed();

    println!("迭代求和: {}, 耗时: {:?}", r1, t1);
    println!("公式求和: {}, 耗时: {:?}", r2, t2);
    println!("算法优化往往比语言层面的微优化更有效！");
}
