# inv-sys
A simple and effective inventory system for games.

## Features
- simple API
- automatic stacking functionality
- slot amount per inventory
- max stacksize via trait

## Usage

```rust
// Implement the Stacksize Trait for your Type that will act as your Item
impl super::Stacksize for char {
	fn get_max_stacksize(&self) -> usize {
		3
	}
}

fn main() {
	// Create the Inventory with a slotsize of 4
	let mut inv = Inv::<char>::new(4);
	// inv-sys works with tuples of the item and an amount.
	// place_at is used to try to place an item at an exact slot
	assert_eq!(
		//Try to place 1 'x' in slot 0
		inv.place_at(('x', 1), 0), 
		None
	);
	// It will return None, because the Item with its amount 
	// could successfully be placed in the inventory

	// See what happens if you try to stack more items, than possible:
	assert_eq!(
		inv.place_at(('x', 3), 0), 
		Some(('x',1))
	);
	// Yes, you have seen correctly! You got one Item back, 
	// because the other ones were used to fill up the slot! 

	// You can get an item from a slot like so:
	// (and we can make sure there are really three Items in slot 0)
	assert_eq!(
		inv.get_slot(0), 
		Some(&('x', 3))
	);

	// You can also stack items quickly. 
	// Stack will look for an incomplete stack of items and fill it first.
	// It will then try to fill the next empty slots, beginning from the start
	assert_eq!(
		inv.stack(('x', 3)), 
		None
	);

	// Fresh Start
	inv = Inv::<char>::new(4);
	// place_at_first_free will only look for free slots, and fill them.
	inv.place_at(('x', 1), 0);
	inv.place_at_first_free(('x', 2));
	assert_eq!(
		inv.get_slot(1), 
		Some(&('x', 2))
	);

	// You can also set a selected slot
	inv.set_selected_slot(1);
	// And decrease the Item Counter
	inv.decrease_selected_slot();
	assert_eq!(
		inv.get_slot(1), 
		Some(&('x', 1))
	);
	// The Item will then become None at amount 0
	inv.decrease_selected_slot();
	assert_eq!(
		inv.get_slot(1), 
		None
	);
}
```

## Todo
 - Better way to handle item remove/decrease
 - Finding items
 - Retrieving Slots
 - Iterator
 - Trees for sorting and performance improvements
 - Simple configuration options
 - Any requests? Please just submit an issue, thanks!

## Contributions
Feel free to open an issue/PR explaining possible improvements or changes

## Help
Also, please do not hesitate and open an issue when needed. I am happy to help!
