fn main() {

    // Arrays

    let basic = ["circle", "triangle", "square"]; // [&str, 3]
    // let bad_array = [10, "triangle", 32.32]

    let bees = ["B"; 100]; // [&str, 100]
    println!("{:?} \n\n", bees);

    // Index
    println!("Basic 2nd is {}", basic[1]);
    
    // Slice
    let ten = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let three_to_five = &ten[2..5];
    let start_at_two = &ten[1..];
    let end_at_five = &ten[..5];
    let everything = &ten[..];
    println!(
        "san-go: {:?}\nni: {:?}\nyon: {:?}\nALL: {:?} ",
        three_to_five, start_at_two,
        end_at_five, everything
    );
    let niroku = &ten[1..=5]; // 2nd to the 6th entry
    println!("Two to six: {:?}", niroku);

    // Vecs

    let a_str = String::from("A");
    let b_str = String::from("B");
    
    let mut a_vec = Vec::new();
    
    a_vec.push(a_str); // Set to a Vec<String>
    a_vec.push(b_str);

    println!("\n\nVector\n{:?}", a_vec);

    // Declaring Vecs
    let str_vec: Vec<String> = Vec::new(); // type
    let mac_vec = vec![8, 16, 32]; // vec macro
    println!("\n{:?}\n{:?}", str_vec, mac_vec);

    // Slice
    let vec_ten = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    let sango = &vec_ten[2..6];
    let ni = &vec_ten[1..];
    let go = &vec_ten[..5];
    let all = &vec_ten[..];
    
    println!(
        "\nSlice: {:?}\nsango: {:?}\nni: {:?}\ngo: {:?}\nall: {:?}",
        vec_ten, sango, ni, go, all
    );

    // Capacity
    println!("\nCapacity\n");
    let mut a_vec = Vec::new();
    println!("{}", a_vec.capacity()); // cap 0
    
    a_vec.push("a");
    println!("{}", a_vec.capacity());
    
    a_vec.push("a");
    a_vec.push("a");
    a_vec.push("a");
    println!("{}", a_vec.capacity());
    
    a_vec.push("a");
    println!("{}", a_vec.capacity());

    println!("\nFaster\n");
    let mut c_vec = Vec::with_capacity(8);
    c_vec.push('a');
    println!("{}", c_vec.capacity());
    c_vec.push('a');
    println!("{}", c_vec.capacity());
    c_vec.push('a');
    println!("{}", c_vec.capacity());
    c_vec.push('a');
    println!("{}", c_vec.capacity());
    c_vec.push('a');
    println!("{}", c_vec.capacity());
    c_vec.push('a');
    println!("{}", c_vec.capacity());

    // Array to Vector
    println!("\nArray into Vector\n");
    let d_vec: Vec<i8> = [1, 2, 3].into();
    println!("{:?}", d_vec);


    // Tuples
    let rnd_tuple = ("hello", 8, vec!['a'], 'b', [8, 9, 10], 7.8);
    
    println!(
        "\n\n# Tuple\n\n1. {:?}\n2. {:?}\n3. {:?}\n4. {:?}\n5. {:?}\n6. {:?}",
        rnd_tuple.0, rnd_tuple.1, rnd_tuple.2, rnd_tuple.3, rnd_tuple.4, rnd_tuple.5
    );

    // Destructuring
    let str_vec = vec!["one", "two", "three"];

    let (a, b, c) = (str_vec[0], str_vec[1], str_vec[2]);
    println!(
        "\n\n# Destructuring\n\na: {:?}, b: {:?}, c: {:?}",
        a, b, c
    );

    let (_, _, variable) = (str_vec[0], str_vec[1], str_vec[2]);
    println!("\nvarible {:?}", variable);
}
