use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let _listener = TcpListener::bind("127.0.0.1:8080");
    println!("TCP 监听已准备");

    match TcpStream::connect("127.0.0.1:8080") {
        Ok(mut stream) => {
            let msg = "Hello, TCP!";
            stream.write_all(msg.as_bytes()).unwrap();
            let mut buf = [0u8; 1024];
            let n = stream.read(&mut buf).unwrap();
            println!("收到 {} 字节", n);
        }
        Err(e) => {
            println!("连接失败: {}（服务器可能未启动）", e);
        }
    }
}
