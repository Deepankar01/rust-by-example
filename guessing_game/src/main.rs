use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);
    loop {
        let mut user_entered_number = String::new(); //mutable
        println!("Enter a number");
        io::stdin()
            .read_line(&mut user_entered_number)
            .expect("Failed to read the line"); //&mut the reference is mutable
        let user_entered_number: u32 = user_entered_number
            .trim()
            .parse()
            .expect("Failed to parse string to number");

        //comparison method no clue till now how match works
        match user_entered_number.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("They are equal!");
                break;
            }
        }
    }
}
