#![doc(html_root_url = "https://docs.rs/inv-sys/1.1.0")]

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
	pub fn new_empty() -> Self {
		Self {
			inner: None
		}
	}

	pub fn new(items: ItemStack<T>) -> Self {
		Self {
			inner: Some(items)
		}
	}

	pub fn is_empty(&self) -> bool {
		self.inner.is_none()
	}

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

	pub fn get_item_type(&self) -> Result<T, InvAccessErr> {
		if let Some(inner) = &self.inner {
			Ok(inner.get_type())
		} else {
			Err(InvAccessErr::SlotEmpty)
		}
	}

	pub fn get_amount(&self) -> Result<usize, InvAccessErr> {
		if let Some(inner) = &self.inner {
			Ok(inner.get_amount())
		} else {
			Err(InvAccessErr::SlotEmpty)
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ItemStack<T> {
	item_type: T,
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
	SlotEmpty
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
	pub fn new(item_type: T, amount: usize) -> Self {
		Self {
			item_type,
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
		let max = self.item_type.get_max_stacksize();
		if self.amount > max {
			let rest = self.amount - max;
			self.amount = max;
			Err(
				ItemStack::<T>::new(
				self.item_type.clone(), 
				rest
			))
		} else {
			Ok(())
		}
	}

	pub fn stack(&mut self, other: ItemStack<T>) -> Result<(), StackErr<T>> {
		if other.item_type == self.item_type {
			self.amount += other.amount;
			self.stacksize_split().map_err(|err| StackErr::StackSizeOverflow(err))
		} else {
			Err(StackErr::ItemTypeDoesNotMatch(other))
		}
	}

	pub fn get_type(&self) -> T {
		self.item_type.clone()
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

	pub fn get_slot(&self, index: usize) -> Result<&Slot<T>, InvAccessErr> {
		match self.slots.get(index) {
			Some(slot) => Ok(slot),
			None => Err(InvAccessErr::SlotOutOfBounds)
		}
	}
}

#[cfg(test)]
mod test;
