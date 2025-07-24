use crate::{stack::{Stackable}};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Slot<T> {
    pub stack: Option<T>,
}

impl<T: Stackable> Slot<T> {
    pub fn is_empty(&self) -> bool {
        self.stack.is_none()
    }

    pub fn can_stack(&self, item: &T) -> bool {
        if let Some(existing) = &self.stack {
            existing.item() == item.item() && existing.amount() < existing.max_amount()
        } else {
            false
        }
    }

    pub fn try_stack(&mut self, item: &mut T) -> u32 {
        if let Some(existing) = &mut self.stack {
            if existing.item() != item.item() {
                return 0;
            }

            let space = existing.max_amount() - existing.amount();
            let to_add = item.amount().min(space);

            *existing.amount_mut() = existing.amount().saturating_add(to_add);
            *item.amount_mut() = item.amount().saturating_sub(to_add);

            to_add
        } else {
            0
        }
    }

    pub fn insert(&mut self, item: T) -> Option<T> {
        if self.is_empty() {
            self.stack = Some(item);
            None
        } else {
            Some(item)
        }
    }

    pub fn take(&mut self) -> Option<T> {
        self.stack.take()
    }
}

pub struct SlotIterator<'a, T> {
    pub(crate) slots: &'a [Slot<T>],
    pub(crate) index: usize,
}

pub struct SlotIteratorMut<'a, T> {
    pub(crate) slots: std::slice::IterMut<'a, Slot<T>>,
}

impl<'a, T> Iterator for SlotIterator<'a, T> {
    type Item = &'a Slot<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.slots.len() {
            let result = Some(&self.slots[self.index]);
            self.index += 1;
            result
        } else {
            None
        }
    }
}

impl<'a, T> Iterator for SlotIteratorMut<'a, T> {
    type Item = &'a mut Slot<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.slots.next()
    }
}
