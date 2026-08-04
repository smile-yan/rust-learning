use axum::{
    routing::get,
    Router,
    http::StatusCode,
};

// handler：返回 &'static str 会自动作为 200 OK 的响应体
async fn hello() -> &'static str {
    "Hello, Rust Web!"
}

// 返回 (StatusCode, String) 元组可同时指定状态码和响应体
async fn users() -> (StatusCode, String) {
    (StatusCode::OK, String::from("[\"Alice\", \"Bob\"]"))
}

// Router 把路径与 handler 关联，get(...) 限定只响应 GET 请求
fn app() -> Router {
    Router::new()
        .route("/", get(hello))
        .route("/users", get(users))
}

fn main() {
    // Playground 无法真正启动 HTTP 服务监听端口，
    // 这里演示路由定义与 handler 可以编译通过。
    let _router = app();
    println!("Axum 路由定义成功");
    println!("GET / -> hello");
    println!("GET /users -> users");
}
