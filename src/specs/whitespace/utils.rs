use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

fn to_bin(mut number: i32) -> Vec<i32> {
    let mut bits: Vec<i32> = Vec::new();
    while number > 0 {
        bits.push(number % 2);
        number /= 2;
    }
    bits.reverse();
    bits
}

pub fn encode_number(number: i32) -> String {
    let mut result = String::new();
    result += if number < 0 { "\t" } else { " " };
    let mut number = number.abs();
    if number == 0 {
        result += " ";
    }
    let mut bits = to_bin(number);
    for bit in bits {
        if bit == 0 {
            result.push(' ');
        } else {
            result.push('\t');
        }
    }
    result += "\n";
    result
}

#[cfg(test)]
mod test {
    use crate::util::unbleach;

    use super::*;

    #[test]
    fn positive_number() {
        let input = 823;
        let expect = "sttssttstttn";
        let actual = encode_number(input);
        assert_eq!(expect, unbleach(actual));
    }

    #[test]
    fn negative_number() {
        let input = -823;
        let expect = "tttssttstttn";
        let actual = encode_number(input);
        assert_eq!(expect, unbleach(actual));
    }
}

pub fn number_to_label(label: &i32) -> String {
    let mut result = String::new();
    let mut number = *label;
    for bit in to_bin(number) {
        if bit == 0 {
            result.push(' ');
        } else {
            result.push('\t');
        }
    }
    result += "\n";
    result
}
