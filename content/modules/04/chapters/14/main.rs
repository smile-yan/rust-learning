// RAII：资源获取即初始化，把资源的生命周期绑定到对象上
struct FileGuard {
    name: String,
}

impl FileGuard {
    fn new(name: &str) -> FileGuard {
        // 构造即"打开"资源
        println!("打开文件: {}", name);
        FileGuard {
            name: name.to_string(),
        }
    }
}

// 实现 Drop：值离开作用域时自动执行清理，无需手动调用
impl Drop for FileGuard {
    fn drop(&mut self) {
        println!("关闭文件: {}", self.name);
    }
}

fn main() {
    {
        let file = FileGuard::new("data.txt");
        println!("使用文件: {}", file.name);
    } // file 离开作用域，drop 被调用
    
    println!("文件已自动关闭");
    
    // 也可以显式 drop
    let file2 = FileGuard::new("temp.txt");
    // std::mem::drop 立即释放资源，而不是等作用域结束
    drop(file2);
    println!("temp.txt 已提前关闭");
}
