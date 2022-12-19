fn main() {
    // 测试String
    let s = String::from("hello world");
    let first = first_word(&s[..]);
    println!("String first word: {}", first);

    // 测试字面值
    let s1 = "hello world";
    let first = first_word(s1);
    println!("first word: {}", first);
}

fn first_word(s: &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
