use std::env;

fn main() {
    let can_calculate = ["pi"];
    match env::args().nth(1).unwrap().to_lowercase().as_str() {
        "pi" => {
            
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
