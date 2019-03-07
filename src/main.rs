use std::io; //brings in input/output lib
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number game! Classic!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); //creates a new istance of a string type

        io::stdin().read_line(&mut guess)       
        //& means reference to the memory
        //&mut makes the reference mutable
        //method returns Result
            .expect("Failed to read line");     
            //error handling with expect method
            //displays output if Result is Ok, error if Result is Err

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Uh-oh! Only integer numbers please!");
                continue; //_ is catchall
            }
        };

        println!("You guessssed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }   
}