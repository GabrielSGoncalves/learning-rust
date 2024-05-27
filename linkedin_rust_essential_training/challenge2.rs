fn main() {
    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_farenheit(celsius_temp);

    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed!");
}

fn celsius_to_farenheit(celsius_temp: f64) -> f64 {
    return (celsius_temp * 1.8) + 32.0;
}