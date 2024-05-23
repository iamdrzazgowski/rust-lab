mod playing_cards {

    use std::fmt;

    #[derive(Debug, Copy, Clone)]
    pub enum Color {
        Spades,
        Hearts,
        Diamonds,
        Clubs
    }

    #[derive(Debug, Clone)]
    pub enum Value {
        Ace,
        King,
        Queen,
        Jack,
        Number(u8)
    }

    #[derive(Debug, Clone)]
    pub struct Card {
        color: Color,
        value: Value
    }

    impl fmt::Display for Card {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let value_ch: char;
            let color_ch: char;
            
            color_ch = match self.color {
                Color::Spades => 'S',
                Color::Hearts => 'H',
                Color::Diamonds => 'D',
                Color::Clubs => 'C'
            };

            value_ch = match self.value{
                Value::Ace => 'A',
                Value::King => 'K',
                Value::Queen => 'Q',
                Value::Jack => 'J', 
                // Value::Number(10) => 'T', też działa
                Value::Number(n) => if n < 10 {
                                        (n + ('0' as u8)) as char
                                    }else{
                                        'T'
                                    }
            };

            write!(f, "({}, {})", value_ch, color_ch)
        }
    }

    #[derive(Debug)]
    pub struct Deck {
        cards: Vec<Card>
    }

    impl Deck {
        pub fn new() -> Deck{
            let mut cards = Vec::new();

            for color in [Color::Spades, Color::Hearts, Color::Diamonds, Color::Clubs] {

                cards.push(Card{color, value: Value::Ace});
                cards.push(Card{color, value: Value::King});
                cards.push(Card{color, value: Value::Queen});
                cards.push(Card{color, value: Value::Jack});

                for i in 2..=10 {
                    cards.push(Card{color, value: Value::Number(i)});
                }
            }

            Deck {cards}
        }

        fn draw(&mut self) -> Option<Card> {
            let card = self.cards.last().cloned();
            self.cards.truncate(self.cards.len() - 1);
            card
        }
    }
}

use playing_cards::{Color, Value, Card, Deck};

fn main(){
    /*let some_color = Color::Spades;
    let some_value = Value::Number(7);
    let some_card = Card{color: Color::Hearts, value: Value::King};*/

    /*println!("{:?}, {:?}", some_color, some_value);
    println!("{:?}", some_card);

    let some_card2 = Card{color: Color::Spades, value:Value::Ace};
    println!("{}", some_card2);*/

    let deck = Deck::new();
    println!("{:?}", deck);
}
