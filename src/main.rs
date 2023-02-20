use std::io;

fn main() {
    loop {
        let mut operation = String::new();
        println!("select operation!!!");
        println!("1--温度转换!!!");
        println!("2--斐波那契数列!!!");
        println!("3---!!!");
        io::stdin().read_line(&mut operation)
            .expect("failed to read line");
        let operation: u32 = match operation.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("input operation error");
                0
            }
        };
        match operation {
            1 => temperature_transform(),
            2 => fbnq(),
            3 => println!("operation error!!!"),
            _ => {
                println!("operation error and break!!!");
                break;
            }
        }
    }
}


fn temperature_transform() {
    const TEMPERATURE_COEFFICIENT: f64 = 33.8;

    let mut operation2 = String::new();
    let mut itemperature = String::new();

    println!("input operation");
    println!("1--x*33.8");
    println!("2--x/33.8");
    io::stdin().read_line(&mut operation2)
        .expect("input_error");

    println!("input itemperature");
    io::stdin().read_line(&mut itemperature)
        .expect("input_error");

    let mut operation1: i32 = match operation2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("input operation error");
            0
        }
    };
    let itemperature: f64 = match itemperature.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("input operation error");
            0.0
        }
    };
    match operation1 {
        1 => {
            let y: f64 = itemperature * TEMPERATURE_COEFFICIENT;
            println!("tx:{}and tr:{}", itemperature, y)
        }
        2 => {
            let y: f64 = itemperature / TEMPERATURE_COEFFICIENT;
            println!("tx:{}and tr:{}", itemperature, y)
        }
        _ => println!("operation error!!!")
    }
}


fn fbnq() {
    println!("input length:");
    let mut operation2 = String::new();
    io::stdin().read_line(&mut operation2)
        .expect("input_error");

    let mut operation2: i64 = match operation2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("input operation error");
            0
        }
    };
    println!("fbnq num:");
    for i in 0..operation2 {
        print!("{},", fbnq2(i))
    };
    println!("fbnq over");
}


fn fbnq2(x: i64) -> i64 {
    if x == 0 {
        x
    } else if x == 1 {
        x
    } else {
        fbnq2(x - 1) + fbnq2(x - 2)
    }
}