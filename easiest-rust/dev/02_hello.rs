fn number() -> i32 {
    8 // if there is no trailing ';' the function will return the value
}

fn multiply(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

fn times_two(num: i32) -> i32 {
    multiply(num, 2)
}

fn square(num: i32) -> i32 {
    let squared_num = multiply(num, num);
    return squared_num;
}

fn main() {
    let code_block_num = {
        let num = 5;
        num * 10
    };

    let no_return = {
        let _a = 8;
    };

    let squared = square(3);
    println!("squared 3: {}\n", squared);

    print!("pring macro - no new line. ");
    println!(
        "Hello, world!\ncode_block_num - {} (display), {:?} (debug) and {:#?} (pretty).\nmultiply(2, number()) - {} (display), {:?} (debug), and {:#?} (pretty).\nno_return - {:?} (debug)",
        code_block_num,
        code_block_num,
        code_block_num,
        multiply(2,number()),
        multiply(2,number()),
        multiply(2,number()),
        no_return
    );

    println!("\n# Ranges\n");
    println!("i8 - Smallest number {}, Largest number {}", i8::MIN, i8::MAX);
    println!("u8 - Smallest number {}, Largest number {}", u8::MIN, u8::MAX);
    println!("i128 - Smallest number {}, Largest number {}", i128::MIN, i128::MAX);
    println!("u128 - Smallest number {}, Largest number {}", u128::MIN, u128::MAX);

    println!("\n# Shadowing");

    let f_num = {
        let x = 10; // x1
        println!("x: {}", x);
        let y = 4;
        println!("x: {}", x);
        let x = times_two(x); // x2 - shadows x1
        println!("x: {}", x);
        let x = x + y; // x3 - shadows x2
        x
    };

    println!("\nfinal shadowed number is {}", f_num);

    // print char and string bytes - b"string_value"
    println!("b\"ab\"  - char bytes: {:?}", b"ab"); // b will print as numbers
    println!("b\"abc. DEF!\" - char bytes: {:?}", b"abc. DEF!"); // b will print as numbers

    // print with r##
    println!("print (r##) ex{:?}", br##"/,'"', and #."##);
    
    let a1 = 'a';
    let a2 = "a";
    println!("a1: {:?} (debug)", a1);
    println!("a2: {:?} (debug)", a2);
    println!("b\"a\": {:?} (debug)", b"a");

    // unicode
    println!("\n\n# Unicode\n");
    println!("hexadecimal for H (as u32, :x): {:x}", 'H' as u32); // print char as hexadecimal
    println!("unicode char 48 (\\u48) \u{48}");

    // pointer address
   	let num = 88;
	let num_ref = &num;
	println!("\nreference pointer (num_ref) {:p}", num_ref); // {:p} - pointer is implegmented for a reference

    
    // binary, hexadecimal, octal
    println!("num - binary(:b): {:b}, hexadecimal(:x): {:x}, octal(:0): {:o}", num, num, num);

    // print order
    let fam = "Kon";
    let dad = "Tachi";
    let son = "Katachi";
    println!("\nThis is {2} {0}, son of {1} {0}", fam, dad, son);

    println!(
        "\n{city1} is in {country}, and {city2} is in {country} too. {city3} is not in {country}",
        city1 = "Austin",
        city2 = "Houston",
        city3 = "Newark",
        country = "Texas"
    );

    println!("\n{:-^30.100}", "CENTER");
    println!("\n{:-^50.100}", "CENTER");
    println!("\n{fam:~<30.100}");
    println!("\n{num:~<30.100}", num = "LEFT");
    println!("\n{num:~>30.100}", num = "RIGHT");
}
