// fn rtn_str() -> &'static String {
fn rtn_str() -> String {
    let country = String::from("Japan");
    // let country_ref = &country;
    // country_ref // This is a problem
    country
}

fn main() {
    println!("A String is always {:?} bytes. It is sized", std::mem::size_of::<String>());
    println!("A i8 is always {:?} bytes. It is sized", std::mem::size_of::<i8>());
    println!("A f64 is always {:?} bytes. It is sized", std::mem::size_of::<f64>());
    println!("An &str can be any size.  '서태지' is {:?} bytes. not sized.", std::mem::size_of_val("서태지"));
    println!("'Hello world!' is {:?} bytes. not sized.", std::mem::size_of_val("Hello world!"));

    // String that takes text and creates a String
    let string_1 = String::from("This is a string of text.");
    
    // make &str into a String
    let string_2 = "This is a string of text".to_string();
    
    // format macro
    let name = "string";
    let string_3 = format!("This is a {} of text", name);
    
    // .into() the String type must be specified
    let string_4: String = "This is a string of text".into();
    
    println!(
        "\n\n#Strings\n\nString::from(): {}\n&str.to_string(): {}\nformat! {}\ninto() {}",
        string_1, string_2, string_3, string_4
    );

    println!("\n\n# Mutable Refs");
    let mut num = 8;
    println!("\nog num {}", num);
    let num_ref = &mut num;
    *num_ref += 10;

    // breaks reference rules
    // let num_ref_i = &num;
    // println!("mutated {}, imut {}", num_ref, num_ref_i);

    println!("mutated {}", num_ref);

    let num2 = 24;
    let num2_ref = &num2;
    println!("imut {}", num2_ref);

    
    println!("\n\nTriple Refs");
    let num2 = 800;
    println!("\nnum2: {}", num2);
    let num2_3ref = &&&num2;
    println!("num2_3ref is equal to num2? {}", num2 == ***num2_3ref);
    
    println!("\n\nShadow Refs");
    let city = String::from("Town");
    let city_ref = &city;
    let city = 100;
    println!("og ref {}, shadow {}", city_ref, city);

    let country = rtn_str();
    print!("\n\ncountry: {}", country)
}
