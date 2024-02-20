use std::any::type_name;
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
}
