#![doc(html_root_url = "https://docs.rs/inv-sys/1.3.0")]

use std::fmt::Debug;

pub trait Stacksize {
	fn get_max_stacksize(&self) -> usize;
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Inv<T> {
	slots: Vec<Slot<T>>,
	maxslots: usize,
	selected_slot: usize
}

impl<T> IntoIterator for Inv<T> {
	type Item = Slot<T>;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.slots.into_iter()
	}
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Slot<T> {
	inner: Option<ItemStack<T>>
}

impl<T> Slot<T>
where T: Stacksize + Eq + Clone {
	/// Creates an empty slot
	pub fn new_empty() -> Self {
		Self {
			inner: None
		}
	}

	/// Creates a new Slot with a given Itemstack
	pub fn new(items: ItemStack<T>) -> Self {
		Self {
			inner: Some(items)
		}
	}

	/// Returns true if the Slot is empty
	pub fn is_empty(&self) -> bool {
		self.inner.is_none()
	}

	/// Tops this slot up with a given ItemStack
	pub fn stack(&mut self, to_place: ItemStack<T>) -> Result<(), StackErr<T>> {
		if let Some(inner) = &mut self.inner {
			match inner.stack(to_place) {
				Ok(()) => Ok(()),
				Err(rest) => Err(rest),
			}
		} else {
			match ItemStack::new_from_stack(to_place) {
				Ok(new) => {
					self.inner = Some(new);
					Ok(())
				},
				Err((new, rest)) => {
					self.inner = Some(new);
					Err(rest)
				},
			}
		}
	}

	/// Returns the optional ItemStack in the Slot
	pub fn inner(&self) -> &Option<ItemStack<T>> {
		&self.inner
	}

	/// Returns the item in the Slot
	pub fn get_item(&self) -> Result<&T, InvAccessErr> {
		if let Some(inner) = &self.inner {
			Ok(inner.get_item())
		} else {
			Err(InvAccessErr::SlotEmpty)
		}
	}

	/// Returns the amount of items in the Slot
	pub fn get_amount(&self) -> Result<usize, InvAccessErr> {
		if let Some(inner) = &self.inner {
			Ok(inner.get_amount())
		} else {
			Err(InvAccessErr::SlotEmpty)
		}
	}

	/// Decreases amount by 1
	pub fn decrease_amount(&mut self) -> Result<(), InvAccessErr> {
		if let Some(inner) = &mut self.inner {
			if inner.get_amount() > 0 {
				inner.amount -= 1;
				if inner.amount == 0 {
					self.inner = None;
				}
				Ok(())
			} else {
				unreachable!();
				//Err(InvAccessErr::AmountInsufficient)
			}
		} else {
			Err(InvAccessErr::SlotEmpty)
		}
	}

	/// Decreases amount by a given arbitrary number
	pub fn decrease_amount_by(&mut self, amount: usize) -> Result<(), InvAccessErr> {
		if let Some(inner) = &mut self.inner {
			if inner.get_amount() >= amount {
				inner.amount -= amount;
				if inner.amount == 0 {
					self.inner = None;
				}
				Ok(())
			} else {
				Err(InvAccessErr::AmountInsufficient)
			}
		} else {
			Err(InvAccessErr::SlotEmpty)
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ItemStack<T> {
	item: T,
	amount: usize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StackErr<T>
where T: Stacksize + Eq + Clone {
	ItemTypeDoesNotMatch(ItemStack<T>),
	StackSizeOverflow(ItemStack<T>)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum InvAccessErr {
	SlotOutOfBounds,
	SlotEmpty,
	ItemNotFound,
	AmountInsufficient
}

pub type InvOverflow<T> = ItemStack<T>;

impl<T> From<StackErr<T>> for ItemStack<T>
where T: Stacksize + Eq + Clone {
	fn from(item: StackErr<T>) -> Self {
		match item {
			StackErr::ItemTypeDoesNotMatch(x) => x,
			StackErr::StackSizeOverflow(x) => x,
		}
	}
}

impl<T> ItemStack<T> 
where T: Stacksize + Eq + Clone {
	pub fn new(item: T, amount: usize) -> Self {
		Self {
			item,
			amount
		}
	}

	pub fn new_from_stack(mut new: ItemStack<T>) -> Result<Self, (Self, StackErr<T>)> {
		match new.stacksize_split() {
			Ok(()) => Ok(new),
			Err(rest) => Err((new, StackErr::StackSizeOverflow(rest)))
		}
	}

	fn stacksize_split(&mut self) -> Result<(), ItemStack<T>> {
		let max = self.item.get_max_stacksize();
		if self.amount > max {
			let rest = self.amount - max;
			self.amount = max;
			Err(
				ItemStack::<T>::new(
				self.item.clone(), 
				rest
			))
		} else {
			Ok(())
		}
	}

	pub fn stack(&mut self, other: ItemStack<T>) -> Result<(), StackErr<T>> {
		if other.item == self.item {
			self.amount += other.amount;
			self.stacksize_split().map_err(|err| StackErr::StackSizeOverflow(err))
		} else {
			Err(StackErr::ItemTypeDoesNotMatch(other))
		}
	}

	pub fn get_item(&self) -> &T {
		&self.item
	}

	pub fn get_amount(&self) -> usize {
		self.amount
	}
}

impl<T> Inv<T> 
where T: Stacksize + Eq + Clone {
	pub fn new(maxslots: usize) -> Self {
		Inv {
			slots: vec![Slot::new_empty(); maxslots],
			maxslots: maxslots,
			selected_slot: 0
		}
	}

	// Fill filled slots only
	fn auto_stack_inner_filled(&mut self, to_place: ItemStack<T>) -> Result<(), StackErr<T>> {
		let mut state = to_place.clone();
		for slot in self.slots.iter_mut() {
			if !slot.is_empty() {
				match slot.stack(state) {
					Ok(()) => return Ok(()),
					Err(rest) => {
						state = rest.into();
					}
				}
			} else {
				continue
			}
		}
		Err(StackErr::StackSizeOverflow(state))
	}

	// Fill empty slots only
	fn auto_stack_inner_empty(&mut self, to_place: ItemStack<T>) -> Result<(), StackErr<T>> {
		let mut state = to_place.clone();
		for slot in self.slots.iter_mut() {
			if slot.is_empty() {
				match slot.stack(state) {
					Ok(()) => return Ok(()),
					Err(rest) => {
						state = rest.into();
					}
				}
			}
		}
		Err(StackErr::StackSizeOverflow(state))
	}

	/// Add items to the Inventory
	/// Already used slots will be filled before empty slots will
	pub fn auto_stack(&mut self, to_place: ItemStack<T>) -> Result<(), InvOverflow<T>> {
		match self.auto_stack_inner_filled(to_place) {
			Ok(()) => return Ok(()),
			Err(rest) => {
				match self.auto_stack_inner_empty(rest.into()) {
					Ok(()) => return Ok(()),
					Err(rest) => Err(rest.into()),
				}
			}
		}
	}

	/// Add items to a specific Slot
	pub fn stack_at(&mut self, index: usize, to_place: ItemStack<T>) -> Result<Result<(), StackErr<T>>, InvAccessErr> {
		match self.slots.get_mut(index) {
			Some(slot) => {
				Ok(
					match slot.stack(to_place) {
						Ok(()) => Ok(()),
						Err(rest) => Err(rest),
					}
				)
			},
			None => Err(InvAccessErr::SlotOutOfBounds)
		}
	}

	/// Take the entire ItemStack sitting in a Slot at a given position.
	/// This means, that the ItemStack will be taken out of the slot, leaving it empty 
	pub fn take_stack(&mut self, index: usize) -> Result<ItemStack<T>, InvAccessErr> {
		match self.slots.get_mut(index) {
			Some(slot) => {
				if let Some(filled) = &slot.inner {
					let take = filled.clone();
					slot.inner = None;
					Ok(take)
				} else {
					Err(InvAccessErr::SlotEmpty)
				}
			},
			None => Err(InvAccessErr::SlotOutOfBounds)
		}
	}

	/// Return a Slot with at a given position
	pub fn get_slot(&self, index: usize) -> Result<&Slot<T>, InvAccessErr> {
		match self.slots.get(index) {
			Some(slot) => Ok(slot),
			None => Err(InvAccessErr::SlotOutOfBounds)
		}
	}

	/// Return a Slot with at a given position mutably
	pub fn get_slot_mut(&mut self, index: usize) -> Result<&mut Slot<T>, InvAccessErr> {
		match self.slots.get_mut(index) {
			Some(slot) => Ok(slot),
			None => Err(InvAccessErr::SlotOutOfBounds)
		}
	}

	/// Return a Slot with a given item
	pub fn find_slot(&self, item: T) -> Result<&Slot<T>, InvAccessErr> {
		for slot in self.slots.iter() {
			if let Some(inner) = &slot.inner {
				if *inner.get_item() == item {
					return Ok(slot);
				}
			}
		}
		Err(InvAccessErr::ItemNotFound)
	}

	/// Return a Slot with a given item mutably
	pub fn find_slot_mut(&mut self, item: T) -> Result<&mut Slot<T>, InvAccessErr> {
		for slot in self.slots.iter_mut() {
			if let Some(inner) = &slot.inner {
				if *inner.get_item() == item {
					return Ok(slot);
				}
			}
		}
		Err(InvAccessErr::ItemNotFound)
	}
}

#[cfg(test)]
mod test;
