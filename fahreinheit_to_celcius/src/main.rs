fn main() {
    let temp1 = fahrenheit_to_celcius(80.0);
    let temp2 = celcius_to_fahrenheit(32.0);

    println!("{temp1}, {temp2}");
}

fn fahrenheit_to_celcius(temperature: f64) -> f64 {
    (temperature - 32.0) * 5.0 / 9.0
}

fn celcius_to_fahrenheit(temperature: f64) -> f64 {
    temperature * 1.8 + 32.0
}