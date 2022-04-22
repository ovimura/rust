fn main() {
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32;
    let mut min: i32;
    let mean: f64;

    max = -100;
    for &item in numbers.iter() {
        if item > max {
            max = item;
        }
    }

    min = 100;
    for &item in numbers.iter() {
        if item < min {
            min = item;
        }
    }

    let mut total = 0;

    for &item in numbers.iter() {
        total += item;
    }
    mean = total as f64 / numbers.len() as f64;
    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed!");
}
