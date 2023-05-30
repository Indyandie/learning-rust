# Loops

Loop iterate through a process. There are 3 types of loops, `loop`, `while`, and `for`.

## Loop

A `loop` will repeat indefinitely unless there is terminating logic to break the loop. 

```rust
// infinite loop
loop {
    println!("Eternity is forever...");
}

let mut loop_counter = 0;

loop {
    loop_counter += 1;
    println!("loop counter: {}", loop_counter);
    if loop_counter > 6 {
        break;
    }
}
```

### Loop Labels

> A `break` or `continue` will affect the inner most loop by default.

Embedded loops can be named with loop labels using a leading `' single quote` and a trailing `:`, for example `'loop_name:`. Then a `break` can be a applied to a specific loop. 


```rust
let mut loop_counter_1 = 0;
let mut loop_counter_2 = 0;

'loop_1: loop {
    loop_counter_1 += 1;
    println!("counter 1: {}", loop_counter_1);

    if loop_counter_1 > 9 {
        println!("Entering loop 2");

        'loop_2: loop {
            println!("counter 2: {}", loop_counter_2);
            loop_counter_2 += 1;

            if loop_counter_2 > 5 {
                break 'loop_1;
            }
        }
    }
}
```

## While Loop

A `while loop` repeats while a condition is met. Use a `while loop` when the number of repetitions is unknown. 

```rust
let mut while_counter = 10;
let mut while_condition = false; // bool

while !while_counter {
    while_counter -= 1;

    println!("counter: {}", while_counter);

    if while_counter < 1 {
        while_condition = true;
    }
}
```

## For Loop

A `for loop` repeats a particular number of times. Ranges are commonly used with `for loops`, using `..` or `..=`.  

```rust
for number in 0..3 { // esclusive
    println!("number: {}", number);
}

for num in 0..=10 { // inclusive
    println!("numver: {}", num);
}

for _ in 0..8 {
    println!("Repeat, no variable.");
}
```

### Enumerate 

The `enumerate` function can be be used to keep track of the number of loops. 

#### Ranges

> The range must be wrapped in `()` parentheses.

```rust
for (index, value) in (2..32).enumerate() {
    println!("index = {}, value = {}", index, value)
}
```

### Iterators

```rust
let lines = "hello\nworld".lines();

for (linenumber, line) in lines.enumerate() {
   println!({}, {}, linenumber, line);
}
```

## Break & Return 
 
A `break` can be used to return a value, including the value after the `break` followed by `;`.

```rust
let mut loop_counter = 0;

let return_value = loop {
    loop_counter += 1;
    if loop_counter % 53 == 3 {
        break loop_counter;
    }
}

println!("The return value is {}.", return_value)
```

## Match Color Loop

```rust
fn color_comp(rgb: (i32, i32, i32)) {
    let (red, green, blue) = rgb;
    println!("Checking color composition of red {}, green {}, and blue {}.", red, blue, green);
    let colors = vec![(red, "red"), (green, "green"), (blue, "blue")]

    // check min  10
    let mut all_colors_min_10 = true;
    for color in colors {
        if color < 10 {
            all_colors_min_10 = = false;
            println!("Not much {}.)", color.1);
        }
    } 

    if all_colors_min_10 {
        println!("Each color has at leas 10.\n");
    }

}

fn main() {
    let red = (200, 0, 0);
    let gray = (50, 50, 50);
    let redish = (200, 50, 0);

    color_comp(red);
    color_comp(gray);
    color_comp(redish);
}
```
