#![doc(html_root_url = "https://docs.rs/inv-sys/0.2.2")]

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Inv<T> {
	slots: Vec<Option<(T, usize)>>,
	maxslots: usize,
	selected_slot: usize
}

pub trait Stacksize {
	fn get_max_stacksize(&self) -> usize;
}

impl<T> Inv<T> where 
T: Stacksize + Eq + Clone {
	pub fn new(maxslots: usize) -> Self {
		Inv {
			slots: vec![None; maxslots],
			maxslots: maxslots,
			selected_slot: 0
		}
	}

	/// Returns an Optional Item for the amount of an item, that could not be placed
	pub fn place_at(&mut self, item: (T, usize), slot: usize) -> Option<(T, usize)> {
		if self.slot_valid(slot) {
			let item_at_spot = self.slots.get_mut(slot).unwrap().clone();
			return match item_at_spot {
				Some(ias) => {
					let new_item = self.split_item_for_stacksize((item.0, item.1 + ias.1));
					let ptr = self.slots.get_mut(slot).unwrap();
					*ptr = new_item[0].clone();
					new_item[1].clone()
				},
				None => {
					let new_item = self.split_item_for_stacksize(item);
					let ptr = self.slots.get_mut(slot).unwrap();
					*ptr = new_item[0].clone();
					new_item[1].clone()
				}
			}
		}
		Some(item)
	}

	/// Splits an item, so that the first one is safe to store in the inventory, 
	/// and the second is able to be processed by the user, if there is need
	fn split_item_for_stacksize(&self, item: (T, usize)) -> [Option<(T, usize)>; 2] {
		//check if item is over the stacksize
		if item.1 == 0 {
			return [None, None];
		}

		let max_stacksize = item.0.get_max_stacksize();
		if item.1 > max_stacksize {
			return [Some((item.0.clone(), max_stacksize)), Some((item.0, item.1 - max_stacksize))];
		}

		return [Some(item), None];
	}

	/// Tries to stack the item, returns remaining item.assert_eq!
	/// If no item is found, that it can be stacked on, the place_at_first_free method is used
	pub fn stack(&mut self, item: (T, usize)) -> Option<(T, usize)> {
		for (i, slot) in self.slots.iter_mut().enumerate() {
			match slot {
				None => continue,
				Some(slotval) => {
					//if items are equal and slot not full (allow overflow)
					if slotval.0 == item.0 
					&& slotval.1 < slotval.0.get_max_stacksize() {
						let rest = self.place_at(item, i);
						if rest != None && self.can_be_filled_with(rest.clone().unwrap()) {
							return self.stack(rest.unwrap());
						}
						else {
							return rest;
						}
					}
				}
			}
		}
		//no item found => try to use first free spot
		self.place_at_first_free(item)
	}

	/// If there still can be filled some slot with a minimum amount of 1, this should return true
	pub fn can_be_filled_with(&self, item: (T, usize)) -> bool {
		for slot in self.slots.iter() {
			let x = match slot {
				None => true,
				Some(slotval) => slotval.0 == item.0 && slotval.1 < slotval.0.get_max_stacksize()
			};
			if x {
				return true;
			}
		}
		false
	}

	/// Try to place the item at the first free spot, returns the remaining item
	pub fn place_at_first_free(&mut self, item: (T, usize)) -> Option<(T, usize)> {
		for (i, slot) in self.slots.iter_mut().enumerate() {
			match slot {
				None => {
					let rest = self.place_at(item, i);
					if rest != None && self.can_be_filled_with(rest.clone().unwrap()) {
						return self.place_at_first_free(rest.unwrap());
					}
					else {
						return rest;
					}
				},
				_ => continue
			}
		}
		Some(item)
	}

	/// Get item of a specific slot position
	pub fn get_slot(&self, slot: usize) -> Option<&(T, usize)> {
		self.slots.get(slot).unwrap().as_ref()
	}

	/// Get item of the selected slot position
	pub fn get_selected_slot(&self) -> Option<&(T, usize)> {
		self.slots.get(self.selected_slot).unwrap().as_ref()
	}

	/// Set the selected slot
	pub fn set_selected_slot(&mut self, slot: usize) -> bool {
		return if self.slot_valid(slot) {
			self.selected_slot = slot;
			true
		} else {
			false
		}
	}

	/// Decreases the amount of the item in the selected slot by 1
	pub fn decrease_selected_slot(&mut self) -> bool {
		let item = self.slots.get_mut(self.selected_slot).unwrap();
		match item {
			None => false,
			Some(x) => {
				x.1 -= 1;
				if x.1 == 0 {
					*item = None
				}
				true
			} 
		}
	}

	//checks if a slot would be valid in this inventory
	fn slot_valid(&self, slot: usize) -> bool {
		slot < self.maxslots
	}
}

impl<T> IntoIterator for Inv<T> {
    type Item = Option<(T, usize)>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.slots.into_iter()
    }
}

#[cfg(test)]
mod test;
