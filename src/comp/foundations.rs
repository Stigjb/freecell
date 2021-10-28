use yew::prelude::*;

pub struct Foundations {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub callback: Callback<usize>,
}

impl Component for Foundations {
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
        html! {
            <div class="foundations">
                <div class="foundation" onclick=self.link.callback(|_| 0_usize) />
                <div class="foundation" onclick=self.link.callback(|_| 1_usize) />
                <div class="foundation" onclick=self.link.callback(|_| 2_usize) />
                <div class="foundation" onclick=self.link.callback(|_| 3_usize) />
            </div>
        }
    }
}
