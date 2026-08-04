use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    // 读写锁示例
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles = vec![];

    // 多个读线程
    for _ in 0..3 {
        // Arc 提供线程安全的共享所有权，clone 只增加引用计数
        let data = Arc::clone(&data);
        // move 闭包把 data 的所有权移入新线程
        let handle = thread::spawn(move || {
            let read = data.read().unwrap();
            println!("读取: {:?}", *read);
        });
        handles.push(handle);
    }

    // 一个写线程
    let data = Arc::clone(&data);
    let write_handle = thread::spawn(move || {
        // write() 获取独占写锁，会等待所有读锁释放
        let mut write = data.write().unwrap();
        write.push(4);
        println!("写入后: {:?}", *write);
    });
    handles.push(write_handle);

    // join 等待所有线程结束，避免 main 提前退出
    for handle in handles {
        handle.join().unwrap();
    }
}
