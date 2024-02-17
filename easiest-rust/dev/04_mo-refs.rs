fn pr_str(pr_str: &String) {
    println!("{}", pr_str);
}

fn add_str(add_str: &mut String) {
    add_str.push_str("-more_text");
    println!("{}", add_str);
} 

fn add_word(mut da_str: String) {
    da_str.push_str("-added-word");
    println!("{}", da_str)
}

fn main() {
    println!("\n\n# Func ref\n");
    let my_string = String::from("hello");
    pr_str(&my_string);
    pr_str(&my_string);

    println!("\n\n# Func mut ref\n");
    let mut my_string = String::from("A string");
    add_str(&mut my_string);
    add_str(&mut my_string);
    println!("original variable: {}", my_string);

    println!("\n\n# Func mut var\n");
    let my_string = String::from("A string");
    add_word(my_string);
    // println!("{}", my_string)
}
