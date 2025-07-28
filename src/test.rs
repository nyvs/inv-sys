use uuid::Uuid;

use crate::{inv::{Inv}, stack::Stackable};

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct MyStack {
    pub uuid: Uuid,
    pub inventory_refer: Uuid,
    pub item_refer: ItemType,
    pub amount: u32
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
enum ItemType {
    Weapon,
    Ammo
}

use std::fmt::Debug;

impl Stackable for MyStack {
    type Item = ItemType;

    fn new_splitted(&self, amount: u32) -> Self {
        Self {
            uuid: Uuid::nil(),
            inventory_refer: self.inventory_refer,
            item_refer: self.item_refer.clone(),
            amount
        }
    }

    fn amount(&self) -> u32 {
        self.amount
    }

    fn amount_mut(&mut self) -> &mut u32 {
        &mut self.amount
    }

    fn max_amount(&self) -> u32 {
        match self.item_refer {
            ItemType::Weapon => 1,
            ItemType::Ammo => 20,
        }
    }

    fn item(&self) -> &Self::Item {
        &self.item_refer
    }
}

#[test]
fn main() {
	let mut inv = Inv::<MyStack, Uuid>::new(2, Uuid::new_v4());

	//add 
    let res = inv.auto_stack(MyStack {
        uuid: Uuid::nil(), 
        inventory_refer: Uuid::nil(),
        item_refer: ItemType::Ammo,
        amount: 12
    });

    assert_eq!(res, None);

    //add 
    let res = inv.auto_stack(MyStack {
        uuid: Uuid::nil(), 
        inventory_refer: Uuid::nil(),
        item_refer: ItemType::Ammo,
        amount: 12
    });

    assert_eq!(res, None);

    //add 
    let res = inv.auto_stack(MyStack {
        uuid: Uuid::nil(), 
        inventory_refer: Uuid::nil(),
        item_refer: ItemType::Ammo,
        amount: 17
    });

    assert_eq!(res, Some(MyStack {
        uuid: Uuid::nil(),
        inventory_refer: Uuid::nil(),
        item_refer: ItemType::Ammo,
        amount: 1,
    }));

    assert_eq!(inv.take_from(0), Some(MyStack {
        uuid: Uuid::nil(),
        inventory_refer: Uuid::nil(),
        item_refer: ItemType::Ammo,
        amount: 20,
    }));

    dbg!(inv);
}

#[test]
fn test_auto_stack() {
    let mut inv = Inv::<MyStack, Uuid>::new(4, Uuid::new_v4());

    assert_eq!(inv.insert_at(0, MyStack {
        uuid: Uuid::nil(),
        inventory_refer: Uuid::nil(),
        item_refer: ItemType::Ammo,
        amount: 15,
    }), Ok(()));

    assert_eq!(inv.insert_at(1, MyStack {
        uuid: Uuid::nil(),
        inventory_refer: Uuid::nil(),
        item_refer: ItemType::Ammo,
        amount: 15,
    }), Ok(()));

    assert_eq!(inv.insert_at(2, MyStack {
        uuid: Uuid::nil(),
        inventory_refer: Uuid::nil(),
        item_refer: ItemType::Ammo,
        amount: 15,
    }), Ok(()));

    assert_eq!(inv.auto_stack(MyStack {
        uuid: Uuid::nil(),
        inventory_refer: Uuid::nil(),
        item_refer: ItemType::Ammo,
        amount: 5+5+5+20+5,
    }), Some(MyStack {
        uuid: Uuid::nil(),
        inventory_refer: Uuid::nil(),
        item_refer: ItemType::Ammo,
        amount: 5,
    }));


    let mut inv = Inv::<MyStack, Uuid>::new(4, Uuid::new_v4());
    assert_eq!(inv.insert_at(0, MyStack {
        uuid: Uuid::nil(),
        inventory_refer: Uuid::nil(),
        item_refer: ItemType::Ammo,
        amount: 15,
    }), Ok(()));

    assert_eq!(inv.insert_at(2, MyStack {
        uuid: Uuid::nil(),
        inventory_refer: Uuid::nil(),
        item_refer: ItemType::Ammo,
        amount: 15,
    }), Ok(()));

    inv.auto_stack(MyStack {
        uuid: Uuid::nil(),
        inventory_refer: Uuid::nil(),
        item_refer: ItemType::Ammo,
        amount: 5+5+5,
    });

    assert_eq!(inv.take_from(1), Some(MyStack {
        uuid: Uuid::nil(),
        inventory_refer: Uuid::nil(),
        item_refer: ItemType::Ammo,
        amount: 5,
    }));
}

#[test]
fn find_decrement() {
    let mut inv = Inv::<MyStack, Uuid>::new(4, Uuid::new_v4());

    assert_eq!(inv.insert_at(0, MyStack {
        uuid: Uuid::nil(),
        inventory_refer: Uuid::nil(),
        item_refer: ItemType::Weapon,
        amount: 1,
    }), Ok(()));

    assert_eq!(inv.insert_at(2, MyStack {
        uuid: Uuid::nil(),
        inventory_refer: Uuid::nil(),
        item_refer: ItemType::Ammo,
        amount: 15,
    }), Ok(()));

    assert_eq!(inv.find_mut(&ItemType::Ammo).map(|slot| slot.decrement()), Some(true));
    assert_eq!(inv.find_mut(&ItemType::Ammo).map(|slot| slot.amount()), Some(14));

    assert_eq!(inv.find_mut(&ItemType::Ammo).map(|slot| slot.decrement_by(14)), Some(true));
    assert_eq!(inv.find_mut(&ItemType::Ammo).map(|slot| slot.amount()), Some(0));

    assert!(inv.take_from(2).is_some());

    inv.clean_empty();

    assert_eq!(inv.take_from(2), None);
}