use std::io;

fn main() {
    println!("Enter any number ");

    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read input");
    let number: i32 = number.trim().parse().expect("invalid input");
    let mut i = 1;
    while i <= number {
        if (i % 3) == 0 && (i % 5) == 0 {
            println!("FuzzBuzz");
        } else if (i % 3) == 0 {
            println!("Fuzz");
        } else if (i % 5) == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
        i += 1;
    }
}
