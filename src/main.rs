use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;


fn main() { 
    println!("Guess a number");

    let secret_number : u32 = rand::thread_rng().gen_range(1..100);

    println!("The secret number is {}", secret_number);

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("enter line");

    loop{

        let guess : u32 = match guess.trim().parse() {
            Ok(num)=>  num,
            Err(_) => continue,
    
    
    
        };



        match guess.cmp(&secret_number) {

            Ordering::Equal => {
                println!("{}", "Equal".color("green"));
                break;
            } ,
            Ordering::Less => println!("Less"),
            Ordering::Greater => println!("Greater"),

        }

        break;



    }


}



