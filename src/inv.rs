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
            slots: vec![Slot { inner: None }; 16],
            meta: M::default(),
            slot_amount: 16
        }
    }
}

impl<T, M> Inv<T, M> 
where T: Stackable + Debug + Eq + Clone + Ord + Clone, M: Default {
     pub fn new(slot_amount: usize, meta: M) -> Self {
        Self {
            slots: vec![Slot { inner: None }; slot_amount],
            meta,
            slot_amount
        }
    }

    pub fn insert_at(&mut self, index: usize, stack: T) -> Result<(), T> {
        if let Some(slot) = self.slots.get_mut(index) {
            if slot.is_empty() {
                slot.insert(stack);
                Ok(())
            } else {
                Err(stack)
            }
        } else {
            Err(stack)
        }
    }

    pub fn take_from(&mut self, index: usize) -> Option<T> {
        self.slots.get_mut(index).and_then(|slot| slot.take())
    }

    pub fn auto_stack(&mut self, mut stack: T) -> Option<T> {
        for slot in &mut self.slots {
            if stack.amount() == 0 {
                return None;
            }
            if slot.can_stack(&stack) {
                slot.try_stack(&mut stack);
            }
        }

        for slot in &mut self.slots {
            if stack.amount() == 0 {
                return None;
            }
            if slot.is_empty() {
                let to_add = stack.amount().min(stack.max_amount());
                let new_stack = stack.new_splitted(to_add);
                *stack.amount_mut() = stack.amount().saturating_sub(to_add);

                slot.insert(new_stack);
            }
        }

        if stack.amount() > 0 {
            Some(stack)
        } else {
            None
        }
    }

    pub fn find(&mut self, item: &T::Item) -> Option<&Slot<T>> {
        let r = self.slots.iter().find(|s| s.inner.as_ref().map(|stack| stack.item()) == Some(item));
        r
    }

    pub fn find_mut(&mut self, item: &T::Item) -> Option<&mut Slot<T>> {
        let r = self.slots.iter_mut().find(|s| s.inner.as_ref().map(|stack| stack.item()) == Some(item));
        r
    }

    pub fn iter(&self) -> SlotIterator<T> {
        SlotIterator {
            slots: &self.slots,
            index: 0,
        }
    }

    pub fn clean_empty(&mut self) {
        self.slots.iter_mut().for_each(|slot| {
            if let Some(stack) = &mut slot.inner {
                if stack.amount() == 0 {
                    slot.inner = None;
                }
            }
        });
    }

    pub fn iter_mut(&mut self) -> SlotIteratorMut<T> {
        SlotIteratorMut {
            slots: self.slots.iter_mut(),
        }
    }
}
