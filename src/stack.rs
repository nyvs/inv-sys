pub trait Stackable {
    type Item: PartialEq + Eq + PartialOrd + Ord;
    fn new_splitted(&self, amount: u32) -> Self;
    
	fn amount(&self) -> u32;

	fn amount_mut(&mut self) -> &mut u32;

	fn max_amount(&self) -> u32;

    fn item(&self) -> &Self::Item;
}
