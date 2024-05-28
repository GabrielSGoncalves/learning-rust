fn main() {
    let numbers = [1, 9, -2, 0, 23, 20, -7, 37, 20, 56, -18, 20, 3];
    let mut max: i32;
    let mut min: i32;
    let mut mean: f64;
    
    max = numbers[0];
    min = numbers[0];
    mean = 0.0;
    for &n in numbers.iter() {
        if n > max {
            max = n;
        } else if n < min {
            min = n;
        }
        mean += n as f64;
    }
    mean /= numbers.len() as f64;
    mean = mean.round() * 10.0;
    mean /= 10.0;

    
    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.0);

    println!("Test passed!");
}