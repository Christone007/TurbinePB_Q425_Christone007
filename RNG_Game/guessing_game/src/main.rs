use std::io;

fn main() {
    let mut guess: String = String::from("start");

    io::stdin().read_line(&mut guess);

    println!("The guess is {guess}");
}
