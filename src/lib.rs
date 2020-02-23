extern crate strum;
#[macro_use]
extern crate strum_macros;

use strum::IntoEnumIterator;

#[derive(Debug, EnumIter, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Color {
    Green,
    Purple,
    Red,
}

#[derive(Debug, EnumIter, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Shape {
    Diamond,
    Oval,
    Squiggle,
}

#[derive(Debug, EnumIter, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Shading {
    Open,
    Solid,
    Striped,
}

#[derive(Debug, EnumIter, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Amount {
    One,
    Two,
    Three,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Card {
    color: Color,
    shape: Shape,
    shade: Shading,
    amount: Amount,
}
#[derive(Debug)]
pub struct Deck(pub Vec<Card>);

impl Default for Deck {
    fn default() -> Self {
        Deck(
            Color::iter()
                .flat_map(|color| {
                    Shape::iter().flat_map(move |shape| {
                        Shading::iter().flat_map(move |shade| {
                            Amount::iter().map(move |amount| Card {
                                color,
                                shape,
                                shade,
                                amount,
                            })
                        })
                    })
                })
                .collect::<Vec<Card>>(),
        )
    }
}

impl Deck {
    pub fn cards(&self) -> impl Iterator<Item = &Card> {
        self.0.iter()
    }
}

#[derive(Debug)]
pub struct Game {
    deck: Deck,
    open_cards: Vec<Card>,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            deck: Default::default(),
            open_cards: Vec::new(),
        }
    }
}

impl Game {
    pub fn _draw(&mut self, _n: usize) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deck_has_unique_cards() {
        let mut set = std::collections::HashSet::new();

        for card in Deck::standard().cards() {
            assert!(set.insert(card))
        }
    }

    #[test]
    fn deck_has_size() {
        assert_eq!(Deck::standard().cards().count(), 3 * 3 * 3 * 3);
    }
}
