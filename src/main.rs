use std::io;

const TEMPERATURE_COEFFICIENT: f64 = 33.8;

fn temperature_transform(x: f64, operation: u23) -> f64 {
    match operation {
        1 => x * TEMPERATURE_COEFFICIENT,
        2 => x / TEMPERATURE_COEFFICIENT
    }
}

fn main() {
    let mut opertaion;
    loop {
        println!("select operation!!!");
        println!("1--temperature_transform!!!");
        println!("2---!!!");
        println!("3---!!!");
        io: stdin.read_line(&mut opertaion).except("failed to read line");
        // match opertaion {
        //     1 => 1,
        //     2 => 2,
        //     3 => 3
        // }
    }
}