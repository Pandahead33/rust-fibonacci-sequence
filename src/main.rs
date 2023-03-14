use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        print!("Enter length of Fibonacci sequence to generate: ");
        stdout().flush().unwrap();

        let mut length_of_fib_sequence = String::new();
        stdin()
            .read_line(&mut length_of_fib_sequence)
            .expect("Failed to read line");

        if length_of_fib_sequence.trim() == "quit" || length_of_fib_sequence.trim() == "exit" {
            println!("Thanks for using Lucas's Fibonacci sequence generator!");
            break
        }

        let length_of_fib_sequence: u8 = match length_of_fib_sequence.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        generate_fibonacci_sequence(length_of_fib_sequence);
        println!();
    }
}

fn generate_next_fibonacci_number(current_fib: (i64, i64)) -> (i64, i64) {
    let (last_fib_number, number_before_last_fib_number) = current_fib;
    let next_fib_number = last_fib_number + number_before_last_fib_number;

    (next_fib_number, last_fib_number)
}

const DEFAULT_FIBONACCI_STARTING_NUMBERS: (i64, i64) = (0, 1);

fn generate_fibonacci_sequence(num_to_generate: u8) {
    let mut last_two_fib_nums = DEFAULT_FIBONACCI_STARTING_NUMBERS;

    println!("Printing the first {num_to_generate} numbers of the Fibonacci sequence.");
    println!("{}", last_two_fib_nums.0);

    for _number in 1..num_to_generate {
        last_two_fib_nums = generate_next_fibonacci_number(last_two_fib_nums);
        println!("{}", last_two_fib_nums.0);
    }
}
