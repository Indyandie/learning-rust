struct Pokemon {
    id: u16,
    name: String,
    level: u8,
    r#type: Vec<String>,
}

struct Item {
    id: u16,
    name: String,
    cost: f64,
    attributes: Vec<String>,
}

impl Item {
    fn new(id: u16, name: String, cost: f64, attributes: Vec<String>) -> Self {
        Self {
            id,
            name,
            cost,
            attributes,
        }
    }
}

fn process_item_strings(item: &Item) {
    let Item {
        id: _,
        name,
        cost: _,
        attributes,
    } = item;

    let item_strings = vec![name, &attributes[0]];

    println!("The item Strings are {:?}", item_strings);
}

enum PokemonType {
    Normal(String),
    _Fire(String),
    _Water(String),
}

fn main() {
    println!("# Destructuring\n");

    println!("## Struct\n");

    let bulbasaur = Pokemon {
        id: 1,
        name: format!("Bulbasaur"),
        level: 5,
        r#type: vec![format!("grass"), format!("poison")],
    };

    let Pokemon {
        id: a,
        name: b,
        level: c,
        r#type: d,
    } = bulbasaur;

    println!("{} {}: level {}, types - {}, {}", a, b, c, d[0], d[1]);
    println!("{a} {b}: level {c}, types - {d:?}");

    let pokeball = Item::new(1, format!("master-ball"), 3.5, vec![format!("holdable")]);

    println!();
    process_item_strings(&pokeball);

    println!("id: {}, cost: {}", pokeball.id, pokeball.cost);

    println!("\n## Enums\n");

    let basic = PokemonType::Normal(format!("Meowth"));
    if let PokemonType::Normal(pokemon_name) = &basic {
        println!("The Normal Pokemon is {pokemon_name}")
    } else {
        println!("nothing")
    }

    let name = match basic {
        PokemonType::Normal(poke_name) => poke_name,
        _ => "Nothing".to_string(),
    };

    println!("name: {name}");
}
