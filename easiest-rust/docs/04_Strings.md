# Strings

Two type of strings, `String` and `&str`. Both support `UTF-8` and emojis. Each is closely related. 

```rust
let name = "서태지 😂"; // &str
let other = String::from("Hello! 😂"); // String
```

`&str` is a simple and _fast string_. It is used with a reference `&` because the _stack_ need to know the size. Not an owned type.

```rust
let my_str = "this is a &str"
```

`String` is more functional but is slower than `&str`. It's a pointer with data on the _heap_. It's an owned type.

`&str` is a dynamically sized type. Different values can be a different size. Because of this `&str` needs a pointer because **Rust** knows it's size.


### Creating a String

```rust
// String that takes text and creates a String
let string_1 = String::from("This is a string of text.");

// make &str into a String
let string_2 = "This is a string of text".to_string();

// format macro
let name = "string"
let string_3 = format!("This is a {} of text", name);

// .into() the String type must be specified
let string_4: String = "This is a string of text".into();
```

## const & static

Values that don't change are declared with `const` or `static`. The type is not inferred for these declarations.

> THey are both very similar except that `static` has fixed memory location and can act a as global variable. 

> `const` is most commonly used.

They are written in SCREAMING_SNAKE_CASE outside of `main`.

```rust
const NUMBER_OF_MONTHS: u8 12;
static SEASONS: [&str, 4] = ["Sprint", "Summer", "Fall", "Winter"]
```

## More on refs

The references below have a type of `&String`. Hundreds of ref can be created.

```rust
let country = String::from("Japan");
let ref_one = &country;
let ref_two = &country;

println!("{}", ref_one);
```

The problem...

```rust
fn rtn_str() -> String {
    let country = String::from("Japan");
    let country_ref = &country;
    country_ref // This is a problem
}

fn main() {
    let country = rtn_str();
}
```

The code above won't work because the referenced `String` only lives inside the function code block, and then it dies. It is erased from memory and can't be referenced. This happens because there is no ownership.

## Mutable Reference

Using a reference to change data.

```rust
// `num` is `i32` and `num_ref` is `&mut i32` type.

let mut num = 8; // the variable must be mutable
let num_ref = &mut num;
```

cannot `num_ref += 10` cause the types are different `num_ref` is an `&i32`. The type can be adjust using `*`. `*` will use the direct value being referenced.

```rust
*num_ref += 10;
println!("{}", num);


let num2 = 800;
let ref3 = &&&second;
println!("re3 is equal to num2? {}", num2 == ***ref3);
```

This method using the `*` is called dereferencing.

### Rules for mutable and immutable (shared) references

1. If only immutable references, there is no limit.

```rust
let num = 32;
let num_ref_1 = &num;
let num_ref_2 = &&num;
let num_ref_3 = &&&num;
let num_ref_4 = &&&&num;
```

2. Only 1 mutable reference is allowed.

```rust
let mut num = 32;
let num_ref_1 = &mut num;
*num_ref_1 = 64;
print!("mutable ref: {}", num_ref_1);

// this is not allowed
let num_ref_2 = &mut num;
```

3. Immutable and mutable cannot be referenced together.

```rust
let mut num = 32;

// only one type can exist
let num_mut_ref = &mut num;
let num_imut_ref = &num;
```

## More Shadowing

Seeing how shadowing work with references. Shadowing doesn't destroy the value but blocks it.

```rust
let city = String::from("Town");
let city_ref = &city;
let city = 100;
println!("og ref{}, shadow {}", city_ref, city);
```

## Passing References to functions

**Rust** rule: values can only have _1 owner_.

```rust
fn pr_str(pr_str: String) {
    println!("{}", pr_str)
}

fn main() {
    let my_string = String::from("hello");
    pr_str(my_string); // success - pr_str becomes the new owner of my_string
    pr_str(my_string); // error: my_string was destroyed
}
```

### The flow

1. `my_string` `String` is created, `my_string` is the **owner**
2. `my_string` is passed to `pr_str()` which doesn't return anything, after `pr_str` is done the `my_string` `String` is dead. 
3. Cannot pass `my_string` to `pr_str()` because it's dead.

> You can pass return the value back from `pr_str()` and shadow the variable. 

```rust
fn pr_str(pr_str: String) -> String {
    println!("{}", pr_str);
    pr_str
}

fn main() {
    let my_string = String::from("hello");
    let my_string = r_str(my_string); // success
    pr_str(my_string);  // succes
}
```

### Using references with functions

`pr_str` will take a `String` referenece

```rust
fn pr_str(pr_str: &String) {
    println!("{}", pr_str);
}

fn main() {
    let my_string = String::from("hello");
    pr_str(&my_string);
    pr_str(&my_string);
}
```

### Mutubale Refs

```rust
fn add_str(add_str: &mut String) {
    add_str.push_str("-more_text");
    println!("{}", add_str);
} 

fn main() {
    let mut my_string = String::from("A string");
    add_str(&mut my_string);
    add_str(&mut my_string);
}
```

### Review

- `variable: String` - takes a `String` and owns it. Doesn't return variable and it dies.
- `variable: &String` - borrows a `String` and can look at it.
- `variable: &mut String` - borrows a string and can change it. 

## Passing a Mutuable Variable

```rust
fn main() {
    let my_string = String::from("A string");
    add_word(my_string);
}

fn add_word(mut da_str: String) {
    da_str.push_str("-added-word");
    println!("{}", da_str)
}
```

