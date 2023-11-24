use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct Deck {
    cards: Vec<Card>
}

#[derive(Debug, Copy, Clone)]
pub struct Card(u8);

impl Deck {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut cards: Vec<Card> = (1..=52).map(Card).collect();
        cards.shuffle(&mut rng);
        Deck { cards }
    }
    
    pub fn take_top(&mut self) -> Option<Card> {
        self.cards.pop()
        
    }

    /// Will take n cards from the deck and can return
    /// less than or up to the amount requested
    pub fn take_n(&mut self, n: u8) -> Vec<Card> {
        let mut cards = Vec::new();
        for i in 0..n {
            match self.take_top() {
                Some(card) => cards.push(card),
                None => return cards
            }
        }
        cards
    }
}

impl Card {
    pub fn suit(&self) -> &'static str {
        match self.0 % 4 {
            x => "a",
            _ => todo!()
        }
    }
}