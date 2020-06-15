use std::io;

fn main() {

    // println!("{}", 5.0 / 9.0)

    println!("Enter the temperature in farenheit:");

    let mut farenheit = String::new();

    io::stdin()
        .read_line(&mut farenheit)
        .expect("Please enter a valid temperature");

    let farenheit: f64 = farenheit.trim().parse().expect("Enter a valid number");

    let celcius = (farenheit - 32.0) * (5.0 / 9.0);

    println!("Your temperature in celcius is: {:.2}", celcius);
}
