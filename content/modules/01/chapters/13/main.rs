fn main() {
    dotenv::dotenv().ok();

    let host = std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port: u16 = std::env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .expect("PORT 必须是数字");

    println!("服务启动: http://{}:{}", host, port);
}
