use super::card::Card;
use crate::playing_card::PlayingCard;
use yew::prelude::*;

pub struct Cells {
    pub props: Props,
    pub link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub cells: [Option<PlayingCard>; 4],
    pub callback: Callback<usize>,
}

impl Component for Cells {
    type Message = usize;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.props.callback.emit(msg);
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let _children = &self.props.cells.iter().enumerate().map(|(i, c)| match c {
            None => empty_cell(self.link.callback(move |_| i)),
            Some(card) => html! {
                <Card card=card onclick=Callback::noop() />
            },
        });
        html! {
            <div class="cells">
                // { for children }
            </div>
        }
    }
}

fn empty_cell(_onclick: Callback<usize>) -> Html {
    html! { <div class="cell" onclick=Callback::noop() /> }
}
