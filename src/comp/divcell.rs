use yew::prelude::*;

use super::cascade::Cascade;
use super::cells::Cells;
use super::foundations::Foundations;
use crate::playing_card::{Deck, PlayingCard};

pub struct Divcell {
    link: ComponentLink<Self>,
    marked_card: Option<PlayingCard>,
    cells: [Option<PlayingCard>; 4],
    foundations: [Option<PlayingCard>; 4],
    cascades: [Vec<PlayingCard>; 8],
}

pub enum Msg {
    Redraw,
    CardMarked(PlayingCard),
    CellClicked(usize),
    FoundationClicked(usize),
}

impl Component for Divcell {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut deck = Deck::new();
        deck.shuffle();
        let cells = [None, None, None, None];
        let foundations = [None, None, None, None];
        let cascades: [Vec<PlayingCard>; 8] = [
            deck.0[0..7].to_vec(),
            deck.0[7..14].to_vec(),
            deck.0[14..21].to_vec(),
            deck.0[21..28].to_vec(),
            deck.0[28..34].to_vec(),
            deck.0[34..40].to_vec(),
            deck.0[40..46].to_vec(),
            deck.0[46..52].to_vec(),
        ];
        Self {
            link,
            marked_card: None,
            cells,
            foundations,
            cascades,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CellClicked(n) => {
                if let Some(card) = &self.marked_card {
                    self.cells[n as usize] = Some(card.clone());
                    self.marked_card = None;
                }
            }
            Msg::FoundationClicked(n) => {
                if let Some(card) = &self.marked_card {
                    self.foundations[n as usize] = Some(card.clone());
                    self.marked_card = None;
                }
            }
            Msg::CardMarked(c) => {
                self.marked_card = Some(c);
            }
            _ => (),
        }
        true
    }

    fn view(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::Redraw);
        let mark_card = self.link.callback(Msg::CardMarked);
        let children = self.cascades.iter().map(|cards| {
            html! { <Cascade cards=cards.clone() mark_card=mark_card.clone() /> }
        });
        // let cells = self.props;
        html! {
            <>
            <div class="game-container">
                <div class="top">
                    <Cells
                        cells=[None, None, None, None]
                        callback=self.link.callback(Msg::CellClicked)
                    />
                    <Foundations
                        callback=self.link.callback(Msg::FoundationClicked)
                    />
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
