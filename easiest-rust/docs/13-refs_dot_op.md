# References & Dot Operator

Normally, to access the value from a _reference_ (`&value`) the `*` is used (`*value`) to _dereference_. 

```rust
fn main() {
  let num = 8;
  let num_ref = &num;

  // this will not compile, can't compare u8 and &u8
  // println!("{}", num == num_ref);
  println!("{}", num == *num_ref);
}
```

But when a method is used, Rust will dereference. The dot operator (`.`) in a method dereferences the value.   

```rust
struct Item {
  num: u8
}

fn main() {
    let item = Item { num: 8 };
    let ref_item_num = &item.num; // &u8 type

    // cannot compare &u8 and u8
    // println!("{}", ref_item_num == 8);
    println!("{}", *ref_item_num == 8);

    //using the dot oparator 
    let ref_item_num_dot = &item;

    // no ref required
    println!("{}", ref_item_num_dot.num == 8);
}
```

## Method Example

```rust
struct Item {
  num: 8
}

impl Item {
  fn compare_num(&self, other_num: u8) {
    println!("{} and {} are equal, {}.", self.num, other_num, self.num == other_num);
  }
}

fn main() {
  let item = Item{ num: 8, };

  let ref_item = &item;
  let ref_item_deep = &ref_item;

  item.compare_num(8);
  ref_item.compare_num(8);
  ref_item_deep.compare_num(8);
}
```
