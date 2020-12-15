use crate::comp::divcell::Divcell;
use log::*;
use yew::prelude::*;

pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        info!("view()");
        html! {
            <div class="container">
                <main>
                    <header class="header">
                        <h1>{ "Freecell" }</h1>
                    </header>
                    <Divcell />
                </main>
                <footer class="info">
                    <p>{ "Written by " }<a href="https://github.com/stigjb/" target="_blank">{ "Stig Johan Berggren" }</a></p>
                    <p>{ "Card graphics by " }<a href="https://kenney.nl/assets/boardgame-pack" target="_blank">{ "kenney.nl" }</a></p>
                </footer>
            </div>
        }
    }
}
