fn fifth_opt(val: Vec<i32>) -> Option<i32> {
    if val.len() < 5 {
        None
    } else {
        Some(val[4])
    }
}

fn main() {
    let vec1 = vec![21, 32, 43];
    let vec2 = vec![5, 15, 25, 35, 45, 55];
    let vec1_opt = fifth_opt(vec1.clone());
    let vec2_opt = fifth_opt(vec2.clone());

    print!("{vec1:?}\n{vec1_opt:?}\n\n{vec2:?}\n{vec2_opt:?}\n")
}
