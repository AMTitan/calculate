use std::env;
use std::io::{Write, self};

fn main() {
    let can_calculate = ["pi"];
    match env::args().nth(1).unwrap().to_lowercase().as_str() {
        "pi" => {
            let mut pi:f64 = 0.0;
            let mut item:f64 = 1.0;
            loop {
                if (item-1.0)%4.0 == 0.0 {
                    pi+=4.0/item;
                }
                else {
                    pi-=4.0/item;
                }
                item+=2.0;
                print!("\rinteration: {} pi: {}", (item+1.0)/2.0, pi);
                io::stdout().flush().ok().expect("Could not flush stdout");
            }
        },
        "help" => {
            println!("This is all the different things I can calculate:");
            for i in can_calculate {
                println!("     {}", i);
            }
        },
        _ => {
            println!("I dont know how to calculate this, to see everything I can do, type help after the exicutiable");
        }
    }
}
