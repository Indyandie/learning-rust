fn color_comp(rgb: (i32, i32, i32)) {
    let (red, green, blue) = rgb;
    println!("Checking color {red}, {green}, {blue}.");
    let colors = vec![(red, "red"), (green, "green"), (blue, "blue")];

    let mut colors_min_10 = true;
    for color in colors {
        if color.0 < 10 {
            colors_min_10 = false;
            println!("Not much {}", color.1);
        }
    }

    if colors_min_10 {
        println!("Each color has atleast 10.");
    }
    println!();
}
fn main () {
    println!("# Loops");

    println!("# loop");
    let mut lp_count = 0;

    loop {
        lp_count += 1;

        println!("loop counter: {lp_count}");
        if lp_count > 6 {
            break;
        }
    }
    println!("\n");

    println!("# loop labels");
    let mut lp_count1 = 0;
    let mut lp_count2 = 0;

    'lp1: loop {
        lp_count1 += 1;
        println!("counter 1: {lp_count1}");

        '_lp2: loop {
            println!("counter 2: {lp_count2} \n");
            lp_count2 += 1;

            if lp_count2 > 5 {
                break 'lp1;
            }
        }
    }
    println!("\n");

    println!("# while loop");
    let mut wh_count = 10;
    let mut wh_stat = false;

    while !wh_stat {
        wh_count -= 1;
        println!("counter: {wh_count}");

        if wh_count < 0 {
            wh_stat = true;
            println!("Done!")
        }
    }
    println!("\n");

    println!("# for loop");
    for num in 0..3 {
        println!("number: {num}");
    }
    println!("\n Iterator\n");

    for (idx, num) in (2..=10).enumerate() {
        println!("{idx}: {num}")
    }
    println!("\n");

    let lines = "hello\nworld!\nwhats\nup".lines();
    for (lnum, ln) in lines.enumerate() {
        println!("{lnum}: {ln}")
    }
    println!("\n");

    println!("# break return value");
    let mut count = 0;

    let rtn_val = loop {
        count += 1;
        if count % 53 == 3 {
            break count;
        }
    };
    println!("return value is {rtn_val}");
    println!();
    println!("\n");

    println!("# loop");
    let red = (200, 0, 0);
    let gray = (50, 50, 50);
    let redish = (220, 20, 0);
    
    color_comp(red);
    color_comp(gray);
    color_comp(redish);
    println!("\n");
}
