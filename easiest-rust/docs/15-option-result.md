# Options & Results

## Options

The `Option` enum can be used to for values that might not exist, `Some(value)` if the it exist and `None` if it doesn't.

### Example

#### Bad Idea

This code will cause **Rust** to panic.

```rust
fn fifth(val: Vec:<i32>) -> i32 {
  val[4]
}

fn main() {
  let vec1 = vec![1, 2];
  let vec1_5 = fifth(vec1);
}
```

#### Good Idea

```rust
fn fifth_opt(val: Vec<i32>) -> Option<i32> {
  if val.len() < 5 { // .len() Vec length
    None
  } else {
    Some(val[4])
  }
}

fn main() {
  let vec1 = vec![1, 2];
  let vec2 = vec![9, 8, 7, 6, 5, 4, 3];

  print!("{:?}\n{:?}", fifth_opt(vec1), fifth_opt(vec2));
}
```

Use `.unwrap()` to get the value inside of a `Some(val)`. Does not work with `None`, it doesn't contain a value.

The `.unwrap()` function is not the only way to get the value from a `Some()`, a `match` could be used access the value.

```rust
fn fifth_opt(val: Vec<i32>) -> Option<i32> {
  if val.len() < 5 {
    None
  } else {
    Option(val[4])
  }
}

fn print_opts(opts: Vec<Options<i32>>) {
  for opt in opts {
    match opt {
      Some(num) => print!("Some value: {num}"),
      None => print!("Nothing.")
    }
  }
}

fn main() {
  let vec1 = vec![1, 2];
  let vec2 = vec![9, 8, 7, 6, 5, 4, 3];

  let mut vec_opts = Vec::new();
  vec_opts.push(fifth_opt(vec1));
  vec_opts.push(fifth_opt(vec2));

  print_opts(vec_pts);
}
```

### Option Signature

```rust
enum Option<T>{
  None,
  Some(T),
}

fn main(){}
```

- Some uses a _generic type_
- No trait limits
- None has no value

### `.is_some` Method

The `.is_some` method return `true` if the `Option` is a `Some()` with a value.

```rust
for vc in opt_vec {
  let vec_5 = vec_fifth(vc);

  if vec_5.is_some() {
    print!("{}", vec_5.unwrap());
  } else {
    print!("Nothing!")
  }
}
```
