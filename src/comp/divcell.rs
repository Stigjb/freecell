use log::*;
use yew::prelude::*;

use super::cells::cells_comp;
use super::foundations::foundations_comp;
use super::cascade::Cascade;
use crate::playing_card::{Deck, PlayingCard};

pub struct Divcell {
    link: ComponentLink<Self>,
    cascades: [Vec<PlayingCard>; 8],
}

pub enum Msg {
    Redraw,
}

impl Component for Divcell {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut deck = Deck::new();
        deck.shuffle();
        let cascades: [Vec<PlayingCard>; 8] = [
            deck.0[0..7].iter().cloned().collect::<Vec<_>>(),
            deck.0[7..14].iter().cloned().collect::<Vec<_>>(),
            deck.0[14..21].iter().cloned().collect::<Vec<_>>(),
            deck.0[21..28].iter().cloned().collect::<Vec<_>>(),
            deck.0[28..34].iter().cloned().collect::<Vec<_>>(),
            deck.0[34..40].iter().cloned().collect::<Vec<_>>(),
            deck.0[40..46].iter().cloned().collect::<Vec<_>>(),
            deck.0[46..52].iter().cloned().collect::<Vec<_>>(),
        ];
        Self { link, cascades }
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
        let children = self.cascades.iter().map(|cards| {
            html! { <Cascade cards=cards.clone() /> }
        });
        info!("{:?}", children);
        html! {
            <>
            <div class="game-container">
                <div class="top">
                    { cells_comp() }
                    { foundations_comp() }
                </div>
                <div class="tableau">
                    { for children }
                </div>
            </div>
            <button onclick=onclick>{ "Redraw" }</button>
            </>
        }
    }
}
