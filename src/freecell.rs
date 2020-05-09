use log::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{HtmlImageElement, Request, RequestInit, RequestMode, Response};
use yew::prelude::*;

use crate::components::{cascades::Cascades, cells::Cells, foundations::Foundations};
use crate::fetch::{FetchError, FetchState};
use crate::playing_card::{random_card, PlayingCard};
use crate::texture_atlas::{Location, TextureAtlas};

pub struct Freecell {
    link: ComponentLink<Self>,
    spritesheet: HtmlImageElement,
    texture_atlas: FetchState<TextureAtlas>,
    state: State,
}

#[derive(Clone, PartialEq, Default)]
pub struct State {
    pub cells: [Option<PlayingCard>; 4],
}

pub enum Msg {
    Failed(JsValue),
    SetFetchState(FetchState<TextureAtlas>),
    Clicked,
    SpritesLoaded,
}

impl Component for Freecell {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let spritesheet = HtmlImageElement::new().expect("Couldn't create new Image");
        Self {
            link,
            spritesheet,
            texture_atlas: FetchState::default(),
            state: State::default(),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SpritesLoaded => {
                info!("SpritesLoaded message received");
            }
            Msg::Failed(_) => error!("Something failed, oh noes"),
            Msg::Clicked => self.state.cells[1] = Some(random_card()),
            Msg::SetFetchState(texture_atlas) => self.texture_atlas = texture_atlas,
        }
        true
    }

    fn rendered(&mut self, first_render: bool) {
        if !first_render {
            return;
        }

        // Initialize spritesheet
        let link = self.link.clone();
        let onload = Closure::wrap(Box::new(move || {
            link.send_message(Msg::SpritesLoaded);
        }) as Box<dyn Fn()>);
        self.spritesheet
            .set_onload(Some(onload.as_ref().unchecked_ref()));
        self.spritesheet.set_src("playingCards.png");
        onload.forget();

        // Load sprite info
        let link = self.link.clone();
        let future = async move {
            match fetch_texture_atlas().await {
                Ok(sys_info) => {
                    link.send_message(Msg::SetFetchState(FetchState::Success(sys_info)))
                }
                Err(err) => link.send_message(Msg::SetFetchState(FetchState::Failed(err))),
            }
        };
        spawn_local(future);
        self.link
            .send_message(Msg::SetFetchState(FetchState::Fetching));
    }

    fn view(&self) -> Html {
        info!("view()");
        let onclick = self.link.callback(|_| Msg::Clicked);
        match &self.texture_atlas {
            FetchState::Success(atlas) => html! {
                <>
                <div class="game-container">
                    <Cells
                        cell1=&self.state.cells[0]
                        cell2=&self.state.cells[1]
                        cell3=&self.state.cells[2]
                        cell4=&self.state.cells[3]
                        atlas=atlas
                    />
                    <Foundations />
                    <Cascades />
                </div>
                <button onclick=onclick>{ "Klikk" }</button>
                </>
            },
            _ => html! { "Loading resources" },
        }
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
