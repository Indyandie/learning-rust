# Destructuring

Both `structs` and `enums` can also be destructured. 

## Struct

```rust
struct Pokemon {
    id: u16,
    name: String,
    level: u8,
    r#type: Vec<String>,
}

fn main() {
    let bulbasaur = Pokemon {
        id: 1,
        name: "Bulbasaur".to_string(),
        level: 5,
        r#type: vec!["grass".to_string(), "poison".to_string()],
    };

    let Pokemon {
        id: a,
        name: b,
        level: c,
        r#type: d,
    } = bulbasaur;

    println!("{} {}: level {}, types - {}, {}", a, b, c, d[0], d[1])
}
```

```rust
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
    id,
    name,
    cost,
    attributes
  } = item;

  let item_strings = vec![name, &attributes[0]];
  println!("The item Strings are {:?}", item_strings);
}

fn main() {
  let pokeball = Item::new(1, "master-ball".to_string(), 3.5, vec!["holdable".to_string()]);
  prcess_item_strings(&pokeball);
}
```

## Enum

```rust
enum PokemonType {
    Normal(String),
    _Fire(String),
    _Water(String),
}

fn main() {
  let basic = PokemonType::Normal(format!("Meowth"));

  if let PokemonType::Normal(pokemon_name) = &basic {
      println!("The Normal Pokemon is {pokemon_name}")
  }

  let name = match basic {
      PokemonType::Normal(poke_name) => poke_name,
      _ => "Nothing".to_string(),
  };
}
```

