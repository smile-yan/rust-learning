use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    println!("随机数: {}", rng.gen_range(1..100));
    println!("布尔: {}", rng.gen_bool(0.5));

    let chars: String = std::iter::repeat(|| {
        rng.sample(rand::distributions::Alphabetic)
    }).take(8).collect();
    println!("随机字符串: {}", chars);
}
