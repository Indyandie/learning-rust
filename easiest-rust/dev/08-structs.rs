// struct UnitStruct;

struct Color (u8, u8, u8);
struct CoolColor (String, Color);
struct ColorName<'a> { // need lo learn about lifetimes
    name: &'a str,
    color: Color,
} // no ; required due to the code block {}

#[derive(Debug)]
struct Pokemon {
    name: String,
    pokedex_number: u16,
    r#type: Vec<String>,
}

fn main () {
    println!("# Structs");

    let blue = CoolColor("Blue".to_string(), Color(0, 0, 255));

    println!("## Tuple Structs");
    println!("{} is a CoolColor tuple struct. It has {} blue.", blue.0, blue.1.2);
    println!("\n\n");

    let red = Color(255, 0, 0);
    let color_red = ColorName {
        name: "Red",
        color: red,
    };
    
    println!("## Tuple Structs");
    println!("{} is a ColorName named struct. it has {} red.", color_red.name, color_red.color.0);

    println!("\n\n");

    let name = "Bulbasaur".to_string();
    let pokedex_number = 1;
    let poison = "Poison".to_string();
    let grass = "Grass".to_string();

    let bulbasaur = Pokemon {
        name,
        pokedex_number,
        r#type: vec![grass, poison]
    };

    println!("Pokemon number {} is {}. Type is {} and {}.", bulbasaur.pokedex_number, bulbasaur.name, bulbasaur.r#type[0], bulbasaur.r#type[1]);
    println!("\n{:?}", bulbasaur)
}
