enum ThingsInTheSky {
    Sun(String),
    Stars(String),
}

enum Mood {
    Happy,
    Angry,
    Sad,
}

fn getsky(time: i32) -> ThingsInTheSky {
    use ThingsInTheSky::*;

    match time {
        6..=18 => Sun("The Sun is in the sky.".to_string()), // match a range
        _ => Stars("The stars are in the sky.".to_string()),
    }
}

fn printsky(state: &ThingsInTheSky) {
    use ThingsInTheSky::*;

    match state {
        Sun(desc) => println!("{desc}"),
        Stars(desc) => println!("{desc}"),
    }
}

fn getmoodlevel(mood: &Mood) -> u8 {
    use Mood::*;

    match mood {
        Happy => 10,
        Angry => 5,
        Sad => 0,
    }
}

enum PowerLevels {
    DoYouEvenLift,
    Weak,
    GardenAnt,
    GroceryBags,
    OverNineThousand = 90001,
    OldManStrength,
}

enum Number {
    U16(u16),
    I32(i32),
}

fn makenumber(number: i32) -> Number {
    use Number::*;

    match number.is_positive() {
        true => U16(number as u16),
        false => I32(number as i32),
    }
}

fn main() {
    use Number::*;

    println!("# Variant Values\n");

    let some_numbers = vec![makenumber(-234), makenumber(23)];
    for num in some_numbers {
        match num {
            U16(num) => println!("{} is a u16.", num),
            I32(num) => println!("{} is a i32.", num),
        }
    }
    println!("\n\n");

    println!("# Variant Values\n");

    use PowerLevels::*;

    let powerlevels = vec![
        DoYouEvenLift,
        Weak,
        GardenAnt,
        GroceryBags,
        OverNineThousand,
        OldManStrength,
    ];

    for level in powerlevels {
        println!("PowerLevels variant {}", level as i32);
    }

    println!("\n\n");

    println!("# Sky Enums\n");

    let time = 4;
    let mysky = getsky(time);

    printsky(&mysky);

    println!("\n\n");

    println!("# Mood Enums\n");

    let my_mood = Mood::Angry;
    let my_mood_level = getmoodlevel(&my_mood);

    println!("My current mood level is {}.", my_mood_level);
    println!("\n\n");
}
