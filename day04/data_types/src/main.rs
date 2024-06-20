use std::io;


fn main() {
    invalid_access_test()
}


#[allow(dead_code)]
fn int_literals() {
    let decimal = 255;
    let hex = 0xFF;
    let octal = 0o377;
    let binary = 0b1111_1111;
    println!("{},{},{},{}", decimal, hex, octal, binary)
}


#[allow(dead_code)]
fn math_operation() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    let remainder = 43 % 5;
    println!("Add is {}; Subtraction is {}; Multiplication is {}; Division are {},{}; Remainder is {}", 
                sum, difference, 
                product, quotient, 
                truncated, 
                remainder
            );
}

#[allow(dead_code)]
fn tuple_type() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of y is: {y}, Not used: {x},{z}");
    println!("{}:{}:{}", five_hundred, six_point_four, one)
}


#[allow(dead_code)]
fn array_type() {
    let _a = [1, 2, 3, 4, 5];             // must be same type
    let _b: [i32; 5] = [6, 7, 8, 9, 10];
    let _c = [5; 5];                      // can also set value in square bracket, first is value or type second is elements
    
    let _a_first = _a[0];
    let _b_second = _b[1];
}

fn invalid_access_test() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()                         // input number
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index            // parse input index to number 
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}