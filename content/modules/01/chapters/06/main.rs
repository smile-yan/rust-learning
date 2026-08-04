use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // 消息传递
    // mpsc 通道：多生产者单消费者，tx 是发送端，rx 是接收端
    let (tx, rx) = mpsc::channel();

    // move 闭包把 tx 的所有权移入新线程，
    // 保证同一时刻只有一个线程持有它
    thread::spawn(move || {
        let vals = vec!["hi", "from", "the", "thread"];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // rx 可当迭代器用：阻塞等待消息，所有发送端 drop 后循环自动结束
    for received in rx {
        println!("收到: {}", received);
    }

    // 共享状态
    // Arc 是线程安全版的 Rc（原子引用计数），
    // 配合 Mutex 实现跨线程共享可变数据
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // lock 返回锁守卫，离开作用域时自动释放锁（RAII）
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // join 等待所有线程结束，确保计数完成后再读结果
    for handle in handles {
        handle.join().unwrap();
    }

    println!("计数器结果: {}", *counter.lock().unwrap());
}
