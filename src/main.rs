use std::io::stdin;
use std::io::Write;

struct Weight(f32);
struct Height(f32);
struct Bmi(f32);

fn calc_bmi(w: Weight, h: Height) -> Option<Bmi> {
    if h.0 <= 0.0 {
        return None;
    }
    // weight (kg) / height**2 (m)
    Some(Bmi(w.0 / (h.0 * h.0)))
}

#[test]
fn test_calculate_bmi() {
    let r = calc_bmi(Weight(69.0), Height(1.69)).unwrap();
    assert_eq!(r.0, 24.158817);
}

#[test]
fn test_calculate_bmi_failing() {
    let r = calc_bmi(Weight(69.0), Height(-0.0));
    assert!(r.is_none());
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

    let bmi = calc_bmi(weight, height).unwrap();
    println!("BMI is {}", bmi.0);
}
