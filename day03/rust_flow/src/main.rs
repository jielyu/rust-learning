fn main() {
    // if条件
    let num = 3;
    if num < 5 {
        println!("条件为真");
    } else {
        println!("条件为假");
    }

    // let ... if ...
    let b = if num < 5 { true } else { false };
    println!("b={}", b);

    // loop循环
    let mut i = 0;
    loop {
        i += 1;
        if i > 10 {
            break;
        }
    }
    println!("loop, i={}", i);

    // while循环
    let mut i = 0;
    while i < 10 {
        i += 1;
    }
    println!("while, i={}", i);

    // for循环
    let arr = [1, 2, 3, 4, 5];
    let mut sum = 0;
    for v in arr {
        println!("for, v={}", v);
        sum += v;
    }
    println!("for, sum={}", sum);
}
