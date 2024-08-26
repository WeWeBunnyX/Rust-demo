use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;


fn main() {
    println!("Hey , lets guess a number");

    let secret_number = rand::thread_rng().gen_range(0..100);

    println!("The secret number is {}", secret_number);

    loop{

        println!("Input your guess");
        let mut guess: String =  String::new();
        io::stdin().read_line(&mut guess).expect("Error reading line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num)=> num,
            Err(_)=> continue,
            


        } ;
    
        match guess.cmp(&secret_number) {
    
            Ordering::Equal => {
                println!("{}", "Equal".green());
                break;
            },
            Ordering::Greater => println!("{}","Greater".red()),
            Ordering::Less => println!("{}", "Less".red()),
    
    
    
        }

    }


}



//demo