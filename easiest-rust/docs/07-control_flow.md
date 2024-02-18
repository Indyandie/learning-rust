# Control Flow

The order in which individual statements, instructions, or function calls of an imperative program are executed or evaluated.

## `if`

The simplest control flow is `if`.

```rust
let num = 5;

if num == 7 {
    println!("It's seven")
} else if num == 6 {
    println!("It's six")
} else {
    println!("not seven or six.")
}
```

### Logical operators

- `&&` AND
- `||` OR

```rust
let num = 9

if num % 2 == 0 && num % 3 == 0 {
    println!("Divisible by 2 and 3.")
} else if num % 2 > 0 || num > 4 {
    println!("Is even or greater than 4")
} else {
    println!("Who knows?")
}
```

## `match`

Use **match** for a simpler format when **if else** becomes too unwieldy. Every result must be considered for a **match**.

- format is `match` followed by a _code block_ `{}`.
- Each line is called an _arm_, they are separated by `,`
- _Arm_ format: pattern/conditions, fat arrow `=>`, and instructions.

```rust
let num: u8 = 16;

match num {
    0 => println!("It's zero."),
    1 => println!("It's one."),
    2 => println!("It's two."),
    // catch all. Every other number that's not specified. 
    _ => println!("It's some other number."),
}
```

Declare a value with a **match**. 

```rust
let num: u8 = 32;

let match_num = match num {
    0 => 0,
    5 => 10,
    _ => 2,
};
```

More complicated _tuples_

```rust
let sky = "cloudy";
let temp = 30;

match (sky, temp) {
    ("cloudy", 40) => println!("It's cloudy and cold."),
    ("clear", 70) => println!("It's a nice day."),
    ("cloudy", 70) => println!("It's cloudy and warm."),
    _ => println!("I am not sure."),
}
```

### `if` and `match`

```rust
let kids = 5;
let married = true;

match (kids, married) {
    (kids, married) if married == false && kids > 0 => println!("Not married with {} kids.", kids),
    (kids, married) if kids == 0 && married == true => println!("Married and no children."),
    _ => println!("Married? {}. Number of kids {}.", married, kids),
}
```

### Using multiple `_`

```rust
fn match_colors(rgb: (i32, i32, i32)) {
    match rgb {
        (r, _, _) if r < 10 => println!("Not much red"),
        (_, g, _) if g < 10 => println!("Not much green"),
        (_, _, b) if b < 10 => println!("Not much blue"),
        _ => println!("Each color has a least 10.")
    }
}

fn main() {
    let red = (200, 0, 0);
    let green = (0, 200, 0);
    let blue = (0, 0, 200);

    match_colos(red);
    match_colos(green);
    match_colos(blue);
}
```

`match` stops after it has found the first match.

### Same arms

The arms in a `match` must each return the same value type.

```rust
let num = 10;
let sum_var = match num {
    10 => 8,
    _ => "Not ten",
    // arms don't match 
}
```

`if` and `else` behave in the same way. Each scenario must output the same value type.

```rust
let num = 8;

let sum_var = if num == 10 { 8 } else { "some string" };
```

Using a different `let` statement will work.

```rust
let num = 23;

if num == 10 {
    let sum_var = 8;
} else { 
    let sum_var = "some string";
};
```

A `match` value can be aliased with `@`. 

```rust
fn match_num(input: 132) {
    match input {
        num @ 4 => println!("{} is unlucky in China.", num),
        num @ 13 => println!("{} is unlucky in Italy.", num),
        _ => println!("Some basic number."),
    }
}

fn main() {
    match_num(50);
    match_num(13);
    match_num(4);
}
```

