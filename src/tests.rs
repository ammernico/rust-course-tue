#[cfg(test)]
mod tests {
    use crate::{Height, Weight, calc_bmi, BmiError};

    #[test]
    fn test_calculate_bmi() {
        let r = calc_bmi(Weight(69.0), Height(1.69)).unwrap();
        assert_eq!(r.0, 24.158817);
    }

    #[test]
    fn test_calculate_bmi_failing() {
        let r = calc_bmi(Weight(69.0), Height(-0.0));
        assert!(r.is_err());
        let e = r.unwrap_err();
        assert_eq!(e, BmiError::HeightCannotBeZero);
    }
}
