// Rust 用结构体 + impl 实现封装
struct BankAccount {
    // 字段默认私有，外部不能直接读写 balance
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn new(owner: &str, balance: f64) -> Self {
        BankAccount {
            owner: owner.to_string(),
            balance,
        }
    }

    // 修改必须经过方法，非法金额被挡在门外
    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    // 只读访问通过 getter 暴露
    fn balance(&self) -> f64 {
        self.balance
    }
}

// Rust 用 trait 实现多态
trait Animal {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        println!("汪汪");
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("喵喵");
    }
}

// &dyn Animal 是 trait 对象：运行时动态分发，
// 可接受任何实现了 Animal 的类型
fn make_speak(animal: &dyn Animal) {
    animal.speak();
}

fn main() {
    let mut account = BankAccount::new("Alice", 100.0);
    account.deposit(50.0);
    println!("{} 的余额: {}", account.owner, account.balance());

    let dog = Dog;
    let cat = Cat;
    // 同一接口、不同实现各自响应，这就是多态
    make_speak(&dog);
    make_speak(&cat);
}
