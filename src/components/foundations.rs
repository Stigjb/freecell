use yew::prelude::*;

pub struct Foundations {}

impl Component for Foundations {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="foundations">
                <div class="no-card" />
                <div class="no-card" />
                <div class="no-card" />
                <div class="no-card" />
            </div>
        }
    }
}
