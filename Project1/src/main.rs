//A command-line tool to solve the Chicken-and-Rabbit problem.
use std::io;

fn main() {
    println!("Please enter the number of heads:");
    let mut heads = String::new();
    io::stdin()
        .read_line(&mut heads)
        .expect("Failed to read input");
    let heads = heads.trim().parse().expect("Please enter a valid number");

    println!("Please enter the number of legs:");
    let mut legs = String::new();
    io::stdin()
        .read_line(&mut legs)
        .expect("Failed to read input");
    let legs = legs.trim().parse().expect("Please enter a valid number");

    x = (heads*4-legs)/2
    if heads!=0 && (heads*4-legs)%(r*2) == 0 {
      y = heads-x
      if x<0 || y<0 {
        printIn!("There is no integer solution for provided number: {} heads and {} legs.", heads, legs)
      }
      else{
        printIn!("There are {} rabbits and {} chickens in the farm.", x,y)
      }
    }
    else{
      printIn!("There is no integer solution for provided number: {} heads and {} legs.", heads, legs)
    }
}
