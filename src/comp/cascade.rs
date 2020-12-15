use super::card::card_comp;
use crate::playing_card::PlayingCard;
use log::*;
use yew::prelude::*;

pub struct Cascade {
    pub props: Props,
}

#[derive(Properties, Clone, Debug)]
pub struct Props {
    pub cards: Vec<PlayingCard>,
}

impl Component for Cascade {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        info!("{:?}", self.props);
        html! {
            <div class="cascade">
                { for self.props.cards.iter().map(card_comp) }
            </div>
        }
    }
}
