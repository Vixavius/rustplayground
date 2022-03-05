// Global scope constant
const X_POINTS: u32 = 543;

fn main() {
    variables();
    control_flow();
}

fn control_flow() {
    let number = 3;

    // If _expression_
    if number < 5 {
        println!("less than condition");
    } else {
        println!("greater than condition");
    }

    // Must be boolean condition, "if number" would not be accepted
    if number != 0 {
        println!("Number is non-zero");
    }

    // Because if is an expression, it can be used on right side of let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number); // 5

    // The value of code blocks evaluate to the last expression in them

    let mut counter = 0;

    // Loop expression
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    }; // end of let statement, assigns the value to result
    println!("The result is {}", result);

    // While loop
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("While loop complete.");

    // For loop - iteration
    let a = [11, 22, 33, 44, 55, 66];

    for element in a.iter() {
        println!("The value is: {}", element);
    }

    // Ranges -- 4! 3! 2! 1!
    for number in (1..=4).rev() {
        println!("{}!", number);
    }
}

fn variables() {
    // Type is reuqired here. Function scope constant.
    const MAX_POINTS: u32 = 100_000; // Numeric literals

    // Immutable variable
    let y = 4;
    println!("The value of y is: {}", y);

    // But we can shadow it
    let y = 3;
    println!("The value of y is: {}", y);

    // And use a different type. Can avoid ex. spaces_str vs. spaces_num
    let spaces = "        ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    // Type is inferred here
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    println!("X Points is: {}", X_POINTS);
    println!("Max Points is: {}", MAX_POINTS);

    let dec = 98_222; // 98222
    let hex = 0xff; // 255
    let oct = 0o77; // 63
    let bin = 0b1111_0000; // 240
    let byt = b'A'; // 65

    println!("Dec: {}", dec);
    println!("Hex: {}", hex);
    println!("Oct: {}", oct);
    println!("Bin: {}", bin);
    println!("Byt: {}", byt);

    // f64 roughly same speed as f32
    // IEEE-754
    let x = 2.0;
    let y: f32 = 3.0;

    println!("x: {}, y: {}", x, y);

    // char - 4 byte Unicode Scaler value
    let c = 'z';
    let z = '▀';

    println!("{} {}", c, z);

    // Tuple - different types allowed
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // Destructuring pattern matching

    println!("The value of x, y, z is: {}, {}, {}", x, y, z);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0; // Element access
    let six_point_four = x.1;
    let one = x.2;

    println!("{} {} {}", five_hundred, six_point_four, one);

    // Stack array - same type
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    // let bad = a[5]; - will not compile
    println!("First: {}", first);

    let a = [3; 5]; // [3, 3, 3, 3, 3]
    println!("{}", a[0]);

    // Type annotations are not needed because they're declared in the function
    let y = 4;
    another_function(y, 5);

    // let x = (let y = 6); // Let statement does not return a value

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn plus_one(x: i32) -> i32 {
    x + 1 // Lonely expression - no semicolon needed
}
