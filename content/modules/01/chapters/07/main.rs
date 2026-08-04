use std::convert::TryInto;

// 元组结构体（newtype 模式）：用不同类型区分语义，避免米和千米混用
#[derive(Debug)]
struct Meters(u32);
#[derive(Debug)]
struct Kilometers(u32);

// 实现了 From，编译器会自动赠送反向调用的 Into
impl From<Kilometers> for Meters {
    fn from(k: Kilometers) -> Self {
        Meters(k.0 * 1000)
    }
}

// 类型别名只是起名字，不创建新类型，UserId 与 u64 可以互换
type UserId = u64;

fn process_id(id: UserId) {
    println!("处理用户 ID: {}", id);
}

// 返回类型 !（never 类型）表示函数永不正常返回，
// 常见于 panic 或死循环
fn always_fail() -> ! {
    panic!("这个函数永远不会正常返回");
}

fn main() {
    // as 转换
    let x = 42_i32;
    // as 做基础类型间的显式强转，可能静默截断，需要谨慎
    let y = x as f64;
    println!("{} -> {}", x, y);

    // From / Into
    let k = Kilometers(5);
    // into 的目标类型由变量标注决定
    let m: Meters = k.into();
    println!("{:?}", m);

    // TryInto
    let big: i64 = 300;
    // 可能失败的转换返回 Result：300 超出 u8 上限，会得到 Err
    let small: Result<u8, _> = big.try_into();
    println!("转换结果: {:?}", small);

    // 类型别名
    let id: UserId = 12345;
    process_id(id);

    // never 类型不会执行
    // always_fail();
}
