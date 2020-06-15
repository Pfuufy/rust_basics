fn main() {
    let number = 0;

    if number < 5 {
        println!("Condition was true!");
    } else if number >= 5 {
        println!("Condition was false!");
    }

    if number != 0 {
        println!("The number does not equal 0");
    }

    if number == 0 {
        println!("The number equals 0")
    }

    const CONDITION: bool = false;

    let num = if CONDITION { '1' } else { '2' };

    println!("num: {}", num);
}
