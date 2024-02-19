struct Item {
    num: u8,
}

impl Item {
    fn compare_num(&self, other_num: u8) {
        println!(
            "{} and {} are equal, {}.",
            self.num,
            other_num,
            self.num == other_num
        );
    }
}

fn main() {
    println!("# Dot Operator Dereferencing\n");

    println!("\n## Dereferencing\n");
    let num = 8;
    let num_ref = &num;

    // this will not compile, different types u8 and &u8
    // println!("{}", num == num_ref);
    println!("num and num_ref are equal - {}", num == *num_ref);

    println!("\n## Dot Operator\n");
    let item = Item { num: 8 };
    let ref_item_num = &item.num; // &u8 type

    // cannot compare &u8 and u8
    // println!("{}", ref_item_num == 8);

    println!("ref_item_num is equal to 8 - {}", *ref_item_num == 8);

    // using the dot operator
    let ref_item_num_dot = &item;

    println!("\n\n## Dot Operator");

    // no ref required
    println!(
        "ref_item_num_dot.num is equal to 8 - {}",
        ref_item_num_dot.num == 8
    );

    println!("\n## Struct Method\n");
    let ref_item = &item;
    let ref_item_deep = &ref_item;

    item.compare_num(8);
    ref_item.compare_num(8);
    ref_item_deep.compare_num(8);
}
