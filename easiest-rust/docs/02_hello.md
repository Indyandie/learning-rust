# Print "Hello world!"

```rust
fn main() {
    println!("Hello, world!");
}
```

`fn` is a function
`main` is the function that starts the program
`()` contained variables
`{}` is a code block
`println!` is a macro, a function that writes code. They have a `!` after the name.

## Return something

```rust
fn number() -> i32 {
	8
}
```

`->` skinny arrow, show what a function returns
exclude `;` to return something
or write `return something;`

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
	}:

	println!("cb_num is not accesible")
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

Debug is printing for the programmer. 

Use `{}` for display. Use `{:?}` to debug. 

Pretty print `{:#?}`, is `{:?}` with addtional formatting 

use `print!` to exclude new line at the end.

## Min & Max number for a type

`<type>::MIN` or `<type>::MAX`

```rust
print!("Smallest number {}, Largest number {}", i8::MIN, i8::MAX)
```

## Mutability

Variable declared with `let` are immutable.

```rust
fn main() {
	let num = 5;
	num = 10; // will not work
} 
```

Add `mut` after `let` to mutable. 

```rust
let mut num = 5;
num = 100;
```

You cannot change the type.

```rust
let mut num = 5;
num = "five";
```

## Shadowing

Declaring a variable with the same name as an existing variable. The first variable is not destroyed but we wont have access if it's in the same scope because it's blocked.

```rust
let num = 8;
println!("num og {}", num);
let num = 100;
println!("num og {}", num);
```

If in different scopes we can see both.

```rust
let num = 8;
println!("{}", num)
{
	let num = 39
}
```

Shadowning is helpful when it needs to be changed multiple times.

```rust
let x = 10;
let y = 4;
let x = times_two(x);
x + y
```

