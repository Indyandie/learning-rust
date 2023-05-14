enum ThingsInTheSky {
    Sun,
    Stars,
}

fn getsky(time: i32) -> ThingsInTheSky {
    use ThingsInTheSky::*;

    match time {
        6..=18 => Sun,
        _ => Stars
    }
}

fn printsky(state: &ThingsInTheSky) {
    use ThingsInTheSky::*;

    match state {
        Sun => println!("The Sun is in the sky."),
        Stars => println!("The stars are in the sky.")
    } 
}

fn main() {
    let time = 4;
    let mysky = getsky(time);
    printsky(&mysky);
}
