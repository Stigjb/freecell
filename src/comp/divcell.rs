use log::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement, Request, RequestInit,
    RequestMode, Response,
};
use yew::{html, ComponentLink, Component, ShouldRender, Html};

use crate::fetch::{FetchError, FetchState};
use crate::playing_card;
use crate::texture_atlas::{Location, TextureAtlas};

pub struct Divcell {
    link: ComponentLink<Self>,
}

pub enum Msg {
    Redraw,
}

impl Component for Divcell {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Redraw => self.render().unwrap(),
        }
        true
    }

    fn view(&self) -> Html {
        info!("view()");
        let onclick = self.link.callback(|_| Msg::Redraw);
        html! {
            <>
            <div class="game-container">
                <div class="card" style="position: absolute; top: 10px; left: 10px;"></div>
                <div class="card" style="position: absolute; top: 100px; left: 100px;"></div>
            </div>
            <button onclick=onclick>{ "Redraw" }</button>
            </>
        }
    }
}

impl Divcell {
    fn render(&self) -> Result<(), JsValue> {
        let place_pos = vec![15., 95., 175., 255., 475., 555., 635., 715.];

        let mut deck = playing_card::Deck::new();
        deck.shuffle();
        for (i, card) in deck.0.iter().enumerate() {
            let row = i / 8;
            let col = i % 8;

            let x = 15. + 100. * col as f64;
            let y = 130. + 30. * row as f64;
        }
        Ok(())
    }
}

async fn fetch_texture_atlas() -> Result<TextureAtlas, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = "textureAtlas.json";
    let request: Request = Request::new_with_str_and_init(&url, &opts)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let json: JsValue = JsFuture::from(resp.json()?).await?;
    let atlas: TextureAtlas = json.into_serde()?;

    info!("Loaded texture atlas");

    Ok(atlas)
}
