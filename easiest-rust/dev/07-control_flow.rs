fn main() {
    println!("# Control Flow\n");
    println!("## if\n");

    let num = 5;

    if num == 7 {
        println!("It's seven")
    } else if num == 6 {
        println!("It's six")
    } else {
        println!("{} is not seven or six.", num)
    }

    println!("\n---\n");

    println!("### if: Logical Operators\n");

    if num % 2 == 0 && num % 3 == 0 {
        println!("Divisible by 2 and 3.")
    } else if num % 2 > 0 || num > 4 {
        println!("Is even or greater than 4")
    } else {
        println!("Who knows?")
    }

    println!("\n---\n");

    println!("## Match\n");

    let num: u8 = 16;
    match num {
        0 => println!("It's zero."),
        1 => println!("It's one."),
        2 => println!("It's two."),
        // catch all. Every other number that's not specified. 
        _ => println!("It's some other number."),
    }

    println!("\n---\n");

    println!("### Match: declare a value \n");

    let num: u8 = 32;
    
    let match_num = match num {
        0 => 0,
        5 => 10,
        _ => 2,
    };

    println!("match_num is {}", match_num);

    println!("\n---\n");

    println!("### Match: Using tuples \n");

    let sky = "cloudy";
    let temp = 40;
    
    match (sky, temp) {
        ("cloudy", 40) => println!("It's cloudy and cold."),
        ("clear", 70) => println!("It's a nice day."),
        ("cloudy", 70) => println!("It's cloudy and warm."),
        _ => println!("I am not sure."),
    }

    println!("\n---\n");
}
