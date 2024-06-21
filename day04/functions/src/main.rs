fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');
    five_return();
    plus_one_return();
}


fn another_function(x: i32) {
    println!("The value of x: {}", x);
}


fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}


fn five() -> i32 {
    5                               // 結尾不加;才能return
}
fn five_return() {
    let x = five();                 // 等於 let x = 5;
    println!("The value of x is: {}", x);
}
fn plus_one(x: i32) -> i32 {
    x + 1
}
fn plus_one_return() {
    let x = plus_one(5);
    println!("The value of x is: {x}");
}