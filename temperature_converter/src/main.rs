/// Allows a user to convert temperature from
/// degrees celsius to fahrenheit and vice versa
fn main() {
    println!("Temperature converter\n");
    println!("1. Convert from Fahrenheit to Celsius");
    println!("2. Convert from Celsius to Fahrenheit");
    println!("3. Exit");

    let mut option = String::new();

    println!("\nPick an option:");
    std::io::stdin()
        .read_line(&mut option)
        .expect("Unable to read user input");

    let option = option.trim();

    match option {
        "1" => {
            let temp = get_temp("fahrenheit");
            println!(
                "{} degrees fahrenheit is {:.2} degrees celsius",
                temp,
                to_celsius(temp)
            )
        }
        "2" => {
            let temp = get_temp("celsius");
            println!(
                "{} degrees celsius is {:.2} degrees fahrenheit",
                temp,
                to_fahrenheit(temp)
            )
        }
        _ => println!("Invalid option, try again"),
    }
}

/// Returns a temperature in degrees celsius
///
/// ## Parameters
///
/// * `fahrenheit` - a float that holds the value of fahrenheit temperature
fn to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * (5.0 / 9.0)
}

/// Returns a temperature in degrees fahrenheit
///
/// ## Parameters
///
/// * `celsius` - a float that holds the value of celsius temperature
fn to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 1.8) + 32.0
}

/// Returns temperature value from the user
///
/// ## Parameters
///
/// * `temp_type` - a string representing the temperature input type (celsius or fahrenheit).
fn get_temp(temp_type: &str) -> f64 {
    println!("\nEnter {} temperature:", temp_type);

    let mut temp = String::new();
    std::io::stdin()
        .read_line(&mut temp)
        .expect("Unable to read user input");

    temp.trim().parse().unwrap_or(0.0)
}
