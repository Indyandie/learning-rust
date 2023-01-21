fn number() -> i32 {
    8 // if no ';' will return
}

fn multiply(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

fn times_two(num: i32) -> i32 {
    multiply(num, 2)
}

fn main() {
    let code_block_num = {
        let num = 5;
        num * 10
    };

    let no_return = {
        let _a = 8;
    };

    print!("no new line. ");
    println!(
        "Hello, world! Number {:?} and {}. no return {:#?}",
        code_block_num,
        multiply(2,number()),
        no_return
    );

    println!("\n\nSmallest number {}, Largest number {}", i8::MIN, i8::MAX);
    println!("Smallest number {}, Largest number {}", u8::MIN, u8::MAX);
    println!("Smallest number {}, Largest number {}", i128::MIN, i128::MAX);
    println!("Smallest number {}, Largest number {}", u128::MIN, u128::MAX);

    println!("\nShadowing");

    let f_num = {
        let x = 10;
        let y = 4;
        let x = times_two(x);
        let x = x + y;
        x
    };

    println!("\nfinal shadowed number is {}", f_num);

    // print char and string bytes
    println!("\nchar bytes: {:?}", b"abc. DEF!"); // b will print as numbers

    // print with r##
    println!("print ex{:?}", br##"/,'"', and #."##);
    
    // unicode
    println!("\nhexadecimal for H: {:x}", 'H' as u32); // print char as hexadecimal
    println!("unicode char 48 \u{48}");

    // pointer address
   	let num = 88;
		let num_ref = &num;
		println!("\nreference pointer {:p}", num_ref);
    
    // binary, hexadecimal, octal
    println!("binary: {:b}, hexadecimal: {:x}, octal: {:o}", num, num, num);

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
    println!("\n{num:~<30.100}", num = "LEFT");
    println!("\n{num:~>30.100}", num = "RIGHT");
}
