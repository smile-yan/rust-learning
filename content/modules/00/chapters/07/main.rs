// derive 宏让编译器自动实现常用 trait：
// Debug 用于 {:?} 打印，Clone 用于复制，PartialEq 用于比较
#[derive(Debug, Clone, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl 块中为结构体定义方法和关联函数
impl Rectangle {
    // 关联函数，常用于构造
    // 没有 self 参数，调用时用 类型名::函数名
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }

    // 方法
    // 第一个参数 &self 表示只读借用调用者
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 可变方法示例
    // &mut self 才能修改字段，且要求调用者用 mut 声明
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
}

fn main() {
    let mut rect1 = Rectangle { width: 30, height: 50 };
    // {:?} 能打印全靠上面 derive 出的 Debug
    println!("矩形: {:?}", rect1);
    println!("矩形面积: {}", rect1.area());

    rect1.scale(2);
    println!("放大后: {:?}, 面积: {}", rect1, rect1.area());

    let rect2 = Rectangle { width: 10, height: 40 };
    println!("rect1 能容纳 rect2? {}", rect1.can_hold(&rect2));

    // :: 语法调用关联函数，相当于其他语言的"静态工厂方法"
    let sq = Rectangle::square(20);
    println!("正方形: {:?}, 面积: {}", sq, sq.area());
}
