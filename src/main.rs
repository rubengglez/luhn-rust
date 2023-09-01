pub fn luhn(cc_number: &str) -> bool {
    let no_spaces = cc_number.replace(" ", "");
    if no_spaces.len() < 2 {
        return false;
    }

    if !no_spaces.chars().all(char::is_numeric) {
        return false;
    }

    let mut sum = 0;
    let mut i = 1;
    let mut iter = no_spaces.rsplit("");
    while let Some(c) = iter.next() {
       if c.is_empty() {
            continue;
        }

        let digit = c.parse::<i32>().unwrap();
        if i % 2 == 0 {
            let doubled = digit * 2;
            sum += sum_digits(doubled);
        } else {
            sum += digit;
        }
        i += 1;
    }

    sum % 10 == 0
}

fn sum_digits(mut num: i32) -> i32 {
    let mut sum = 0;
    while num != 0 {
        sum += num % 10;
        num /= 10;
    }
    sum
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {}
