use std::io;

fn main() {
    println!("Hi, welcome to  guess game.!");
    println!("Please type your favorite number: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
