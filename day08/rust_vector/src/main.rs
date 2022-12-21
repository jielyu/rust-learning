fn main() {
    // 创建Vec
    let mut v: Vec<i32> = Vec::new();

    // 更新Vec
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // 读取数组中的元素
    println!("v[1]={}", v[1]);

    // 使用get方法访问Vec元素
    match v.get(2) {
        Some(value) => {
            println!("value is {}", value);
        }
        None => println!("no that element"),
    }

    // 遍历Vec
    let mut sum = 0;
    for i in v {
        sum += i;
    }
    println!("sum={}", sum);

    // 使用枚举类型在Vec中存储不同类型的值
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
