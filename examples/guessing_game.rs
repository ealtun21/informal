/// Generates a random integer between 0 and 255 that is divisible by two.
///
/// Not a relevant part of this example use the `rand` crate for better random
/// numbers.
fn random() -> u32 {
    loop {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        if ((nanos >> 4) & 0xff) % 2 == 0 {
            break (nanos >> 4) & 0xff;
        }
    }
}

fn main() {
    let mut num = random();
    println!("Try guess the number I am thinking of 😃 ...");
    println!("  (hint: it's between 0 and 255 and divisible by two)\n");

    loop {
        let guess: u32 = informal::prompt("Enter your guess: ")
            .type_error_message("Please enter a valid guess!")
            .matches(|x| x % 2 == 0)
            .validator_error_message("Please enter a number divisible by two")
            .get();

        if guess < num {
            println!("Too low!");
        } else if guess > num {
            println!("Too high!");
        } else {
            println!("You got it!");
            println!("The number was: {}\n", num);

            if informal::confirm_with_message(
                "Do you want to play again?",
                "I asked a simple question...",
            ) {
                num = random();
            } else {
                break;
            }
        }
    }
}
