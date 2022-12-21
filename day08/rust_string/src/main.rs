use std::fmt::format;
fn main() {
    // 创建新的字符串
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();

    // 更新字符串
    let mut s = String::from("hello");
    s.push_str(" world"); // 追加字符串
    s.push('!'); // 追加字符
    println!("s={}", s);

    // format字符串
    let s2 = String::from("hello rust");
    let s3 = format!("{}, {}", s, s2);
    println!("s3={}", s3);

    // 字符串不支持索引
    // 字符串切片，必须小心切片范围不在字符边界导致崩溃
    let s = &s[0..5];
    println!("s={}", s);

    // 遍历字符串
    let s = String::from("你好");
    for c in s.chars() {
        println!("c={}", c);
    }
    for b in s.bytes() {
        println!("b={}", b);
    }
}
