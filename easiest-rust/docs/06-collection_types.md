# Collection Types

Collections can be used when you need more than one value at time.

## Arrays `[]`

_Arrays_ are the simplest _collection type_, with the least functionality, they're very fast.

- Data is inside `[]`
- Must not change size or type
- All values must use same the type

The type of array is `[type; number]`. Example - the type of `["One", "Two"]` is `[&str, 2]`. Each _array_ can be a different type.

### Examples

> Rust will return the type when bad instructions are present. `array01.aslfjalsj()`

```rust
let arr1 = ["one", "two"]; // [&str, 2]
let arr2 = ["one", "two", "three"] // [&str, 3]
```

Declare an _array_ with all the same values.

> Commonly used to create buffers. 

```rust
let aaas = ["a"; 10]
let mut buffers = [0; 640]
```

## Array index

Get _array_ entries by referring to the index: `some_array[index]`. The index starts at `0`. 
```rust
let nums = [32, 89, -323];
println!("Nums 2nd entry is {}", nums[1])
```

## Slice

Get a portion of an array by slicing it. Use a `&` because the compiler doesn't know the size. Use `..` to set the range. `&some_array[3..7]`

- Index number start at `0`, not `1`
- Index ranges are exclusive (does not include the last entry number)

```rust
let ten = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let three_to_five = &ten[2..5];
let start_at_two = &ten[1..];
let end_at_five = &ten[..5];
let everything = &ten[..];

println!(
    "san-go: {:?}\nni: {:?}\ngo: {:?}\nALL: {:?} ",
    three_to_five,
    start_at_two,
    one_at_five,
    everything
)
```

### Inclusive Range

To include the last number in the range use `=` as follows: `&arr[0..=7]`

```rust
let ten = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let niroku = &ten[1..=5]; // 2nd to the 6th entry
```

---

## Vectors

_Vectors_ and _arrays_ have a similar relationship to that of `&str` and `String`. _Vectors_ are slower than _arrays_ and more functional. A _vector_ is declared with `Vec`. 

```rust
let a_str = String::from("A");
let b_str = String::from("B");

let mut a_vec = Vec::new();

a_vec.push(a_str); // Set to a Vec<String>
a_vec.push(b_str);
```

Defining a vector's type: `Vector<type>`
- `Vector<String>` a `Vec` of `Strings`
- `Vector<(i32, i32)>` a `Vec` of `tuples` that contain `i32`
- `Vector<Vector<String>>` a `Vec` of `Vec` of `Strings`

Declaring the vector type...

```rust
let d_vec: Vec<String> = Vec::new();
```

Using the `vec!` macro to declare a vector.

```rust
let m_vec = vec![31, 87, 10]; // Vec<i32>
```

### Slice Vectors

```rust
let vec_ten = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let sango = &vec_ten[2..6];
let ni = &vec_ten[1..];
let go = &vec_ten[..5];
let all = &vec_ten[..];

println!(
    "sango: {}\nni: {}\ngo: {}\nall: {}",
    sango, ni, go, all
)
```

### Capacity (speed it up)

A _vector_ has a specific space called **capacity**. When a _vector's_ capacity is filled it goes through **reallocation**. When this happenes the capacity will double and the _vector_ items will be moved to the new space. Use `.capacity()` to look at a _vector's_ capacity.


```rust
let mut a_vec = Vec::new();
println!("{}", a_vec.capacity()); // cap 0

a_vec.push("a");
println!("{}", a_vec.capacity());

a_vec.push("a");
a_vec.push("a");
a_vec.push("a");
println!("{}", a_vec.capacity());

a_vec.push("a");
println!("{}", a_vec.capacity());
```

The are two **reallocations** `0 to 4` and `4 to 8`. It can be made faster by defining a capacity to avoid multiple **reallocations**.

```rust
let mut c_vec = Vec::with_capacity(8);

c_vec.push('a');
println!("{}", c_vec.capacity());
c_vec.push('a');
println!("{}", c_vec.capacity());
c_vec.push('a');
println!("{}", c_vec.capacity());
c_vec.push('a');
println!("{}", c_vec.capacity());
c_vec.push('a');
println!("{}", c_vec.capacity());
c_vec.push('a');
println!("{}", c_vec.capacity());
```

### Array to Vector

Convert an array into a `Vec` using `.into()`

```rust
let arr = [22, 24, 32];
let d_vec: Vec<i8> = [1, 2, 3].into();
let e_vec: Vec<i8> = [32, 18, 99].into();
let arr_to_vec: Vec<i8> = arr.into();
```

## Tuples

_Tuples_ are defined using `()`. Functions use empty tuples when they don't receive any values and return and empty tuple if nothing is defined.

```rust
fn example() {};
// is short for
fn example() -> () {};
```

_Tuples_ can hold many things and different types too. Items are indexed with numbers (`0`, `1`, `2`). They are accessed using `.` instead of `[]`. `my_tuple.0`

```rust
let rnd_tup = ("hello", 8, vec!['a'], 'b', [8, 9, 10], 7.8);

println!(
    "# Tuple\n\n1. {:?}\n2. {:?}\n3. {:?}\n4. {:?}\n5. {:?}\n6. {:?}",
    rnd_tuple.0,
    rnd_tuple.1,
    rnd_tuple.2,
    rnd_tuple.3,
    rnd_tuple.4,
    rnd_tuple.5
)
```

The `tuple` above type is `(&str, i32, Vec<char>, char, [i32; 3], f64)`.


## Destructuring

_Tuples_ can be used for **destructuring**, using `_` to skip values.

```rust
fn main() {
    let str_vec = vec!["one", "two", "three"];

    let (a, b, c) = (str_vec[0], str_vec[1], str_vec[2]);
    println!(
        "a: {:?}, b: {:?}, c: {:?}",
        a, b, c
    )

    let (_, _, variable) = (str_vec[0], str_vec[1], str_vec[2]);
    println!("\nvarible {:?}", variable)
}
```
