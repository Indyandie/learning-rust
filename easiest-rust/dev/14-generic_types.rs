use std::any::type_name;
use std::cmp::PartialOrd;
use std::fmt::{Debug, Display};

#[derive(Debug)]
struct Animal {
    name: String,
    id: i32,
}

fn debug_item<T: Debug>(item: T) {
    println!("Debug item: {item:?}");
}

fn gen_type<T>(var: T) -> T {
    println!("This is generic type.");
    var
}

fn get_type<T: Debug>(var: T) -> &'static str {
    let name = type_name::<T>();
    println!("value: {var:?}\n\ttype: {name:?}\n\n");
    name
}

fn compare_display<T: Display, U: Display + PartialOrd>(msg: T, num1: U, num2: U) {
    let comparison = num1 > num2;
    println!("\n\n{msg}\nIs {num1} greater than {num2}? {comparison}");
}

fn where_gen<T, U, V, W>(name: T, id: U, msg: V, list: W)
where
    T: Display,
    U: Display,
    V: Display,
    W: Debug,
{
    println!("{name}\n\t{id}\n\t{msg}\n\t{list:?}")
}

fn main() {
    let num = gen_type(43);
    get_type(num);

    let str = gen_type("Hello!".to_string());
    get_type(str);

    // ----------------------------------------------------------
    // Traits

    let gary = Animal {
        name: format!("Gary"),
        id: 1,
    };
    println!("name: {}\nid: {}\n\n", gary.name, gary.id);
    let num = 3248;

    debug_item(&gary);
    get_type(gary);
    debug_item(num);

    // ----------------------------------------------------------
    // Multiple generic types

    compare_display("Yo!", 324, 3);
    compare_display("Another comparison...", 324, 30000);

    let msg = "Yet another comparison with defined variables...";
    let num1: i8 = 10;
    let num2: i8 = 40;

    compare_display(msg, num1, num2); // generic rules: num1 and num2 must be the same type

    // ----------------------------------------------------------
    // where

    let name = "Name";
    let id = 123;
    let msg = "Hello there!".to_string();
    let list = vec![1, 2, 3, 4, 5, 123, 52];

    where_gen(name, id, msg, list);
}
