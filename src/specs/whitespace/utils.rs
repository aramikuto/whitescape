pub fn encode_number(number: i32) -> String {
    let mut result = String::new();
    result += if number < 0 { "\t" } else { " " };
    let mut number = number.abs();
    if number == 0 {
        result += " ";
    }
    let mut bits: Vec<i32> = Vec::new();
    while number > 0 {
        bits.push(number % 2);
        number /= 2;
    }
    bits.reverse();
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
