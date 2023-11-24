mod deck;

use deck::*;

fn main() {
    let mut deck = Deck::new();
    
    println!("Random Card: {:?}", deck);
    deck.take_top();
    println!("Random Card: {:?}", deck);
}
