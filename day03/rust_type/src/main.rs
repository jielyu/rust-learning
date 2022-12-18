fn main() {
    // 整数类型
    let p8: u8 = 255;
    let n8: i8 = -127;
    let p16: u16 = 65535;
    let n16: i16 = -32767;
    let p32: u32 = std::u32::MAX;
    let n32: i32 = std::i32::MIN;
    let p64: u64 = std::u64::MAX;
    let n64: i64 = std::i64::MIN;
    let a: isize = std::isize::MAX;

    println!(
        "p8={}, \r\nn8={}, \r\np16={}, \r\nn16={}, \r\np32={}, \r\nn32={}, \r\np64={}, \r\nn64={}, \r\na={}",
        p8, n8, p16, n16, p32, n32, p64, n64, a
    );

    // 浮点数
    let x32: f32 = 3.14;
    let x64: f64 = std::f64::MAX;
    println!("x32={}", x32);
    println!("x64={}", x64);

    // 布尔类型
    let b: bool = false;
    println!("b={}", b);

    // 字符类型
    let c = 'a';
    println!("c={}", c);

    // 元组类型
    let t: (i32, f64, u8) = (500, 6.4, 1); // 创建元组
    let (x, y, z) = t; // 解构元组
    println!("t={:#?}, x={}, y={}, z={}", t, x, y, z);

    // 数组类型
    let arr = [1, 2, 3, 4, 5];
    println!("arr[0]={}, arr[4]={}", arr[0], arr[4]);
}
