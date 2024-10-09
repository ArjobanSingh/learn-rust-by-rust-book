use std::io;

const TEMP_F: &str = "Fahrenheit";
const TEMP_C: &str = "Celsius";

fn main() {
    gen_n_th_fib();
    // convert_temperature();
}

fn gen_n_th_fib() {
    println!("Please type a positive number: ");
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read a line");

    let num: u128 = num
        .trim()
        .parse()
        .expect("Please type a valid positive number");
    
    let ans = if num == 0 {
        0
    } else if num == 1 {
        1
    } else {
        let mut old_prev: u128 = 0;
        let mut prev: u128 = 1;
        for _n in 2..=num {
            let sum = old_prev + prev;
            old_prev = prev;
            prev = sum;
        }
        prev
    };

    println!("The fibonacci no. at position {num} is: {ans}");
}

fn convert_temperature() {
    println!("What is your temp type. type 0 for {TEMP_F} and type 1 for {TEMP_C}");
    let mut temp_type = String::new();

    io::stdin()
        .read_line(&mut temp_type)
        .expect("Failed to read a line");
    
    let temp_type: u8 = temp_type
        .trim()
        .parse()
        .expect("Please type a number");

    if temp_type != 0 && temp_type != 1 {
        println!("Invalid type option");
        return;
    }

    println!("Type current temperature value");
    let mut temp_value = String::new();
    io::stdin()
        .read_line(&mut temp_value)
        .expect("Failed to read a temp value");

    let temp_value: f64 = temp_value
        .trim()
        .parse()
        .expect("Enter a valid temperature range");

    let result = if temp_type == 0 {
        convert_farh_to_cel(temp_value)
    } else {
        convert_cel_to_farh(temp_value)
    };

    let opposite_temp_type = if temp_type == 0 { TEMP_C } else { TEMP_F };
    let temp_type = if temp_type == 0 { TEMP_F } else { TEMP_C };

    println!("{temp_value} {temp_type} is equal to {result} {opposite_temp_type}");

}

fn convert_farh_to_cel(farh: f64) -> f64 {
    (farh - 32.0) * 5.0/9.0
}

fn convert_cel_to_farh(cel: f64) -> f64 {
    (cel * 9.0/5.0) + 32.0
}