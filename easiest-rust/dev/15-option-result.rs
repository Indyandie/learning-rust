fn fifth_opt(val: Vec<i32>) -> Option<i32> {
    if val.len() < 5 {
        None
    } else {
        Some(val[4])
    }
}

fn print_opts(opts: Vec<Option<i32>>) {
    for opt in opts {
        match opt {
            Some(num) => println!("Some value: {num}"),
            None => println!("Nothing"),
        }
    }
}

fn main() {
    let vec1 = vec![21, 32, 43];
    let vec2 = vec![5, 15, 25, 35, 45, 55];
    let vec1_opt = fifth_opt(vec1.clone());
    let vec2_opt = fifth_opt(vec2.clone());

    println!("# Option\n\n## Basic\n");
    print!("{vec1:?}\n{vec1_opt:?}\n\n{vec2:?}\n{vec2_opt:?}\n\n---\n");

    println!("## Option Values\n");
    let unwrapped_opt = vec2_opt.unwrap();
    println!("Unwrapped value: {unwrapped_opt}\n");

    let mut vec_opts = Vec::new();
    vec_opts.push(fifth_opt(vec1));
    vec_opts.push(fifth_opt(vec2));
    print_opts(vec_opts);
}
