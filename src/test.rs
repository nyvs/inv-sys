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
fn auto_stack_slotsize() {
	let mut inv = Inv::<char>::new(1);
	assert_eq!(
		inv.auto_stack(ItemStack::new('c', 12)).unwrap_err(),
		ItemStack::new('c', 9)
	);
}
/*

#[test]
fn iterator() {
	let mut inv = Inv::<char>::new(2);
	inv.place_at(('x', 1), 0).ok();
	inv.place_at(('y', 3), 2).ok();
	
	for (num, slot) in inv.into_iter().enumerate() {
		if slot.is_some() {
			match num {
				0 => {
					assert_eq!(slot.unwrap().0, 'x')
				},
				2 => {
					assert_eq!(slot.unwrap().0, 'y')
				},
				_ => unreachable!()
			}
		} else {
			assert!(slot.is_none())
		}
	}
}*/