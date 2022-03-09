//Some imported libraries that would be required later on in the program
use std::thread::{sleep};
use std::time;
use std::time::Duration;

//Creates the function named fizz_buzz
fn fizz_buzz() {

    //Declares some variables that would be needed later on
    let mut counting_up: i32 = 0;
    let five_hundred_millis: Duration = time::Duration::from_millis(500);

    //This while loop counts up to 100
    while counting_up <= 99 {
        //This make the program sleep for five hundred milliseconds each loop
        sleep(five_hundred_millis);
        //Increments the variable "counting_up" by 1
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
    std::mem::drop(five_hundred_millis);
}



fn main() {
    //Runs the function fizz_buzz created above the main function
    fizz_buzz();
}
