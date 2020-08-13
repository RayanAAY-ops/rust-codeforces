use std::io;
use std::process;

fn main() {
    let mut guess = String::new();

    io::stdin()
        .read_line( &mut guess)
        .expect("please enter valid");
    let  guess: i8 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => process::exit(1),
    };


    if guess >=1 {


        if guess <=100 {
            for _i in 0..guess{
                let mut word = String::new();
                io::stdin()
                    .read_line(&mut word)
                    .expect("please enter valid");

                let word = word.trim();

                if word.len()  < 11 {
                    println!("{}",(word);
                }
                else {
                    if word.len() <= 100 {
                    let word_len = word.len() -2; // we substract the first and last element + \n of the vector
                    let length_word: Vec<char> = word_len.to_string().chars().rev().take(3).collect(); // convert the length number into a vector of characters

                    let mut first_char  = word.to_string().chars().next().unwrap().to_string(); //extract the first character

                    let last_char: Vec<char> = word.to_string().chars().rev().take(3).collect(); // extract the last character



                    if word_len >= 10 {

                        first_char.push(length_word[1]); // pushing to the first character which is a string now
                        // you can't push to a character
                        first_char.push(length_word[0]);

                        first_char.push(last_char[0]);

                        println!("{}", first_char);

                    }
                    else {

                        first_char.push(length_word[0]);

                        first_char.push(last_char[0]);

                        println!("{}", first_char);
                    }
                }
                    else {
                        process::exit(1);

                    }
                }
            }
        }

        else {
            process::exit(1);

        }
    }else {
        process::exit(1);

    }
}


