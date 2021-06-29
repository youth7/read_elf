pub fn print_align(var_name: &str, value: &str) {
    let diff = 34 - var_name.len();
    let spaces = " ".repeat(diff);
    let new_var_name = format!("{}{}", var_name, spaces);
    println!("{}:\t\t\t{}", new_var_name, value);
}
