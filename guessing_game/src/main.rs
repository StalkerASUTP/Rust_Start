use std::io;
fn main() {
    println!("Guess the number!");

    println!("Please input yout guess.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
            .expect("Failed to read line");
    println!("You guessed: {}", guess);//значение переменной помещается вместо заполнителя {} при печати
    println!("You guessed: {guess}");//значение переменной помещается вместо заполнителя {} при печати
}