## Type References 

If the type is not specified the compiler will the decide for you. 

```rust
fn main() {
	let num = 8; // i32
}
```

`i32` is the default for integers

### Specify Type

```rust
let num: u8 = 8 // u8
let num = 98u8 // u8
```

## Float

Fractions `4.4`.

```rust
let my_float = 4.4;
```

`f32` and `f64` are float types

```rust
let float_one: f64 = 5.0;
let float_two: f32 = 8.3;

// this will not work because the types are different 
let float_three: float_one + float_two; 

// this can be fixed by casting with the correct type
let float_three: float_one + float_two as f64; 

// or let rust decided by excluding the type
let float_one = 5.0; // rust will usaully choose f64 
let float_two = 8.3; 

let float_three: float_one + float_two; 
```
