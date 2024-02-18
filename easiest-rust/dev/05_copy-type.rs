fn prt_num(num: i32) {
    println!("{}", num);
}

fn get_len(input: String) {
    println!("Length is {} word(s)", input.split_whitespace().count());
}

fn get_len_ref(input: &String) {
    println!("Length is {} word(s)", input.split_whitespace().count());
}

fn lp_rtn(mut cnt: i32) -> i32 {
    loop {
        cnt += 1;
        if cnt % 50 == 0 {
            break;
        }
    }
    cnt
}
fn main() {
    println!("Copy type");
    let my_num = 8;
    prt_num(my_num);
    prt_num(my_num);

    // Using clones
    println!("\n\nUsing Clone");
    let mut my_str = String::new();
    for _ in 0..50 {
        my_str.push_str(", more words");
        get_len(my_str.clone());
    }

    // Using refs
    println!("\n\nUsing Refs");
    let mut my_str = String::new();
    for _ in 0..50 {
        my_str.push_str(", more words");
        get_len_ref(&my_str);
    }

    // Uninitialized
    println!("\n\nUsing Uninitialized Var");
    let my_num;

    {
        let num = { 170 };

        my_num = lp_rtn(num)
    }

    println!("my_num: {}", my_num);

    let my_num;
    {
        my_num = 100;
    }

    println!("my_num: {}", my_num);
}
