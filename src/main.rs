use std::io::stdin;
use std::io::Write;

struct BmiValues {
    weight: f32,
    height: f32,
}

fn calc_bmi(b: BmiValues) -> f32 {
    // weight (kg) / height**2 (m)
    b.weight / (b.height * b.height)
}

fn take_user_input(value: &str) -> f32 {
    let r: f32;
    loop {
        print!("Please enter the {}: ", value);
        let _ = std::io::stdout().flush();

        let mut input_line = String::new();
        stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");

        let input = input_line
            .trim() // removes whitespaces
            .parse();

        match input {
            Ok(input) => {
                r = input;
                break;
            }
            _ => {
                println!("bad input")
            }
        }
    }
    r
}

fn main() {
    let weight = take_user_input("weight in kilograms");
    let height = take_user_input("height in meters");

    let bmi_values = BmiValues { weight, height };

    let bmi = calc_bmi(bmi_values);
    println!("BMI is {}", bmi);
}
