fn main() {
    // 栈上的数据：大小固定，分配和释放都很快
    let x = 5;
    let y = true;
    let arr: [i32; 3] = [1, 2, 3];
    println!("栈上的数据: {}, {}, {:?}", x, y, arr);
    
    // 堆上的数据：指针（地址）在栈上，实际内容在堆上，
    // 由所有者负责释放
    let s = String::from("hello");
    let v = vec![1, 2, 3];
    // Box 是最简单的堆分配：把值显式装箱到堆上
    let b = Box::new(42);
    
    println!("堆上的 String: {}", s);
    println!("堆上的 Vec: {:?}", v);
    println!("堆上的 Box: {}", b);
    
    // 函数调用会在栈上创建新的栈帧
    print_size();
}

fn print_size() {
    let local = 10;
    println!("函数内的栈变量: {}", local);
}
