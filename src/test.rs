use super::*;

impl Stacksize for char {
	fn get_max_stacksize(&self) -> usize {
		3
	}
}

#[test]
fn auto_stack() {
	let mut inv = Inv::<char>::new(4);
	assert!(
		inv.auto_stack(ItemStack::new('c', 6)).is_ok()
	);

	// 2*3 = 6
	assert_eq!(
		inv.get_slot(0).unwrap().get_amount(), 
		Ok(3)
	);
	assert_eq!(
		inv.get_slot(1).unwrap().get_amount(), 
		Ok(3)
	);
}

#[test]
fn auto_stack_filled() {
	let mut inv = Inv::<char>::new(4);

	inv.auto_stack(
		ItemStack::new('c', 1)
	).ok();
	inv.auto_stack(
		ItemStack::new('a', 6)
	).ok();
	inv.auto_stack(
		ItemStack::new('c', 5)
	).ok();

	// 3c + 3a + 3a + 3c
	assert_eq!(
		*inv.get_slot(0).unwrap(), 
		Slot::new(ItemStack::new('c', 3))
	);
	assert_eq!(
		*inv.get_slot(1).unwrap(), 
		Slot::new(ItemStack::new('a', 3))
	);
	assert_eq!(
		*inv.get_slot(2).unwrap(), 
		Slot::new(ItemStack::new('a', 3))
	);
	assert_eq!(
		*inv.get_slot(3).unwrap(), 
		Slot::new(ItemStack::new('c', 3))
	);
}

#[test]
fn stack_at() {
	let mut inv = Inv::<char>::new(3);

	// cant be placed, slot out of bounds
	assert!(
		inv.stack_at(
			3, ItemStack::new('x', 1)
		).is_err()
	);

	// overflow
	assert_eq!(
		inv.stack_at(
			2, ItemStack::new('a', 4)
		),
		Ok(Err(StackErr::StackSizeOverflow(ItemStack::new('a', 1))))
	);

	assert!(
		inv.stack_at(
			1, ItemStack::new('b', 1)
		).is_ok()
	);

	// item cant be stacked, item type does not match
	assert_eq!(
		inv.stack_at(
			1, ItemStack::new('y', 1)
		),
		Ok(Err(StackErr::ItemTypeDoesNotMatch(ItemStack::new('y', 1))))
	);

	assert!(
		inv.stack_at(
			0, ItemStack::new('c', 1)
		).is_ok()
	);

	assert!(
		inv.stack_at(
			1, ItemStack::new('b', 1)
		).is_ok()
	);

	// 1c2b3a
	assert_eq!(
		*inv.get_slot(0).unwrap(), 
		Slot::new(ItemStack::new('c', 1))
	);
	assert_eq!(
		*inv.get_slot(1).unwrap(), 
		Slot::new(ItemStack::new('b', 2))
	);
	assert_eq!(
		*inv.get_slot(2).unwrap(), 
		Slot::new(ItemStack::new('a', 3))
	);
}

#[test]
fn take_stack() {
	let mut inv = Inv::<char>::new(16);

	// placing 2t
	inv.stack_at(5, ItemStack::new('t', 1)).ok();
	inv.auto_stack(ItemStack::new('t', 1)).ok();

	// taking 2t
	assert_eq!(
		inv.take_stack(5), 
		Ok(ItemStack::new('t', 2))
	);

	// taking again yields empty slot
	assert_eq!(
		inv.take_stack(5), 
		Err(InvAccessErr::SlotEmpty)
	);
}

#[test]
fn auto_stack_slotsize() {
	let mut inv = Inv::<char>::new(1);
	assert_eq!(
		inv.auto_stack(ItemStack::new('c', 12)).unwrap_err(),
		ItemStack::new('c', 9)
	);
}

#[test]
fn find_slot_decrease() {
	let mut inv = Inv::<char>::new(10);
	inv.auto_stack(ItemStack::new('c', 3)).ok();
	inv.auto_stack(ItemStack::new('x', 3)).ok();
	assert!(
		inv.find_slot_mut('x').unwrap().decrease_amount().is_ok()
	); // -1
	assert!(
		inv.find_slot_mut('x').unwrap().decrease_amount_by(2).is_ok()
	); // -2
	assert_eq!(
		inv.find_slot_mut('x'),
		Err(InvAccessErr::ItemNotFound) // because its at 0 (None)
	);
	// decrease c by 4
	assert_eq!(
		inv.find_slot_mut('c').unwrap().decrease_amount_by(4),
		Err(InvAccessErr::AmountInsufficient)
	); // -4
}

