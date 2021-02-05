use super::*;

impl Stacksize for char {
	fn get_max_stacksize(&self) -> usize {
		3
	}
}

#[test]
fn slotsize() {
	let mut inv = Inv::<char>::new(2);
	inv.place_at(('x', 1), 0);
	inv.place_at(('y', 3), 1);
	assert_eq!(inv.place_at(('z', 3), 3), Some(('z', 3)));
}

#[test]
fn slot_full() {
	let mut inv = Inv::<char>::new(2);
	inv.place_at(('x', 1), 0);
	assert_eq!(inv.place_at(('y', 3), 0), Some(('y', 1)));
}

#[test]
fn stacking() {
	let mut inv = Inv::<char>::new(2);
	assert_eq!(inv.stack(('x', 1)), None);
	assert_eq!(inv.get_slot(0).unwrap().1, 1);
	assert_eq!(inv.get_slot(1), None);
	assert_eq!(inv.stack(('x', 1)), None);
	assert_eq!(inv.get_slot(0).unwrap().1, 2);
	assert_eq!(inv.get_slot(1), None);
}

#[test]
fn stacksize_single() {
	let mut inv = Inv::<char>::new(2);
	assert_eq!(inv.place_at(('x', 10), 0), Some(('x', 7)));
}

#[test]
fn find_free() {
	let mut inv = Inv::<char>::new(3);
	assert_eq!(inv.place_at(('x', 2), 0), None);
	assert_eq!(inv.place_at(('y', 2), 2), None);
	assert_eq!(inv.place_at_first_free(('z', 3)), None);
	assert_eq!(inv.get_slot(1), Some(&('z', 3)));
}

#[test]
fn stacksize_multiple() {
	let mut inv = Inv::<char>::new(2);
	assert_eq!(inv.place_at(('x', 1), 0), None);
	assert_eq!(inv.place_at(('x', 1), 0), None);
	assert_eq!(inv.place_at(('x', 1), 0), None);
	assert_eq!(inv.place_at(('x', 1), 0), Some(('x', 1)));
	assert_eq!(inv.get_slot(0).unwrap().1, 3);
}

#[test]
fn stacksize_stack_overflow() {
	let mut inv = Inv::<char>::new(2);
	assert_eq!(inv.stack(('x', 3)), None);
	assert_eq!(inv.stack(('x', 1)), None);
	assert_eq!(inv.get_slot(1), Some(&('x', 1)));
}

#[test]
fn find_item() {
	let mut inv = Inv::<char>::new(3);
	assert_eq!(inv.place_at(('x', 1), 1), None);
	assert_eq!(inv.stack(('x', 1)), None);
	assert_eq!(inv.get_slot(1).unwrap().1, 2);
}

#[test]
fn selected_slot() {
	let mut inv = Inv::<char>::new(300);
	assert_eq!(inv.set_selected_slot(300), false);
	assert_eq!(inv.set_selected_slot(2), true);
	assert_eq!(inv.get_selected_slot(), None);
	assert_eq!(inv.place_at(('z', 2), 2), None);
	assert_eq!(inv.get_selected_slot(), Some(&('z', 2)));
}

#[test]
fn fill_with_stack() {
	let mut inv = Inv::<char>::new(4);
	assert_eq!(inv.stack(('x', 12)), None);
	for i in 0..4 {
		assert_eq!(inv.get_slot(i), Some(&('x', 3)));
	}
	// These are 12 too many
	assert_eq!(inv.stack(('u', 12)), Some(('u', 12)));
	// These are 12 too many too
	assert_eq!(inv.stack(('x', 12)), Some(('x', 12)));
}

#[test]
fn fill_with_stack_place_first() {
	let mut inv = Inv::<char>::new(4);
	assert_eq!(inv.place_at(('x', 1), 1), None);
	assert_eq!(inv.stack(('x', 11)), None);
	for i in 0..4 {
		assert_eq!(inv.get_slot(i), Some(&('x', 3)));
	}
}

#[test]
fn fill_with_place_at_first_free() {
	let mut inv = Inv::<char>::new(6);
	assert_eq!(inv.place_at_first_free(('z', 6)), None);
	assert_eq!(inv.place_at_first_free(('x', 6)), None);
	assert_eq!(inv.place_at_first_free(('z', 6)), None);
	for i in 0..2 {
		assert_eq!(inv.get_slot(i), Some(&('z', 3)));
	}
	for i in 2..4 {
		assert_eq!(inv.get_slot(i), Some(&('x', 3)));
	}
	for i in 4..6 {
		assert_eq!(inv.get_slot(i), Some(&('z', 3)));
	}
}

#[test]
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
	// It will return None, because the Item with its amount could successfully be placed in the inventory

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

	// You can also stack items quickly. Stack will look for an incomplete stack of items and fill it first.
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