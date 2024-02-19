# Generics

_Generic type_ to support multiple types for a functions, for example `i32`, or `f16`.

## Format

_Generic types_ are defined after the function name with angled brackets (`<Generic_Type_Name>`) ussually a single capital letter (`T`, `U`, `V`, etc.), normally `<T>`.

```rust
fn gen_type <T> (num: T) -> {
	print!("value: {num}");
	num
}
