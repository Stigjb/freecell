use super::card::Card;
use crate::playing_card::PlayingCard;
use yew::prelude::*;

pub struct Cascade {
    pub props: Props,
}

#[derive(Properties, Clone, Debug)]
pub struct Props {
    pub cards: Vec<PlayingCard>,
    pub mark_card: Callback<PlayingCard>,
}

impl Component for Cascade {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let children = self
            .props
            .cards
            .iter()
            .map(|c| html! { <Card card=c onclick=self.props.mark_card.clone() /> });
        html! {
            <div class="cascade">
                { for children }
            </div>
        }
    }
}
