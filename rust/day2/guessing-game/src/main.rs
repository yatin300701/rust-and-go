use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;
fn main() {



    // remaining color in output

    println!("Welcome to Guess the Number");
   


    let secret_num = rand::thread_rng().gen_range(0..101);
    

    println!("Secret number generated is {} ", secret_num);

   

    loop {

        println!("Please type the number");
        let mut guess = String::new();

        

        io::stdin().read_line(&mut guess).expect("Faild to read line");
        
        let guess_number:u32 = match  guess.trim().parse() {
            Ok(num)=>num,
            Err(_) => continue,
        };

        match guess_number.cmp(&secret_num) {
            Ordering::Greater => println!("{}","Too high".red()),
            Ordering::Less => println!("{}","Too low".red()),
            Ordering::Equal => {
                println!("{}","You guessed correct".green());
                break;
            }
        }
    
    }
   
} 
