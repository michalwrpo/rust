fn main() {
    let n = 128;
    let x = fibonacci(n);

    println!("{n}th Fibonacci number is {x}");
}

fn fibonacci(n: u8) -> u128 {
        if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        }

    let mut a = 0;
    let mut b = 1;
    let mut index = 1;
    
    while index != n {
        let next = a + b;
        a = b;
        b = next;

        index += 1; 
    }

    b
}
