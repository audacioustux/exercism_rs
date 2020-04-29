pub fn is_armstrong_number(num: u32) -> bool {
    let num_s = num.to_string();
    let no_digits = num_s.len();

    num_s.chars().fold(0, |sum, n| {
        sum + n.to_digit(10).unwrap().pow(no_digits as u32)
    }) == num
}
