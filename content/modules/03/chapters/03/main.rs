use std::sync::mpsc;
use std::thread;

fn main() {
    // 使用 channel 在线程间传递数据所有权
    let (tx, rx) = mpsc::channel();

    // move 闭包把 tx 的所有权转移进子线程，主线程从此不再持有
    thread::spawn(move || {
        let data = String::from("来自子线程的数据");
        tx.send(data).unwrap();
        // data 的所有权已经发送出去，这里不能再使用
    });

    // recv 会阻塞当前线程，直到收到消息或所有发送端被丢弃
    let received = rx.recv().unwrap();
    println!("收到: {}", received);
}
