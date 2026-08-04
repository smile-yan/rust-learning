use std::rc::Rc;

fn main() {
    // 正常情况：Rc 在离开作用域后释放
    {
        // Rc 提供共享所有权，内部维护引用计数
        let data = Rc::new(vec![1, 2, 3]);
        // clone 只把计数加一（下面打印为 2），并不复制数据
        let _clone = Rc::clone(&data);
        println!("引用计数: {}", Rc::strong_count(&data));
    } // 两个 Rc 都离开作用域，计数归零，内存释放
    
    println!("正常 Rc 使用不会泄漏");
    
    // 循环引用示例（会导致泄漏，应避免）
    // a 和 b 互相持有对方，计数永远归不了零；
    // 真要打破循环需配合 Weak
    // let a = Rc::new(RefCell::new(None));
    // let b = Rc::new(RefCell::new(Some(Rc::clone(&a))));
    // *a.borrow_mut() = Some(Rc::clone(&b));
    
    println!("Rust 的借用规则帮助我们避免大多数循环引用。");
}
