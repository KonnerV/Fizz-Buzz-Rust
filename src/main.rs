//Imports
use std::thread::{sleep};
use std::time;
use std::time::Duration;

//Main Class
fn main() {
    let mut counting_up: i32 = 0;

    //Count to 100 and check if the number is Fizz, Buzz or FizzBuzz
    while counting_up <= 99 {
        sleep(time::Duration::from_millis(500));
        counting_up = counting_up+1;

        if counting_up % 5 == 0 && counting_up % 3 == 0 {
            println!("FizzBuzz");
        }
        else if counting_up % 3 == 0 {
            println!("Fizz");
        }
        else if counting_up % 5 == 0 {
            println!("Buzz");
        }
        else {
            println!("{}", counting_up);
        }
    }
    std::mem::drop(counting_up);
}
