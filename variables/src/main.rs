use std::io;

fn main() {
    // 可变变量
    let mut x= 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // 常量
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);

    // 隐藏
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x in the outer scope is: {}", x);

    // 转换时需要提供明确的类型注解
    let guess: u32 = "42".parse().expect("Not a number!");

    // 标量类型
    // 整型、浮点型、布尔类型、字符类型
    // 整型
    let a = 98;
    let a = 0x1f;
    let a = 0o77;
    let a = 0b1111_0000;
    let a = b'A';

    // 浮点型
    let a = 2.0; // f64
    let a: f32 = 3.0; // f32

    let a = 5 / 2;
    println!("The value of a is: {}", a);

    // 布尔型
    let a = true;
    let _a: bool = false;

    // 字符类型
    let a = 'z';
    let a = 'ℤ';
    let a = '😻';
    let a :&str = "z";

    // 复合类型
    // 元组类型
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x :(i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // 数组类型
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {} is: {}", index, element);

    another_function();
}

// 函数
fn another_function() {
    println!("Another function.");
}
