use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // random number between 1 - 100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
    // .read_line(&mut guess),
    // handle to get input from the user.
    // &mut guess.
    // -> The & indicates that this argument is a reference,
    // -> which gives you a way to let multiple parts of your code access one piece of data
    // -> without needing to copy that data into memory multiple times.
        .read_line(&mut guess)
        // Kinda like .JS try catch -> handle potential failure
        .expect("Failed to read line");

        // We create a variable named guess.
        // But wait, doesn’t the program already have a variable named guess?
        // It does, but Rust allows us to shadow the previous value of guess with a new one. This feature is often used in situations in which you want to convert a value from one type to another type. Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess for example. (Chapter 3 covers shadowing in more detail.)
        // We bind guess to the expression guess.trim().parse(). The guess in the expression refers to the original guess that was a String with the input in it. The trim method on a String instance will eliminate any whitespace at the beginning and end. Although u32 can contain only numerical characters, the user must press enter to satisfy read_line. When the user presses enter, a newline character is added to the string. 
        // For example, if the user types 5 and presses enter, guess looks like this: 5\n. The \n represents “newline,” the result of pressing enter. The trim method eliminates \n, resulting in just 5.
        // The parse method on strings parses a string into some kind of number. Because this method can parse a variety of number types, we need to tell Rust the exact number type we want by using let guess: u32. The colon (:) after guess tells Rust we’ll annotate the variable’s type. Rust has a few built-in number types; the u32 seen here is an unsigned, 32-bit integer. It’s a good default choice for a small positive number. You’ll learn about other number types in Chapter 3. Additionally, the u32 annotation in this example program and the comparison with secret_number means that Rust will infer that secret_number should be a u32 as well. So now the comparison will be between two values of the same type!
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    // like JS switch statements but better
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("!!YOU WIN!!"),
    }
}