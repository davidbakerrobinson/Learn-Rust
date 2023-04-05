fn main() {
    let condition = true;
    let number = if condition { 5 } else if false { 10 } else  { 6 };

    println!("The value of number is: {}", number);
}