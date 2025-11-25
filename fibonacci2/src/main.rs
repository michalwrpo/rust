use std::io;

fn fibonacci(n : u8) -> u128 {
    if n == 0 {
        return 0;
    }

    let mut f_prev = 0;
    let mut f = 1;

    for _ in 2..=n {
        let tmp = f;
        f += f_prev;
        f_prev = tmp;
    }

    f
}

fn main() {
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read a line.");

    let n : u8 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not a natural number.");
            return;
        }
    };

    println!("Fibonacci({}) = {}", n, fibonacci(n));
}
