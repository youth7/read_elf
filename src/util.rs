const TAB: &str = "\t\t\t";

pub fn print_align(var_name: &str, value: &str) {
    let diff = 34 - var_name.len();
    let spaces = " ".repeat(diff);
    let new_var_name = format!("{}{}", var_name, spaces);
    // println!("{}需要补全的长度是{}", var_name, value.len());
    println!("{}：{}{}", new_var_name, TAB, value);
}