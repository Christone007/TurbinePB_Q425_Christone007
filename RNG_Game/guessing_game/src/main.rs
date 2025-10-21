use rand;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number: i8 = rand::random_range(0..=100);
    let mut player_name = String::new();
    let mut no_of_attempts: i8 = 8;
    let mut detonated: bool = false;

    println!("Welcome to the Game, Soldier!\nWhat is your name?");
    io::stdin()
        .read_line(&mut player_name)
        .expect("Failed to read line");

    println!(
        "\nMISSION MANIFEST:
    You are a member of the special anti-bomb squad, {player_name}
    A bomb has been planted at the Mando Dam by terrorist elements.
    If this bomb goes off, entire communities would be flooded!

    Your mission is to DETONATE THE BOMB!
    The bomb is locked with a number between 0 and 100.
    You must guess the correct code {player_name}

    YOU HAVE ONLY 10 attempts
    The world looks up to you!\n\n\n"
    );

    while no_of_attempts > 0 {
        let mut guess: String = String::new();

        println!("ENTER THE CORRECT CODE_ ({no_of_attempts} attempts left)");
        no_of_attempts -= 1;

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read input");

        let guess: i8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Wrong input: The code must be a number between 0 and 100");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("❌ {guess} is too small!, Try a higher number!"),
            Ordering::Greater => println!("❌ {guess} is too big, try a smaller number"),
            Ordering::Equal => {
                println!(
                    "✅ CODE MATCHED
                BOMB DETONATED SUCCESSFULLY!\n\n"
                );
                detonated = true;
                break;
            }
        }
    }

    if detonated {
        println!(
            "🟢🟢🟢 MISSION ACCOMPLISHED! 🟢🟢🟢

            Congratulations Soldier! You have saved entire communities and livelihoods.

        WELL DONE!"
        );
    } else {
        println!("🔴🔴🔴 MISSION FAILD 🔴🔴🔴\n\n");
        println!(
            "BREAKING NEWS:
        💥💥 BOMB EXPLOSION 💥💥 rocks Mando Dam
        
        Special Anti-momb squad team wiped out!
        13 communities flooded!
        1,330 people feared dead mostly women and children!
        over 2,000 hospitalized and hundreds still missing!"
        );
    }
}
