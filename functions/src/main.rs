fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labeled_measurement(5, 'h');

    let five = five();
    println!("The value of five is: {}", five);

    let x = x();
    println!("The value of x is: {}", x);

    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    the_loop();

    while_loop();

    for_loop();

    for_loop_with_range();
}

fn another_function(x: i32) {
    println!("Another function.");
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, label: char) {
    println!("The measurement is {} {}", value, label);
}

fn five() -> i32 {
    5
}

fn x() -> i32 {
    let x = {
        let y = 3;
        y + 1
    };
    return x;
}

fn the_loop() {
    let mut counter = 0;
    loop {
        counter += 1;
        println!("again!");
        if counter == 10 {
            break;
        }
    }
}

fn while_loop() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }
}

fn for_loop_with_range() {
    for number in(1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");
}
