// 泛型函数：T 是类型参数，PartialOrd 约束保证可以用 > 比较
// 返回引用 &T 而不是 T，避免移动所有权（T 可能没有实现 Copy）
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for item in list {
        if item > max {
            max = item;
        }
    }
    max
}

// 泛型结构体：x 和 y 必须是同一类型 T；
// derive(Debug) 让 {:?} 可以打印
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// impl 后面的 <T> 是声明泛型参数，
// 这些方法对所有类型的 Point<T> 都可用
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 只为 Point<f64> 实现的方法，其他类型的 Point 调不到它
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    // 调用时编译器自动推断 T = i32
    println!("最大值是 {}", largest(&numbers));

    // 同一个函数复用在 char 上，这正是泛型的价值
    let chars = vec!['y', 'm', 'a', 'q'];
    println!("最大字符是 {}", largest(&chars));

    // 整数推断为 i32，这里用的是通用实现里的 x() 方法
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    println!("点: {:?}", p);

    // f64 类型的 Point 额外拥有 distance_from_origin 方法
    let fp = Point { x: 3.0, y: 4.0 };
    println!("到原点距离: {}", fp.distance_from_origin());
}
