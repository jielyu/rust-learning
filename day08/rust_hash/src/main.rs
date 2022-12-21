use std::collections::HashMap;
fn main() {
    // 创建HashMap实例
    let mut scores = HashMap::new();

    // 添加元素
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores={:#?}", scores);

    // 访问HashMap中的值
    println!("scores['Blue']={}", scores["Blue"]);

    // 遍历HashMap
    for (key, value) in &scores {
        println!("key={}, value={}", key, value);
    }

    // 更新HashMap
    scores.insert(String::from("Blue"), 25);
    println!("scores['Blue']={}", scores["Blue"]);
}
