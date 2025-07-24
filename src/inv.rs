use crate::{slot::{Slot, SlotIterator, SlotIteratorMut}, stack::Stackable};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Inv<T, M> {
	slots: Vec<Slot<T>>,
	pub meta: M,
    slot_amount: usize
}

impl<T, M> IntoIterator for Inv<T, M> {
	type Item = Slot<T>;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.slots.into_iter()
	}
}

impl<T, M> Default for Inv<T, M>
where T: Eq + Clone + Ord + Stackable + Debug, M: Default + Debug {
    fn default() -> Self {
        Inv {
            slots: vec![Slot { stack: None }; 16],
            meta: M::default(),
            slot_amount: 16
        }
    }
}

impl<T, M> Inv<T, M> 
where T: Stackable + Debug + Eq + Clone + Ord + Clone, M: Default {
     pub fn new(slot_amount: usize, meta: M) -> Self {
        Self {
            slots: vec![Slot { stack: None }; slot_amount],
            meta,
            slot_amount
        }
    }

    pub fn insert_at(&mut self, index: usize, item: T) -> Result<(), T> {
        if let Some(slot) = self.slots.get_mut(index) {
            if slot.is_empty() {
                slot.insert(item);
                Ok(())
            } else {
                Err(item)
            }
        } else {
            Err(item)
        }
    }

    pub fn take_from(&mut self, index: usize) -> Option<T> {
        self.slots.get_mut(index).and_then(|slot| slot.take())
    }

    pub fn auto_stack(&mut self, mut item: T) -> Option<T> {
        for slot in &mut self.slots {
            if item.amount() == 0 {
                return None;
            }
            if slot.can_stack(&item) {
                slot.try_stack(&mut item);
            }
        }

        for slot in &mut self.slots {
            if item.amount() == 0 {
                return None;
            }
            if slot.is_empty() {
                let mut new_stack = item.clone();
                let to_add = new_stack.amount().min(new_stack.max_amount());

                *new_stack.amount_mut() = new_stack.amount().saturating_add(new_stack.amount() - to_add);
                *item.amount_mut() = item.amount().saturating_sub(to_add);

                slot.insert(new_stack);
            }
        }

        if item.amount() > 0 {
            Some(item)
        } else {
            None
        }
    }

    pub fn iter(&self) -> SlotIterator<T> {
        SlotIterator {
            slots: &self.slots,
            index: 0,
        }
    }

    pub fn iter_mut(&mut self) -> SlotIteratorMut<T> {
        SlotIteratorMut {
            slots: self.slots.iter_mut(),
        }
    }
}
