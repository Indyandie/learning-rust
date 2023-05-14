# Enums

- (rust enums)[https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/enums.html]
- (easy-rust)[https://dhghomon.github.io/easy_rust/Chapter_25.html]

An `enum` (enumartion) is a type which represents data that's one of several variants (choices). They resemble structs but are very different.
- Struct provides a way to create complex data types, Example a DND character
- Enum provides a way to represent data variants, Example a DND character's race and class

## Declaring an `enum`

Variants can optionally store a data type. Each variant can store a different data type or none. This is why `enums` are sometimeis referred to as `sum types`. 

```rust
enum TimeOfDay {
    Morning,
    Afternoon,
    Evening,
    Night,
    Midnight,
}

enum Words {
    Limerick(String),
    Poem(String),
    Story(String),
}

fn main() {
   let morning = TimeOfDay::Morning; 
   let limerick = Words::Limerick("Oh Johnny boi the pipes are calling".to_string())
}
```

## Import Enums

By importing the `enum` to the function you omit `EnumName::` and refer directly to the variant. 
```rust
enum Seasons {
    Winter,
    Spring,
    Summer,
    Fall,
}

fn main() {
    use Seasons::*

    let cold_season = Winter;
}
```

## Enum Variants

If an enum variant doesnt store data it can be cast as an int to obtain the index. You can assign this manually `Variant = 32`, if another variant is add after it will be `33`.

```rust
enum Binary{
    Zero,
    One
}

fn main() {
    use Binary::*

    let zero = Zero as i32 // 0
}
```
