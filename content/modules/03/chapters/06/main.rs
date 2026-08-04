// 安全包装层：内部使用 unsafe，但对外提供安全接口
fn split_at_mut(
    slice: &mut [i32],
    mid: usize,
) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    assert!(mid <= len, "mid 不能超过切片长度");

    // 返回两个不重叠的可变切片
    // 安全代码无法同时借出两个可变切片，
    // 这个安全抽象只能借助裸指针实现
    let ptr = slice.as_mut_ptr();
    // unsafe 块圈出编译器无法验证的操作；
    // 上面的 assert 保证两个切片不重叠
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut numbers = [1, 2, 3, 4, 5];
    // 调用方完全处于安全代码中：unsafe 被封装在函数内部
    let (left, right) = split_at_mut(&mut numbers, 2);

    left[0] = 10;
    right[0] = 20;

    println!("left: {:?}", left);
    println!("right: {:?}", right);
    println!("原始数组: {:?}", numbers);
}
