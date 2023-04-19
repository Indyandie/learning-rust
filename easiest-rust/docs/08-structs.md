# Structs

Structs are used to create new types.  They are created with keyword `struct`, and should be `PascalCased`.

There are 3 type of **structs**.
- unit
- tuple
- named

## Unit Structs

Unit structs are empty.

```rust
struct UnitStruct;
```

## Tuple Struct

Also known as unnamed structs. To define tuple struct only the types are required.

```rust
struct CoolColors (String, u8, u8, u8);

fn main () {
    let blue = ("Blue".to_string(), 0, 0, 100);

    println!("{} is a cool color.", blue.0);
}
```

## Named Structs

To define a name struct a name and type are required for each property. 

```rust
struct Color (u8, u8, u8)

struct ColorNames {
    name: String,
    color: Color,
}

fn main () {
    let red = Color(255, 0, 0)
    
    let color_red = ColorNames {
        name: "Red".to_string(),
        color: red, // optional trailing comma
    }
}
```

If a variable has the same name as the struct's property, the name can be written once.

```rust
struct Pokemon {
    name: String:
    pokadex_number: u16,
    r#type: Vec<String>, 
}

fn main () {
    let name = "Bulbasaur";
    let pokadex_number = 1;
    let grass = "Grass".to_string();
    let poison = "Poison".to_string();

    let bulbasaur = Pokemon {
        name,
        pokadex_number,
        r#type: vec![grass, poison]
    }
}
```

# Debug

To debug with `{:?}` within the `println` macro add `#[derive(Debug)]` above the struct declartion.

```rust
#[derive(Debug)]
struct BuggyStruct(u8, String, u32);

fn main () {
    let buggy = BuggyStruct(88, "ABC".to_String, 3242);

    println!("Debug {:?}", buggy)
}
```
