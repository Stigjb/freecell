use yew::prelude::*;

use crate::playing_card::PlayingCard;
use crate::texture_atlas::TextureAtlas;

pub struct Cells {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub cell1: Option<PlayingCard>,
    pub cell2: Option<PlayingCard>,
    pub cell3: Option<PlayingCard>,
    pub cell4: Option<PlayingCard>,
    pub atlas: TextureAtlas,
}

impl Component for Cells {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="cells">
                { cell(&self.props.cell1) }
                { cell(&self.props.cell2) }
                { cell(&self.props.cell3) }
                { cell(&self.props.cell4) }
            </div>
        }
    }
}

fn cell(maybe_card: &Option<PlayingCard>) -> Html {
    match maybe_card {
        Some(card) => html! {
            <div class="card">
                { format!("{:?} of {:?}", card.rank, card.suit) }
            </div>
        },
        None => html! { <div class="no-card" /> },
    }
}
