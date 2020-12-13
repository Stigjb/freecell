use yew::{html, ComponentLink, Component, ShouldRender, Html};


pub struct Card {
    _link: ComponentLink<Self>,
}

impl Component for Card {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            _link: link,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="card" style="position: absolute; top: 10px; left: 10px;"></div>
        }
    }
}
