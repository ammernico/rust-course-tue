use std::io::stdin;
use std::io::Write;

fn calc_bmi(weight: f32, height: f32) -> f32 {
    // weight (kg) / height (m)
    weight / (height * height)
}

fn take_user_input() -> f32 {
    let mut input_line = String::new();
    stdin()
    .read_line(&mut input_line)
    .expect("Failed to read line");

    let r: f32 = input_line
        .trim()  // removes whitespaces
        .parse()
        .expect("Not a valid float");
    r
}

fn terminal_user_bmi() {
    print!("Please enter the weight in kilograms: ");
    let _ = std::io::stdout().flush();
    let weight: f32 = take_user_input();

    print!("Please enter the height in meters : ");
    let _ = std::io::stdout().flush();
    let height: f32 = take_user_input();

    let bmi = calc_bmi(weight, height);
    println!("BMI is {}", bmi);
}

fn main() {
    terminal_user_bmi()
}
