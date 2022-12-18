fn main() {
    // 常量
    const PI: f64 = 3.141592654;

    // 常变量 (不能改变的变量)
    let a = 12;
    // a = 10; //会报错

    // 隐藏
    let a = 20 + a;

    // 变量 (可变变量)
    let mut x = 3;
    x = x + 10;
    println!("PI={}, a={}, x={}", PI, a, x);
}
