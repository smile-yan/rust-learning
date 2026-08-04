use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // Box 在堆上分配
    let b = Box::new(5);
    println!("b = {}", b);

    // Rc 共享所有权
    // Rc<RefCell<T>> 是单线程下「共享 + 可变」的经典组合：
    // Rc 提供多所有权，RefCell 提供内部可变性
    let shared = Rc::new(RefCell::new(vec![1, 2, 3]));
    println!("引用计数: {}", Rc::strong_count(&shared));

    {
        // Rc::clone 只增加引用计数，不做深拷贝，开销很小
        let shared2 = Rc::clone(&shared);
        println!("clone 后引用计数: {}", Rc::strong_count(&shared));

        // borrow_mut 在运行时检查借用规则；
        // shared2 离开作用域后计数自动减一
        shared2.borrow_mut().push(4);
    }

    shared.borrow_mut().push(5);
    println!("共享数据: {:?}", shared.borrow());
    println!("最终引用计数: {}", Rc::strong_count(&shared));
}
