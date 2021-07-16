use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess!");
    let secret = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Enter number 0-100.");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("Fail to read line");
        println!("You guessed: {}", buf.trim());

        let buf: u32 = match buf.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match buf.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }
}
