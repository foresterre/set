use set::Deck;

fn main() {
    let deck = Deck::default();
    for card in deck.cards() {
        println!("card: {:?}", card);
    }
}
