use std::io;

fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

fn main() {
    // There are no variadic arguments in Rust
    let mut numbers: Vec<i32> = Vec::new();
    let mut input_number = String::new();
    let mut collect_numbers = false;

    while !collect_numbers {
        println!("Please enter the number you want to get: ");
        io::stdin()
            .read_line(&mut input_number)
            .expect("TODO: panic message");
        if input_number.trim().is_empty() {
            collect_numbers = true;
        }
        let parsed = input_number.trim().parse::<i32>();

        if parsed.is_ok() {
            numbers.push(parsed.unwrap());
            input_number.clear();
        }
        else { collect_numbers = true }
    }

    let result = sum(&numbers);
    println!("The sum is {}", result);
}
