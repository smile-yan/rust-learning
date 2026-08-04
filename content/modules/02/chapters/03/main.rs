use tokio::time::{sleep, Duration};

// async fn 返回一个 Future：调用时不立即执行，被 await 才真正驱动
async fn say_hello(name: &str, delay: u64) {
    // .await 挂起当前任务等待完成，期间不阻塞所在线程
    sleep(Duration::from_millis(delay)).await;
    println!("你好, {}!", name);
}

// #[tokio::main] 把 async main 包装成同步入口，并启动 tokio 运行时
#[tokio::main]
async fn main() {
    // join! 同时等待多个 Future
    let (r1, r2) = tokio::join!(
        say_hello("Alice", 100),
        say_hello("Bob", 50),
    );
    let _ = (r1, r2);

    // spawn 创建新任务
    let handle = tokio::spawn(async {
        say_hello("Charlie", 30).await;
    });
    // await JoinHandle 等待任务结束；任务 panic 时这里返回 Err
    handle.await.unwrap();

    println!("所有任务完成");
}
