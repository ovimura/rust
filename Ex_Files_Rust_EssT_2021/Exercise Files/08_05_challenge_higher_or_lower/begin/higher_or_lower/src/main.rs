use rand::Rng;
use std::io;

fn main() {
    let mut rn = rand::thread_rng();
    let n = rn.gen_range(1..101);
    let mut gn = String::new();
    let mut guess: i32 = -1;
    while guess != n {
        gn.clear();
        println!("Please guess the integer number:");
        io::stdin()
            .read_line(&mut gn)
            .expect("error: failed to read the file");
        guess = gn
            .trim()
            .parse()
            .expect("error: not a valid integer number");
        if guess > n {
            println!("info: guessed number is higher");
        } else if guess < n {
            println!("info: guessed number is lower");
        }
    }
    println!("Congrats! You guessed the random number: {}", n);
}