#[test]
fn find_slot_decrease_by() {
	let mut inv = Inv::<char>::new(10);
	inv.auto_stack(ItemStack::new('x', 3)).ok();
	assert!(
		inv.find_slot_mut('x').unwrap().decrease_amount_by(2).is_ok()
	);
	assert!(
		inv.find_slot_mut('x').unwrap().decrease_amount_by(2).is_err()
	);
	assert!(
		inv.find_slot_mut('x').unwrap().decrease_amount_by(1).is_ok()
	);
	inv.auto_stack(ItemStack::new('x', 3)).ok();
	assert!(
		inv.find_slot_mut('x').unwrap().decrease_amount_by(3).is_ok()
	);
}

#[test]
fn iterator() {
	let mut inv = Inv::<char>::new(4);
	inv.stack_at(0, ItemStack::new('x', 1)).ok();
	inv.stack_at(2, ItemStack::new('y', 3)).ok();
	
	for (num, slot) in inv.into_iter().enumerate() {
		match num {
			0 => assert_eq!(slot.get_amount(), Ok(1)),
			2 => assert_eq!(slot.get_amount(), Ok(3)),
			_ => assert!(slot.is_empty())
		}
	}
}

#[test]
fn sort() {
	let mut inv = Inv::<char>::new(4);
	inv.auto_stack(ItemStack::new('z', 1)).ok();
	inv.auto_stack(ItemStack::new('w', 1)).ok();
	inv.auto_stack(ItemStack::new('y', 1)).ok();
	inv.auto_stack(ItemStack::new('x', 1)).ok();
	
	inv.sort();

	assert_eq!(inv.get_slot(0).unwrap().get_item(), Ok(&'w'));
	assert_eq!(inv.get_slot(1).unwrap().get_item(), Ok(&'x'));
	assert_eq!(inv.get_slot(2).unwrap().get_item(), Ok(&'y'));
	assert_eq!(inv.get_slot(3).unwrap().get_item(), Ok(&'z'));
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Item {
	Apple,
	Banana,
	Mango,
	Peach,
	Orange
}

impl Stacksize for Item {
	fn get_max_stacksize(&self) -> usize {
		3
	}
}

#[test]
fn main() {
	let mut inv = Inv::<Item>::new(32);

	// cant be placed, slot out of bounds
	assert_eq!(
		inv.stack_at(
			666, ItemStack::new(Item::Peach, 1)
		),
		Err(InvAccessErr::SlotOutOfBounds)
	);

	// overflow, which is returned to you
	assert_eq!(
		inv.stack_at(
			2, ItemStack::new(Item::Apple, 4)
		),
		Ok(Err(
			StackErr::StackSizeOverflow(
				ItemStack::new(Item::Apple, 1)
			)
		))
	);

	// stack Banana at pos 1
	inv.stack_at(
		1, ItemStack::new(Item::Banana, 1)
	).ok();

	// item cant be stacked, 
	// item type does not match (Banana != Orange)
	assert_eq!(
		inv.stack_at(
			1, ItemStack::new(Item::Orange, 1)
		),
		Ok(Err(
			StackErr::ItemTypeDoesNotMatch(
				ItemStack::new(Item::Orange, 1)
			)
		))
	);

	// auto stacking
	// this first fills slot 1 to be at the max of 3
	// since slot 1 already had 1 Banana in it
	// the leftover will be placed in the first available slot,
	// which, in this case, is 0
	assert!(
		inv.auto_stack(
			ItemStack::new(Item::Banana, 3)
		).is_ok()
	);

	// 1 Banana, 3 Bananas
	assert_eq!(
		inv.get_slot(0), 
		Ok(&Slot::new(ItemStack::new(Item::Banana, 1)))
	);
	assert_eq!(
		inv.get_slot(1), 
		Ok(&Slot::new(ItemStack::new(Item::Banana, 3)))
	);

	// you can take a stack out of its slot
	// first, we place 2 Mangos at slot 5
	inv.stack_at(5, ItemStack::new(Item::Mango, 1)).ok();
	inv.auto_stack(ItemStack::new(Item::Mango, 1)).ok();

	// now we just take the stack
	assert_eq!(
		inv.take_stack(5), 
		Ok(ItemStack::new(Item::Mango, 2))
	);

	// slot 5 is empty now
	assert_eq!(
		inv.take_stack(5), 
		Err(InvAccessErr::SlotEmpty)
	);
}