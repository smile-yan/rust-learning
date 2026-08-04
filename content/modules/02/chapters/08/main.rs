// 这是一个 wasm-bindgen 风格示例，需要在 Cargo.toml 中添加依赖
// [dependencies]
// wasm-bindgen = "0.2"

// 下面是真实项目中的导出写法（此处注释掉，使示例无需依赖即可运行）
// use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// pub fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

fn main() {
    println!("WASM 示例代码");
    println!("在浏览器中，这段 Rust 代码可以近乎原生速度运行。");
    
    // 模拟 WASM 导出的函数逻辑
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    println!("add(2, 3) = {}", add(2, 3));
    println!("使用 wasm-pack build 可以生成可在浏览器中加载的 .wasm 文件。");
}
