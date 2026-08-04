use std::os::raw::{c_int, c_char};

// 声明 C 标准库函数
extern "C" {
    fn abs(input: c_int) -> c_int;
    fn strlen(s: *const c_char) -> usize;
}

// 导出 Rust 函数供 C 调用
// #[no_mangle] 禁止符号名修饰，extern "C" 使用 C 的调用约定
#[no_mangle]
pub extern "C" fn rust_add(a: c_int, b: c_int) -> c_int {
    a + b
}

fn main() {
    // 调用 C 函数必须在 unsafe 块中：编译器无法检查外部函数的安全性
    unsafe {
        let result = abs(-42);
        println!("C abs(-42) = {}", result);

        // CString 保证内容以 \0 结尾，
        // as_ptr() 得到可传给 C 的裸指针
        let s = std::ffi::CString::new("Hello").unwrap();
        let len = strlen(s.as_ptr());
        println!("字符串长度: {}", len);
    }

    // 导出的 extern "C" 函数在 Rust 内部也可以像普通函数一样调用
    println!("rust_add(3, 5) = {}", rust_add(3, 5));
}
