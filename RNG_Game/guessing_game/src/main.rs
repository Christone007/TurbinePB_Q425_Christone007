use std::io;
use rand::Rng;

fn main() {
    let mut guess: String = String::from("");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    

    println!("");

    let res = io::stdin().read_line(&mut guess);

    match res {
        Err(a) => println!("{a}"),
        Ok(a) => println!("{a}"),
    }

    println!("The guess is {guess}");
}
