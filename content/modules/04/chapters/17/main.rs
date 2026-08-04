// 方案一：组合
struct Engine {
    horsepower: u32,
}

impl Engine {
    fn start(&self) {
        println!("发动机启动，{} 马力", self.horsepower);
    }
}

// Rust 没有继承，用"有一个"（组合）代替"是一个"（继承）
struct Car {
    brand: String,
    engine: Engine,  // 组合：Car 包含一个 Engine
}

impl Car {
    fn new(brand: &str, horsepower: u32) -> Self {
        Car {
            brand: brand.to_string(),
            engine: Engine { horsepower },
        }
    }

    fn start(&self) {
        println!("{} 汽车启动", self.brand);
        // 委托：把自己的工作转发给内部字段的方法
        self.engine.start();
    }
}

// 方案二：Trait 默认实现
trait Greetable {
    fn name(&self) -> &str;

    // 默认方法：实现者只需提供 name，就能复用 greet 的逻辑
    fn greet(&self) {
        println!("你好，我是 {}", self.name());
    }
}

struct Person {
    name: String,
}

impl Greetable for Person {
    fn name(&self) -> &str {
        &self.name
    }
}

fn main() {
    let car = Car::new("RustMobile", 200);
    car.start();

    let person = Person {
        name: String::from("Alice"),
    };
    person.greet();  // 使用 trait 默认实现
}
