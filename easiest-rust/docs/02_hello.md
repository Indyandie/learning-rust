# Print "Hello world!"

```rust
fn main() {
    println!("Hello, world!");
}
```

- `fn` denotes a function
- `main` is the name of the function that starts the program
- `()` contains variables
- `{}` is a code block
- `println!` is a macro, a function that writes code.
	- Macros have a `!` after the name. `macro_name!`

## Return something

```rust
fn number() -> i32 {
	8
}
```

- `->` skinny arrow, defines the type a function return
- exclude the trailing `;` to return something
	- Or write `return some_value;`

## Accept variables

```rust
fn multi(num1: i32, num2: i32) {
	let product = num1 * num2;
}
```


## Variables and Code Blocks

Variables start and end inside a code block `{}`.

```rust
fn main() {
	{
		let cb_num = 320
	};

	println!("cb_num is not accesible");
}
```

You can use code blocks to return values

```rust
fn main() {
	let cb_num = {
		let num = 8;
		num + 30
	};

	println!("code block num {}", cb_num)
}
```

## Display and debug

> Debug is printing for the programmer. 

- Use `{}` for display.
- Use `{:?}` to debug. 
	- Pretty print `{:#?}`, is `{:?}` with addtional formatting 

use `print!` to exclude new line at the end.

## Min & Max number for a type

`<type>::MIN` or `<type>::MAX`

```rust
print!("i8 smallest number {}, Largest number {}", i8::MIN, i8::MAX)
```

## Mutability

Variables declared with `let` are immutable.

```rust
fn main() {
	let num = 5;
	num = 10; // will not work
} 
```

Add `mut` after `let` to declare a variable as mutable. 

```rust
let mut num = 5;
num = 100;
```

The _type_ of a _variable_ cannot be changed.

```rust
let mut num = 5;
num = "five";
```

## Shadowing

Declaring a _variable_ with the same name as an _existing variable_ is referred to as _shadowing_. The _first variable_ is not destroyed but access is lost if it's in the same scope because it's blocked.

```rust
let num = 8;
println!("first instance of num {}", num);
let num = 100;
println!("the second instance of num {}", num);
```

If _variables_ with the same name exist in different scopes shadowing doesn't occor.

```rust
let num = 8;
println!("{}", num)
{
	let num = 39
}
```

Shadowning is helpful when a _variable_ needs to be changed multiple times.

```rust
let x = 10;
let y = 4;
let x = times_two(x);
x + y
```

