use std::io;
fn main() {
    println!("Enter a number");
    let mut guess = String::new(); //mutable
    io::stdin().read_line(&mut guess).expect("Failed to read the line"); //&mut the reference is mutable
    println!("You entered {}", guess);
}
