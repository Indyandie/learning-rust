# Copy Types

In **Rust** `copy types` are very simple types that exist on the _stack_ and the compiler knows there size.

> The compiler always copies `copy types` when they sent into a function because they're so small. 

## Types

- Integers
- Floats
- Booleans (`true` & `false`)
- `char`

The _copy trait_ can be found in the type documentation under trait implementations **Copy**. 

### Example [`char`](https://doc.rust-lang.org/std/primitive.char.html).

- **Copy** copied when sent into a function
- **Display** can use `{}`
- **Debug** can use `{:?}`

```rust
fn prt_num(num: i32) {
    println!("{}", num);
}

fn main() {
    let my_num = 8;
    prt_num(my_num);
    prt_num(my_num); // no ref or returns needed
}
```

### Clone trait

`String` is not a _copy type_ but it has the **Clone** trait which can be used instead of a reference to pass the value into a function.

Use `string_var.clone()` to create a clone.

```rust
fn pr_str(str_var: String) {
    println!("{}", str_var);
}

fn main() {
    let my_str = String::from("A string.");
    pr_str(my_str.clone());
    pr_str(my_str) // now it's dead
}
```

> `.clone()` copies the value each time it's used. It can use a lot of memory. Using a reference can be faster and use less memory.

```rust
fn get_len(input: String) {
    println!("Length is {} word(s)", input.split_whitespace().count());
}

fn get_len_ref(input: &String) {
    println!("Length is {} word(s)", input.split_whitespace().count());
}

fn main() {

    // Using clones
    let mut my_str = String::new();
    for _ in 0..50 {
        my_str.push_str(", more words");
        get_len(my_str.clone());
    }

    // Using refs
    let mut my_str = String::new();
    for _ in 0..50 {
        my_str.push_str(", more words");
        get_len_ref(&my_str);
    }
}
```

## Unitialized Variables

A variable without a value.

> **Rust** wont compile if something is unitilized. 

```rust
let un_var; 
```

### Uses

If a _code block_ contains a _variable_ that needs to exist outside of the _code block_.

```rust
fn loop_rtn(mut count: i32) {
    loop {
        count += 1;
        if count % 50 == 0 {
            break;
        }
    }
    count
}

fn main() {
    let my_num;

    {
        let num = {
            57    
        };

        my_num = loop_rtn(num)
    }

    println!("{}", my_num);
}
```

`my_num` is declared in `main` so it only exist until the end of the function, and gets it value from inside the _code block_.

> Simpler version
>
```rust
fn main() {
    let my_num;
    {
        my_num = 100;
    }

    println!("{}", my_num)
}
```

It's almost like saying `let my_num = { 100 };`, `my_num` is not `mut`. But it never changed it's value cause it didn't have one to begin with.
