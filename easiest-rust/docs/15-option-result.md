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

## Result

The `Results` enum is similar to `Option` but instead of the existance of value it test the outcome of a scenario.

```rust
enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

Both `Ok` and `Err` return a value.

```rust
fn chck_err() -> Result<(), ()> {
  Ok(()) // warning
}

fn even(in: i32) -> Result<(), ()> {
  if in % 2 == 0 {
    return Ok(())
  }
}

fn main(){
  if even(5).is_ok() {
    println!("Even!")
  } else {
    println!("Odd...")
  }
}
```

> Methods `is_some`, `is_none()`, `is_ok()`, and `is_err()`.

```rust
fn chk_5(num: i32) -> Result<i32, String> {
  match num {
    5 => Ok(num),
    _ => Err("Sorry".to_string()),
  }
}

fn main(){
  let mut res_vec = Vec::new();
  for num in 2..9 {
    res_vec.push(chk_5(num));
  }

  printls!("{res_vec:?}")
}

```

> `.unwrap` on `Err` will cause a panic

```rust
let err_val(vec: Vec<u8>) -> Result<String, FromUtf8Error> {
  // do somet
}
```

Using a `match` with `Option` or `Result` can wordy. The `.get()` method will return and `Option` for a `Vec`.

```rust
let vec1 = vec![2, 3];
let get1 = vec1.get(0);
let get2 = vec1.get(1);

print("{get1:?}\n{get2:?}")
```

Use range.

```rust
fn main(){
  let vec = vec![2, 3, 4, 6, 5]

  for idx in 0..10 {
    match vec.get(idx) {
      Some(num) => print!("{num}"),
      None => {},
    }
  }
}
