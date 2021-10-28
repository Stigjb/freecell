use crate::playing_card::{PlayingCard, Rank, Suit};
use log::info;
use yew::prelude::*;

fn position_of_card(card: &PlayingCard) -> (u8, u8) {
    match card {
        PlayingCard {
            suit: Suit::Spades,
            rank: Rank::Queen,
        } => (0, 0),
        PlayingCard {
            suit: Suit::Spades,
            rank: Rank::King,
        } => (0, 1),
        PlayingCard {
            suit: Suit::Spades,
            rank: Rank::Jack,
        } => (0, 2),
        PlayingCard {
            suit: Suit::Spades,
            rank: Rank::Ace,
        } => (0, 3),
        PlayingCard {
            suit: Suit::Spades,
            rank: Rank::Ten,
        } => (0, 4),
        PlayingCard {
            suit: Suit::Spades,
            rank: Rank::Nine,
        } => (0, 5),
        PlayingCard {
            suit: Suit::Spades,
            rank: Rank::Eight,
        } => (0, 6),
        PlayingCard {
            suit: Suit::Spades,
            rank: Rank::Seven,
        } => (0, 7),
        PlayingCard {
            suit: Suit::Spades,
            rank: Rank::Six,
        } => (0, 8),
        PlayingCard {
            suit: Suit::Spades,
            rank: Rank::Five,
        } => (0, 9),
        PlayingCard {
            suit: Suit::Spades,
            rank: Rank::Four,
        } => (1, 0),
        PlayingCard {
            suit: Suit::Spades,
            rank: Rank::Three,
        } => (1, 1),
        PlayingCard {
            suit: Suit::Spades,
            rank: Rank::Two,
        } => (1, 2),
        PlayingCard {
            suit: Suit::Hearts,
            rank: Rank::Queen,
        } => (1, 4),
        PlayingCard {
            suit: Suit::Hearts,
            rank: Rank::King,
        } => (1, 5),
        PlayingCard {
            suit: Suit::Hearts,
            rank: Rank::Jack,
        } => (1, 6),
        PlayingCard {
            suit: Suit::Hearts,
            rank: Rank::Ace,
        } => (1, 7),
        PlayingCard {
            suit: Suit::Hearts,
            rank: Rank::Ten,
        } => (1, 8),
        PlayingCard {
            suit: Suit::Hearts,
            rank: Rank::Nine,
        } => (1, 9),
        PlayingCard {
            suit: Suit::Hearts,
            rank: Rank::Eight,
        } => (2, 0),
        PlayingCard {
            suit: Suit::Hearts,
            rank: Rank::Seven,
        } => (2, 1),
        PlayingCard {
            suit: Suit::Hearts,
            rank: Rank::Six,
        } => (2, 2),
        PlayingCard {
            suit: Suit::Hearts,
            rank: Rank::Five,
        } => (2, 3),
        PlayingCard {
            suit: Suit::Hearts,
            rank: Rank::Four,
        } => (2, 4),
        PlayingCard {
            suit: Suit::Hearts,
            rank: Rank::Three,
        } => (2, 5),
        PlayingCard {
            suit: Suit::Hearts,
            rank: Rank::Two,
        } => (2, 6),
        PlayingCard {
            suit: Suit::Diamonds,
            rank: Rank::Queen,
        } => (2, 7),
        PlayingCard {
            suit: Suit::Diamonds,
            rank: Rank::King,
        } => (2, 8),
        PlayingCard {
            suit: Suit::Diamonds,
            rank: Rank::Jack,
        } => (2, 9),
        PlayingCard {
            suit: Suit::Diamonds,
            rank: Rank::Ace,
        } => (3, 0),
        PlayingCard {
            suit: Suit::Diamonds,
            rank: Rank::Ten,
        } => (3, 1),
        PlayingCard {
            suit: Suit::Diamonds,
            rank: Rank::Nine,
        } => (3, 2),
        PlayingCard {
            suit: Suit::Diamonds,
            rank: Rank::Eight,
        } => (3, 3),
        PlayingCard {
            suit: Suit::Diamonds,
            rank: Rank::Seven,
        } => (3, 4),
        PlayingCard {
            suit: Suit::Diamonds,
            rank: Rank::Six,
        } => (3, 5),
        PlayingCard {
            suit: Suit::Diamonds,
            rank: Rank::Five,
        } => (3, 6),
        PlayingCard {
            suit: Suit::Diamonds,
            rank: Rank::Four,
        } => (3, 7),
        PlayingCard {
            suit: Suit::Diamonds,
            rank: Rank::Three,
        } => (3, 8),
        PlayingCard {
            suit: Suit::Diamonds,
            rank: Rank::Two,
        } => (3, 9),
        PlayingCard {
            suit: Suit::Clubs,
            rank: Rank::Queen,
        } => (4, 0),
        PlayingCard {
            suit: Suit::Clubs,
            rank: Rank::King,
        } => (4, 1),
        PlayingCard {
            suit: Suit::Clubs,
            rank: Rank::Jack,
        } => (4, 2),
        PlayingCard {
            suit: Suit::Clubs,
            rank: Rank::Ace,
        } => (4, 3),
        PlayingCard {
            suit: Suit::Clubs,
            rank: Rank::Ten,
        } => (4, 4),
        PlayingCard {
            suit: Suit::Clubs,
            rank: Rank::Nine,
        } => (4, 5),
        PlayingCard {
            suit: Suit::Clubs,
            rank: Rank::Eight,
        } => (4, 6),
        PlayingCard {
            suit: Suit::Clubs,
            rank: Rank::Seven,
        } => (4, 7),
        PlayingCard {
            suit: Suit::Clubs,
            rank: Rank::Six,
        } => (4, 8),
        PlayingCard {
            suit: Suit::Clubs,
            rank: Rank::Five,
        } => (4, 9),
        PlayingCard {
            suit: Suit::Clubs,
            rank: Rank::Four,
        } => (5, 0),
        PlayingCard {
            suit: Suit::Clubs,
            rank: Rank::Three,
        } => (5, 1),
        PlayingCard {
            suit: Suit::Clubs,
            rank: Rank::Two,
        } => (5, 2),
        #[allow(unreachable_patterns)]
        _ => (1, 3), // Joker
    }
}

#[derive(Clone)]
pub struct Card {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub card: PlayingCard,
    pub onclick: Callback<PlayingCard>,
}

pub enum Msg {
    Clicked,
}

impl Component for Card {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        info!("Clicked {}", self.props.card);
        self.props.onclick.emit(self.props.card.clone());
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let (col, row) = position_of_card(&self.props.card);
        let class = format!("card row-{} col-{}", row, col);
        let onclick = self.link.callback(|_| Msg::Clicked);
        html! {
            <div class="card-wrapper">
                <div class=class draggable="true" onclick=onclick />
            </div>
        }
    }
}
