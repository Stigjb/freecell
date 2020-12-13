use log::*;
use yew::prelude::*;

use crate::playing_card;
use super::card::Card;

pub struct Divcell {
    link: ComponentLink<Self>,
}

pub enum Msg {
    Redraw,
}

impl Component for Divcell {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        info!("view()");
        let onclick = self.link.callback(|_| Msg::Redraw);
        let mut deck = playing_card::Deck::new();
        deck.shuffle();
        let children = deck.0.iter().enumerate().map(|(i, card)| {
            let col = i / 8;
            let row = i % 8;

            let x = 15. + 100. * col as f32;
            let y = 130. + 30. * row as f32;
            info!("{} {} {} {} ", col, row, x, y);

            html! { <Card pos=(x, y) card=card /> }
        });
        html! {
            <>
            <div class="game-container">
                { for children }
            </div>
            <button onclick=onclick>{ "Redraw" }</button>
            </>
        }
    }
}
