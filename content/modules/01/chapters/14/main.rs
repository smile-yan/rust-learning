use std::collections::{HashMap, HashSet, BTreeMap};

fn main() {
    // HashMap：词频统计
    let text = "hello rust hello world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        *map.entry(word).or_insert(0) += 1;
    }
    println!("词频: {:?}", map);

    // HashSet：去重
    let nums = vec![1, 2, 2, 3, 3, 3];
    let unique: HashSet<_> = nums.iter().collect();
    println!("去重: {:?}", unique);

    // BTreeMap：按键排序
    let mut tree = BTreeMap::new();
    tree.insert("zebra", 3);
    tree.insert("apple", 1);
    tree.insert("mango", 2);
    println!("有序: {:?}", tree);
}
