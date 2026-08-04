fn main() {
    let s1 = String::from("hello");
    
    // s1 的所有权移动到 s2：避免两个指针指向同一堆内存造成二次释放
    let s2 = s1;
    
    // 下面这行会编译错误，因为 s1 不再有效
    // println!("{}", s1);
    
    println!("s2: {}", s2);
    
    // 显式克隆：深拷贝堆数据，两个变量各自独立（有性能开销）
    let s3 = s2.clone();
    println!("s2: {}, s3: {}", s2, s3);
    
    // 基本类型实现 Copy，不会移动
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);
}
