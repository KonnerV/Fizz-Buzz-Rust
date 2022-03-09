use std::thread::{sleep};

fn fizz_buzz() {
    //Variables
    let mut counting_up: i32 = 0;

    while counting_up <= 99 {
        //This make the program sleep for four hundred milliseconds each loop
        sleep(std::time::Duration::from_millis(400));
        counting_up = counting_up+1;
        //Checks if the number is is a multiple of five and three and if it is it will print "FizzBuzz"
        if counting_up % 5 == 0 && counting_up % 3 == 0 {
            println!("FizzBuzz");
        }
        //Checks if the number is a multiple of three and if it is then it prints "Fizz"
        else if counting_up % 3 == 0 {
            println!("Fizz");
        }
        //Checks if the number is a multiple of five and if it is then it prints "Buzz"
        else if counting_up % 5 == 0 {
            println!("Buzz");
        }
        //Prints the regular number if they are not a multiple of five and three or not a multiple of five or three
        else {
            println!("{}", counting_up);
        }
    }
    //Drops the variables
    std::mem::drop(counting_up);
}
fn main() {
    fizz_buzz();
}
