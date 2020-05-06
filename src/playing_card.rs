use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    Rng,
};

#[derive(Debug, Clone, PartialEq)]
pub struct PlayingCard {
    suit: Suit,
    rank: Rank,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Suit {
    Hearts,
    Spades,
    Clubs,
    Diamonds,
}

pub fn random_card() -> PlayingCard {
    let suit: Suit = rand::random();
    let rank: Rank = rand::random();
    PlayingCard { suit, rank }
}

impl PlayingCard {
    pub fn id(&self) -> String {
        let suit = match self.suit {
            Suit::Spades => "Spades",
            Suit::Hearts => "Hearts",
            Suit::Clubs => "Clubs",
            Suit::Diamonds => "Diamonds",
        };
        let rank = match self.rank {
            Rank::Ace => "A",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
        };
        format!("card{}{}", suit, rank)
    }
}

impl Distribution<Suit> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Suit {
        match rng.gen_range(0, 4) {
            0 => Suit::Hearts,
            1 => Suit::Spades,
            2 => Suit::Clubs,
            _ => Suit::Diamonds,
        }
    }
}

impl Distribution<Rank> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Rank {
        match rng.gen_range(0, 13) {
            0 => Rank::Ace,
            1 => Rank::Two,
            2 => Rank::Three,
            3 => Rank::Four,
            4 => Rank::Five,
            5 => Rank::Six,
            6 => Rank::Seven,
            7 => Rank::Eight,
            8 => Rank::Nine,
            9 => Rank::Ten,
            10 => Rank::Jack,
            11 => Rank::Queen,
            _ => Rank::King,
        }
    }
}

pub struct Deck(pub Vec<PlayingCard>);

impl Deck {
    pub fn new() -> Self {
        let mut cards: Vec<PlayingCard> = Vec::with_capacity(52);
        let ranks = vec![
            Rank::Ace,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
        ];
        let suits = vec![Suit::Hearts, Suit::Clubs, Suit::Diamonds, Suit::Spades];
        for rank in ranks.iter() {
            for suit in suits.iter() {
                cards.push(PlayingCard {
                    suit: suit.clone(),
                    rank: rank.clone(),
                })
            }
        }
        Self(cards)
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.0.shuffle(&mut rng);
    }
}
