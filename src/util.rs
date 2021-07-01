use std::usize;

pub fn print_align(var_name: &str, value: &str) {
    let new_var_name = get_fixed_length_string(var_name, 34);
    println!("{}:\t\t\t{}", new_var_name, value);
}

pub fn get_fixed_length_string(str:&str, expected_length:usize)->String{
    let diff = expected_length - str.len();
    let padding = " ".repeat(diff);
    return format!("{}{}", str, padding);
}