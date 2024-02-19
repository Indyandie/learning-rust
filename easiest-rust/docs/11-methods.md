# Implementing Enum & Structs

Use the `impl` block to define `struct` and `enum` functions.

- **Methods**: take `self` (or `&self`, `&mut self`). _Regular methods_ use `.`, `.clone()` is an example.
- **Associated functions**: do not take `self`. _Associated functions_ use `::`, `String::from()` and `Vec::new()` are examples.

## Struct Example

```rust
#[derive(Debug)]
struct Pokemon {
    level: u8,
    name: Pokemons,
}

#[derive(Debug)]
enum Pokemons {
    Pikachu,
    Bulbasaur,
}

impl Pokemon {
    fn new() -> Self {
        // associated function
        // Self is Pokemon

        Self {
            level: 1,
            name: Pokemons::Pikachu,
        }
    }

    // regular methods

    fn change_to_pikachu(&mut self) {
        println!("Changing to pikachu...");
        self.name = Pokemons::Pikachu;
    }

    fn change_to_bulbasaur(&mut self) {
        println!("Changing to pikachu...");
        self.name = Pokemons::Bulbasaur;
    }

    fn level_up(&mut self) {
        self.level += 1;
        println!("This pokemon is now level {}", self.level)
    }

    fn who_is_that_pokemon(&self) {
        match self.name {
            Pokemons::Pikachu => println!("It's Pikachu, level {}", self.level),
            Pokemons::Bulbasaur => println!("It's Bulbasaur. level {}", self.level),
            _ => println!("Unknown pokemon. level {}", self.level),
        }
    }
}

fn main() {
    let mut poke = Pokemon::new();

    poke.who_is_that_pokemon();
    poke.change_to_bulbasaur();
    poke.who_is_that_pokemon();
    poke.level_up();
    poke.who_is_that_pokemon();
    poke.level_up();
    poke.level_up();
    poke.change_to_pikachu();
    poke.who_is_that_pokemon();
}
```

## Enum Example

```rust
enum PokemonType {
  Electric,
  Grass
}

impl PokemonType {
  fn check(&self) {
    match self {
      PokemonType::Electric => println!("Electric type."),
      PokemonType::Grass => println!("Grass type.")
    }
  }
}

fn main() {
  let electric = PokemonType::Electric;
  electric.check();
}
```
