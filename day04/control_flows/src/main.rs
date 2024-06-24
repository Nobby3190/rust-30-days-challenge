#[allow(dead_code)]
fn single_condition(number: u32) {
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

#[allow(dead_code)]
fn multi_condition(number: u32) {
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

#[allow(dead_code)]
fn one_line_condition(number: u32) {
    let condition = true;
    let number = if condition {number} else {0}; // true = number, false = 0 (must be same type at both side)
    println!("The value of number is: {number}")
}

#[allow(dead_code)]
fn loop_condition(count_number: u32) {
    let mut counter = count_number;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}")
}

#[allow(dead_code)]
fn multi_loop_condition(count_number: u32, remain_number: u32) {
    let mut count = count_number;

    'counting_up: loop {
        println!("count is {count}");
        let mut remaining = remain_number;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

#[allow(dead_code)]
// using while usually with condition
fn while_condition() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }
}


// using for usually for iterables
fn for_condition() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is {}", element);
    }

    // in range(reversed)
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}



fn main() {
    // single_condition(3);
    // multi_condition(3);
    // one_line_condition(3);
    // loop_condition(0);
    // multi_loop_condition(0, 10);
    for_condition()
}
