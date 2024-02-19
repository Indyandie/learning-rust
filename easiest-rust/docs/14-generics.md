# Generics

_Generic type_ to support multiple types for a functions, for example `i32`, or `f16`.

## Format

_Generic types_ are defined after the function name with angled brackets (`<Generic_Type_Name>`) ussually a single capital letter (`T`, `U`, `V`, etc.), normally `<T>`.

```rust
fn gen_type <T> (num: T) -> {
	print!("value: {num}");
	num
}
```

## Concrete vs. Generic Type

### Concrete Type

Without the angled bracket defintion Rust will inteprate the type as _concrete type_, a single type. 

```rust
fn con_type (num: ExType) -> ExType {
	print!("Concrete type");
	num
}
```

### Generic Type

```rust
fn gen_type <ExTyp> (num: ExType) -> ExType {
	print!("Concrete type");
	num
}
```

## Traits

To use `Display`, `Debug`, `Copy` or other _traits_ with a _generic type_ the desired trait must be specified with the _generic type_ definition. 

```rust
use std::fmt::Debug;

#[derive(Debug)]
struct Animal {
	name: String,
	age: i8
}

fn debug_item <T: Debug> (item: T) {
	println!("Debug item: {:?}");
}

fn main() {
	let gary = Animal {
		name: "Gary".to_string(),
		age: 40
	};

	let num = 45;
	debug_time(gary);
	debug_time(num);
}
```

## Multiple Generic Types

- `T` - `1` variable, `Display`
- `U` - `2` varaibles, `Display`, `PartialOrd`

```rust
use std::fmt::Display;
use std::cmp::PartialOrd;

fn compare_display <T: Display, U: Display + PartialOrd> (msg: T, num1: U, num2: U) {
	let comparison = num1 > num2;

	println!("{msg}\n\nIs {num1} greater than {num2}? {comparison}");
}

fn main() {
	compare_display("Yo!", 23, 22);
}
```

### Where

The `where` keyword can be used to define very multiple generic types and complex traits. 

```rust
fn where_func <T, U, V, W> (name: T, id: U, msg: V, list: W) 
	where 
		T: Display,
		U: Display,
		V: Display,
		W: Debug,
{
	println!("{name}\n\t{id}\n\t{msg}\n")
	print!("{:?}", list)
}
```

## Rules

- A defined _generic type_ that's used multiple times within the same function in the same call must used the same types. 
- Different _generic types_ within a function can used the same or different types.
