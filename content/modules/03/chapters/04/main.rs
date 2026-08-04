// 组合：Car 由 Engine 和 Wheels 组成
struct Engine;
struct Wheels;

impl Engine {
    fn start(&self) {
        println!("发动机启动");
    }
}

impl Wheels {
    fn roll(&self) {
        println!("车轮转动");
    }
}

struct Car {
    engine: Engine,
    wheels: Wheels,
}

impl Car {
    // 组合的关键：Car 不继承任何行为，而是把具体工作委托给内部组件
    fn start(&self) {
        self.engine.start();
        self.wheels.roll();
        println!("汽车开始行驶");
    }
}

// trait：定义飞行的能力
trait Flyable {
    fn fly(&self);
}

// Bird 只需实现 Flyable，就能获得被 let_it_fly 使用的能力
struct Bird;
impl Flyable for Bird {
    fn fly(&self) {
        println!("鸟在飞翔");
    }
}

// &dyn Flyable 是 trait 对象：运行时动态分发，
// 接受任何实现了 Flyable 的类型
fn let_it_fly(f: &dyn Flyable) {
    f.fly();
}

fn main() {
    let car = Car { engine: Engine, wheels: Wheels };
    car.start();

    let bird = Bird;
    let_it_fly(&bird);
}
