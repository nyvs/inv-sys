use crate::{stack::{Stackable}};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Slot<T> {
    pub(crate) inner: Option<T>,
}

impl<T: Stackable> Slot<T> {
    pub fn is_empty(&self) -> bool {
        self.inner.is_none()
    }

    pub fn can_stack(&self, stack: &T) -> bool {
        if let Some(existing) = &self.inner {
            existing.item() == stack.item() && existing.amount() < existing.max_amount()
        } else {
            false
        }
    }

    pub fn try_stack(&mut self, stack: &mut T) {
        if let Some(existing) = &mut self.inner {
            if existing.item() != stack.item() {
                return;
            }

            let space = existing.max_amount() - existing.amount();
            let to_add = stack.amount().min(space);

            if to_add <= 0 {
                return;
            }

            *existing.amount_mut() = existing.amount().saturating_add(to_add);
            *stack.amount_mut() = stack.amount().saturating_sub(to_add);
        }
    }

    pub fn insert(&mut self, stack: T) -> Option<T> {
        if self.is_empty() {
            self.inner = Some(stack);
            None
        } else {
            Some(stack)
        }
    }

    pub fn inner(&self) -> &Option<T> {
        &self.inner
    }
    
    pub fn take(&mut self) -> Option<T> {
        self.inner.take()
    }

    pub fn amount(&self) -> u32 {
        if let Some(existing) = &self.inner {
            existing.amount()
        } else {
            0
        }
    }

    pub fn decrement(&mut self) -> bool {
        if let Some(existing) = &mut self.inner {
            if existing.amount() > 0 {
                *existing.amount_mut() = existing.amount().saturating_sub(1);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn decrement_by(&mut self, amount: u32) -> bool {
        if let Some(existing) = &mut self.inner {
            if existing.amount() >= amount {
                *existing.amount_mut() = existing.amount().saturating_sub(amount);
                true
            } else {
                false
            }
        } else {
            false
        }
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
