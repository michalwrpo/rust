fn main() {
    let number = 2;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    // this feels illegal
    let x = if number < 5 { 5 } else { 10 };

    println!("The value of x is {x}");

}
