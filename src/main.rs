use std::io;
fn main() {
    println!("Please input the letters you wanna change x -> y ?!");
    let mut user_input:String = String::new();
    io::stdin().read_line(&mut user_input).expect("Err reading your input");
    let letters = letters_to_exchange(&user_input);
    
    println!("Please write your sentence ?!");
    let mut user_input:String = String::new();
    io::stdin().read_line(&mut user_input).expect("Err reading your input");

    let modified_text = replace(&user_input, letters);

    println!("Modified text is: \n {}",modified_text);

}

fn letters_to_exchange(user_input:&String) -> (char,char)
{
    (
        user_input.chars().take(1).last().unwrap(),
        user_input.chars().take(3).last().unwrap()
    )
}

fn replace(user_input:&String,letters:(char,char))-> String
{
    let mut modified_text:String = String::with_capacity(user_input.len());
    for mut letter in user_input.chars()
    {
        if letter == letters.0 {
           letter = letters.1
        }
        modified_text.push(letter);
    }

    modified_text
}
