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

fn even(num: i32) -> Result<(), ()> {
    if num % 2 == 0 {
        Ok(())
    } else {
        Err(())
    }
}

fn check_five(num: i32) -> Result<i32, String> {
    match num {
        5 => Ok(num),
        _ => Err("Not 5.".to_string()),
    }
}

fn main() {
    let vec1 = vec![21, 32, 43];
    let vec2 = vec![5, 15, 25, 35, 45, 55];
    let vec1_opt = fifth_opt(vec1.clone());
    let vec2_opt = fifth_opt(vec2.clone());

    println!("# Option\n\n## Basic\n");
    print!("{vec1:?}\n{vec1_opt:?}\n\n{vec2:?}\n{vec2_opt:?}\n\n---\n\n");

    println!("## Option Values\n");
    let unwrapped_opt = vec2_opt.unwrap();
    println!("Unwrapped value: {unwrapped_opt}\n");

    let mut vec_opts = Vec::new();
    vec_opts.push(fifth_opt(vec1.clone()));
    vec_opts.push(fifth_opt(vec2.clone()));
    print_opts(vec_opts);

    println!("\n---\n");
    println!("\n## .is_some\n");

    let vec_opts = vec![vec1, vec2];
    for vc in vec_opts {
        let vec_5 = fifth_opt(vc);

        if vec_5.is_some() {
            println!("is some unwrapped: {}", vec_5.unwrap());
        } else {
            println!("Nothing!");
        }
    }

    println!("\n---\n");
    println!("\n## get()\n");

    let vec1 = vec![1, 32];
    let get0 = vec1.get(0);
    let get1 = vec1.get(1);
    let get2 = vec1.get(2);

    println!("{get0:?}\n{get1:?}\n{get2:?}");

    println!("\n---\n");
    println!("\n# Result\n");

    println!("\n## is_ok\n");

    let num = 8;
    if even(num).is_ok() {
        println!("Even!\n")
    } else {
        println!("Odd!\n")
    }

    let mut vec_res = Vec::new();

    for num in 1..23 {
        vec_res.push(check_five(num));
    }

    println!("{vec_res:?}");

    // Test error unwrap panic
    // let test_err: Result<i32, String> = Err("test".to_string());
    // let unwrap_err = test_err.unwrap();
    // println!("{unwrap_err:?}")
    // thread 'main' panicked at 15-option-result.rs:86:31:
    // called `Result::unwrap()` on an `Err` value: "test"
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
}
