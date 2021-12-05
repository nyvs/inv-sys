# inv-sys
[![Latest Release][crates-io-badge]][crates-io-url]
[![Documentation][docs-rs-img]][docs-rs-url]

A robust and effective inventory system for games.

## Features
- simple but robust API
- automatic stacking functionality
- taking stacks
- finding slots
- iterator
- max stacksize via trait
- and more!

## Usage
```rust
/* 
* Implement the Stacksize trait for 
* your type that will act as your Item
*/
impl super::Stacksize for char {
  fn get_max_stacksize(&self) -> usize {
    3
  }
}

fn main() {
  let mut inv = Inv::<char>::new(32);

  // cant be placed, slot out of bounds
  assert!(
    inv.stack_at(
      666, ItemStack::new('x', 1)
    ).is_err()
  );

  // overflow, which is returned to you
  assert_eq!(
    inv.stack_at(
      2, ItemStack::new('a', 4)
    ),
    Ok(Err(
      StackErr::StackSizeOverflow(
        ItemStack::new('a', 1)
      )
    ))
  );

  // stack c at pos 1
  inv.stack_at(
    1, ItemStack::new('c', 1)
  ).ok();
  
  // item cant be stacked, 
  // item type does not match (c != y)
  assert_eq!(
    inv.stack_at(
      1, ItemStack::new('y', 1)
    ),
    Ok(Err(
      StackErr::ItemTypeDoesNotMatch(
        ItemStack::new('y', 1)
      )
    ))
  );

  // auto stacking
  // this first fills slot 1 to be at the max of 3
  // since slot 1 already had 1c in it
  // the leftover will be placed in the first available slot,
  // which, in this case, is 0
  assert!(
    inv.auto_stack(
      ItemStack::new('c', 3)
    ).is_ok()
  );

  // 1c3c
  assert_eq!(
    inv.get_slot(0), 
    Ok(&Slot::new(ItemStack::new('c', 1)))
  );
  assert_eq!(
    inv.get_slot(1), 
    Ok(&Slot::new(ItemStack::new('c', 3)))
  );

  // you can take a stack out of its slot
  // first, we place 2t at slot 5
  inv.stack_at(5, ItemStack::new('t', 1)).ok();
  inv.auto_stack(ItemStack::new('t', 1)).ok();

  // now we just take the stack
  assert_eq!(
    inv.take_stack(5), 
    Ok(ItemStack::new('t', 2))
  );

  // slot 5 is empty now
  assert_eq!(
    inv.take_stack(5), 
    Err(InvAccessErr::SlotEmpty)
  );
}
```
You can look at the unit tests for more examples.

## Todo
 - better way to handle item remove/decrease
 - finding items
 - trees for sorting and performance improvements
 - simple configuration options
 - any requests? please just submit an issue, thanks!

## Contributions
Feel free to open an issue/PR explaining possible improvements or changes

## Help
Also, please do not hesitate and open an issue when needed. I am happy to help!

[crates-io-badge]: https://img.shields.io/crates/v/inv-sys.svg
[crates-io-url]: https://crates.io/crates/inv-sys
[docs-rs-img]: https://docs.rs/inv-sys/badge.svg
[docs-rs-url]: https://docs.rs/inv-sys