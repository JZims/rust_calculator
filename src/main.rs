use std::io::{stdin, stdout, Write};

fn read(input: &mut String){
    stdout().flush()
        .expect("Failed to flush.");
    stdin().read_line(input)
        .expect("Failed to read.");
}


fn main() {
    println!("Welcome to the Rust calculator app!");
    println!("------------");

//Can't read input from the terminal as an integer. 
// Must be converted to integer later on.
    loop { 
        let mut num1 = String::new();
        let mut num2 = String::new();
        let mut operator = String::new();

        print!("What is the first number?:  ");
        read(&mut num1);

        print!("What is the second number?:  ");
        read(&mut num2);

        print!("What is the operator? [*, +, -, /]:  ");
        read(&mut operator);

        // println!("{} {} {} = ", num1, operator, num2);

        let num1: f32 = num1.trim().parse().unwrap();
        let num2: f32 = num2.trim().parse().unwrap();
        let operator: char = operator.trim().chars().next().unwrap();

        let operators = String::from("+-*/");

        if !operators.contains(operator) {
            println!("Unknown Operator");
            continue;
        }

        let result = match operator {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '*' => num1 * num2,
            '/' => num1 / num2,
            _=> panic!("Error in operator")
        };

        println!("The result of {} {} {} = {}", num1, operator, num2, result);
    }
    
}
