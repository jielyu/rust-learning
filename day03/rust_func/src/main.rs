fn func(x: i32) -> i32 {
    if x < 0 {
        return -x; // 中间返回使用return
    }
    x // 不带分号的表达式就是返回值
}

fn main() {
    println!("func(-1)={}, func(1)={}", func(-1), func(1));
}
