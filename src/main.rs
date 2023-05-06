use std::io::stdin;
use std::io::Write;

mod tests;

pub struct Weight(f32);
pub struct Height(f32);
#[derive(Debug)]
pub struct Bmi(f32);
#[derive(Debug, PartialEq)]
pub enum BmiError {
    HeightCannotBeZero,
}

pub fn calc_bmi(w: Weight, h: Height) -> Result<Bmi, BmiError> {
    if h.0 <= 0.0 {
        return Err(BmiError::HeightCannotBeZero);
    }
    // weight (kg) / height**2 (m)
    Ok(Bmi(w.0 / (h.0 * h.0)))
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
    let weight = Weight(weight);
    let height = take_user_input("height in meters");
    let height = Height(height);

    let bmi = calc_bmi(weight, height);
    match bmi {
        Ok(bmi) => println!("Bmi is {}", bmi.0),
        _ => println!("Error while calculating"),
    }
}
