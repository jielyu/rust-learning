fn main() {
    println!("f2c(100)={}", f2c(32f32));
    println!("c2f(100)={}", c2f(100f32));
    fib(2);
    fib(3);
    fib(10);
}

// 华氏度转摄氏度
fn f2c(f: f32) -> f32 {
    5.0 / 9.0 * (f - 32.0)
}

// 摄氏度转华氏度
fn c2f(c: f32) -> f32 {
    9.0 / 5.0 * c + 32.0
}

// 生成斐波那契数列
fn fib(n: u32) {
    if n < 1 {
        println!("n must greater than 0, but n={}", n);
        return ();
    }
    if n == 1 {
        println!("1");
        return ();
    }

    let mut n_1 = 1;
    let mut n_2 = 1;
    let mut i = 2;
    print!("1, 1");
    while i < n {
        let n_c = n_1 + n_2;
        n_2 = n_1;
        n_1 = n_c;
        i += 1;
        print!(", {}", n_c);
    }
    print!("\r\n");
}
