# Stack, Heap, & Pointers

The stack and heap are places where memory is stored.

- **Stack** is very fast, a lot faster than the **heap**.
- Rust need to know the size of a variable at compile time. Simple variables like `i32` go on the stack, since exact size is known. `i32` is 4 bytes
- Some type have an unknown size at compile. First the data goes on the **heap** since it can have a variable  size. Then after a **pointer** is assigned it goes on the **stack**. 

Pointers are like a table of contents in a book. A pointer in rust is usually called a **reference**. A **reference** points to the memory of another value. The value is borrowed but not owned. **References** have a `&` in front of them. 

```rust
let reg_var = 89; // regular variable
let ex_reference = &reg_var; // make a reference
```

A reference can reference a reference.

```rust
let num = 5;
let sing_ref = &num;
let double_refs = &sing_ref;
let five_refs = &&&&double_refs;
```

## More printing

`\n` will make a new line, `\t` will make a tab

```rust
print!("# Hello\n\t- one\n\t- two")
```

Inside `""` you can write over many lines, whitespace and line breaks are accounted for

```rust
print!("# Hello
	- one
	- two
	- three
Done")
```


### Output 

```md
# Hello
	- one
	- two
	- three
Done
```

To ignore `"` and other escaped characters use `r#` at the beginning and `#` at the end.

```rust
println!(r#"quotes will print", \n and \t will print too.#)
```

To include `#` add a `#` at the beginning and the end.

```rust
println!(r##"ID: #00000324"##)
```

`r#` can be used to name variables with keywords (`fn`, `let`, `mut`)

```rust
let r#let = "let";
let mut r#mut = "mut";

fn r#return() {
	"return"
}

fn main() {
	let keyword_return = r#return;
	print!("{}", keyword_return)
}
```

To print the bytes of a `char` or `&str`, write `b` before the string. It works for all `ASCII`. For a `char` it called a **byte**, and a **byte string** for a `&str`

```
☺☻♥♦♣♠♫☼►◄↕‼¶§▬↨↑↓→∟↔▲▼123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~
```

```rust
println!("{:?}", b"abcDEF!?.,")
```

You can combine `b` with `r#`. 

```rust
print!("{:?}", br##"/,'"', and #."##)
```

Use the unicode escape `\u{}` to print Unicode characters inside a string `"\u{<Unicode-character>}"` 

```rust
    println!("unicode char \u{48}");
```

`println!`
- `{}` display
- `{:?}` debugs
- `{:#?}` pretty printing
- `{:p}` prints pointer address, location in computer's memory
		```rust
		let num = 8;
		let num_ref = &num;
		println!("pointer address for num: {:p}", num_ref);
		```
- `{:b}` binary
- `{:x}` hexadecimal
- `{:o}` octal


## Print Order

Use numbers to change print order, index starts at `0`

```rust
let fam = "Kon";
let dad = "Tachi";
let son = "Katachi"
println!("\nThis is {2} {0}, son of {1} {0}", fam, dad, son)
```

Name print variables for easier access.

```rust
println!(
	"{city1} is in {country}, and {city2} is in {country} too. {city3} is not in {country}",
	city1 = "Austin",
	city2 = "Houston",
	city3 = "Newark",
	country = "Texas"
)
```

## Complex Printing

```
{<var_name>:<padding_char><alignment><min_len>.<max_len>}
```

`{:-^11}` 
- `-` padding char, 
- `^` alignment (centered), 
- `11` minimum length


```rust
let title = "TODAY'S NEWS";

// no variable name, pad it with -, centered, 30 characters long
println!("{:-^30}", title); 

let bar = "|";

// no variable name, pad with space, 15 characters each, one to the left, one to the right
println!("{: <15}{: >15}", bar, bar); 
let a = "SEOUL";
let b = "TOKYO";

// variable names city1 and city2, pad with -, one to the left, one to the right
println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b); 
```

