use crate::freecell::Freecell;
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
                <section class="todoapp">
                    <header class="header">
                        <h1>{ "Freecell" }</h1>
                    </header>
                    <section class="main">
                        <Freecell />
                    </section>
                    <footer class="footer">
                    </footer>
                </section>
                <footer class="info">
                    <p>{ "Written by " }<a href="https://github.com/stigjb/" target="_blank">{ "Stig Johan Berggren" }</a></p>
                    <p>{ "Card graphics by " }<a href="https://kenney.nl/assets/boardgame-pack" target="_blank">{ "kenney.nl" }</a></p>
                </footer>
            </div>
        }
    }
}
