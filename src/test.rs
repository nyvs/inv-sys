#[cfg(test)]
mod tests {
	impl super::Stacksize for char {
		fn get_max_stacksize(&self) -> usize {
			3
		}
	}
	#[test]
	fn slotsize() {
		let mut inv = super::Inv::<char>::new(2);
		inv.place_at(('x', 1), 0);
		inv.place_at(('y', 3), 1);
		assert_eq!(inv.place_at(('z', 3), 3), Some(('z', 3)));
	}

	#[test]
	fn stacking() {
		let mut inv = super::Inv::<char>::new(2);
		assert_eq!(inv.stack(('x', 1)), None);
		assert_eq!(inv.get_slot(0).unwrap().1, 1);
		assert_eq!(inv.get_slot(1), None);
		assert_eq!(inv.stack(('x', 1)), None);
		assert_eq!(inv.get_slot(0).unwrap().1, 2);
		assert_eq!(inv.get_slot(1), None);
	}

	#[test]
	fn stacksize_single() {
		let mut inv = super::Inv::<char>::new(2);
		assert_eq!(inv.place_at(('x', 10), 0), Some(('x', 7)));
	}

	#[test]
	fn find_free() {
		let mut inv = super::Inv::<char>::new(3);
		assert_eq!(inv.place_at(('x', 2), 0), None);
		assert_eq!(inv.place_at(('y', 2), 2), None);
		assert_eq!(inv.place_at_first_free(('z', 3)), None);
		assert_eq!(inv.get_slot(1), Some(&('z', 3)));
	}

	#[test]
	fn stacksize_multiple() {
		let mut inv = super::Inv::<char>::new(2);
		assert_eq!(inv.stack(('x', 1)), None);
		assert_eq!(inv.stack(('x', 1)), None);
		assert_eq!(inv.stack(('x', 1)), None);
		assert_eq!(inv.stack(('x', 1)), Some(('x', 1)));
		assert_eq!(inv.get_slot(0).unwrap().1, 3);
	}

	#[test]
	fn find_item() {
		let mut inv = super::Inv::<char>::new(3);
		assert_eq!(inv.place_at(('x', 1), 1), None);
		assert_eq!(inv.stack(('x', 1)), None);
		assert_eq!(inv.get_slot(1).unwrap().1, 2);
	}

	#[test]
	fn selected_slot() {
		let mut inv = super::Inv::<char>::new(300);
		assert_eq!(inv.set_selected_slot(300), false);
		assert_eq!(inv.set_selected_slot(2), true);
		assert_eq!(inv.get_selected_slot(), None);
		assert_eq!(inv.place_at(('z', 2), 2), None);
		assert_eq!(inv.get_selected_slot(), Some(&('z', 2)));
	}
}