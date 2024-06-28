fn main() {
    test_case_one();
    test_case_two();
}

//
fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn test_case_one() {
    let s = String::from("hello");
    takes_ownership(s); 
    // s value invalid here

    let x = 5;
    makes_copy(x);
    // x has copied, still valid here
    println!("{x} still valid")
}

//
fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn test_case_two() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);
    println!("{s1}, {s3}")
}




#[allow(dead_code)]
fn scope_case() {
    let s = String::from("world");
    // scope
    {
        let s = String::from("hello");
        println!("{s}");
    }
    // scope end
    println!("{s}");
}

#[allow(dead_code)]
fn deep_copy_case(){
    let s1 = String::from("hello!");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}")
}