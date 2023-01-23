fn match_colors(rgb: (i32, i32, i32)) {
    match rgb {
        (r, _, _) if r < 10 => println!("Not much red"),
        (_, g, _) if g < 10 => println!("Not much green"),
        (_, _, b) if b < 10 => println!("Not much blue"),
        _ => println!("Each color has a least 10.")
    }
}

fn match_nums(input: i32) {
    match input {
        num @ 4 => println!("{} is unlucky in China.", num),
        num @ 13 => println!("{} is unlucky in Italy.", num),
        _ => println!("Some basic number."),
    }
}


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

    println!("### If & Match \n");

    let kids = 5;
    let married = true;

    match (kids, married) {
        (kids, married) if married == false && kids > 0 => println!("Not married with {} kids.", kids),
        (kids, married) if kids == 0 && married == true => println!("Married and no children."),
        _ => println!("Married? {}. Number of kids {}.", married, kids),

    }

    println!("\n---\n");

    println!("### If & Match: Multi Catch All '_'\n");

    let red = (200, 0, 0);
    let green = (0, 200, 0);
    let blue = (0, 0, 200);

    match_colors(red);
    match_colors(green);
    match_colors(blue);

    println!("\n---\n");

    println!("### Match arms \n");

    let num = 10;
    let sum_var = match num {
        10 => 8,
        // _ => "Not ten",  // arms don't match 
        _ => 100 
        
    };

    println!("sum var is {}", sum_var);

    println!("\n---\n");


    println!("### Match arms \n");

    match_nums(50);
    match_nums(13);
    match_nums(4);

    println!("\n---\n");
}
