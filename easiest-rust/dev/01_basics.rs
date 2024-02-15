fn main() {
    //comment
    let some_number = /* another comment */ 101;
    println!("Number is {}", some_number);

    // Primitive Types

    // integers: 
    // signed (+1 -100): i8, i16, 132, i64, i128, isize
    // unsigned (only positive): u8, u16,u32, u64, u128, usize
    // isize, usize: computer architecture
    //
    // i8 is one byte and i128 5 bytes
    //
    
    // chars
    let _first_letter = 'A';
    let _space = ' ';
    let _other_language_char = 'Ꮔ';
    let _emoji = '😺';
    
    // this will not work because the default number type is i32
    // error[E0604]: only `u8` can be cast as `char`, not `i32`
    // println!("{}", some_number as char);

    // a char can be cast as a u8
    println!("i32 cast as u8 as char\n{}", some_number as u8 as char);

    let u8_number: u8 = 110;
    println!("u8 cast as char\n{}", u8_number as char);

    
    // char are encode to use the least amount of memory
    println!("Size of a char is {} bytes", std::mem::size_of::<char>()); // 4 bytes
    println!("char a is {} bytes", "a".len());
    println!("char ß is {} bytes", "ß".len());
    println!("char 国 is {} bytes", "国".len());
    println!("char 𓅱 is {} bytes", "𓅱".len());

    let slice = "Hello!";
    println!("slice is {} bytes and {} chars", slice.len(), slice.chars().count());
    let pie = "안녕!";
    println!("pie is {} bytes and {} chars", pie.len(), pie.chars().count());
    
    // floats
    let float_one: f32 = 2.5;
    let float_two = 7.5;
    let float_three = float_one + float_two;

    println!("float: one {}, two {}, three {}", float_one, float_two, float_three)
}
