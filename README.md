# `derive_lit`

[![](http://meritbadge.herokuapp.com/derive_lit)](https://crates.io/crates/derive_lit)
[![](https://docs.rs/derive_lit/badge.svg)](https://docs.rs/derive_lit)

Are you developing a data structure?

```rust
struct GroceryList {
	num_items: usize,
	items: Vec<String>
}
```

And your data structure lets you add data to it?

```rust
impl GroceryList {
	fn new() -> Self {
		Self {
			num_items: 0,
			items: vec![]
		}
	}

	fn push(&mut self, item: String) {
		self.push(item);
	}
}
```

Wouldn't it be cool if you could do this?

```rust
fn main() {
	let groceries = grocery_list![
		"Milk",
		"Bread",
		"Apples",
		"Flour"
	];

	// do something intersting with your GroceryList...
}
```

What if you could just...

```rust
use derive_lit::VecLit;

#[derive(VecLit)]
struct GroceryList {
	num_items: usize,
	items: Vec<String>	
}
```

You can! Use `derive_lit::*`.
Just a `derive_lit = "0.1.0"` away!

