use log::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{
    Blob, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, ImageBitmap, Request,
    RequestInit, RequestMode, Response,
};
use yew::prelude::*;

use crate::fetch::{FetchError, FetchState};

pub struct Freecell {
    link: ComponentLink<Self>,
    node_ref: NodeRef,
    canvas: Option<HtmlCanvasElement>,
    ctx: Option<CanvasRenderingContext2d>,
    spritesheet: HtmlImageElement,
    sprite_info: FetchState<()>,
}

pub enum Msg {
    Failed(JsValue),
    Redraw,
    SpritesLoaded,
}

impl Component for Freecell {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let spritesheet = HtmlImageElement::new().expect("Couldn't create new Image");
        Self {
            link,
            node_ref: NodeRef::default(),
            canvas: None,
            ctx: None,
            spritesheet,
            sprite_info: FetchState::default(),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SpritesLoaded => {
                info!("SpritesLoaded message received");
                self.render().unwrap();
            }
            Msg::Failed(_) => error!("Something failed, oh noes"),
            Msg::Redraw => self.render().unwrap(),
        }
        true
    }

    fn rendered(&mut self, first_render: bool) {
        if !first_render {
            return;
        }

        info!("rendered(first_render={})", first_render);
        let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();

        let ctx: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        self.canvas = Some(canvas);
        self.ctx = Some(ctx);

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
    }

    fn view(&self) -> Html {
        info!("view()");
        let onclick = self.link.callback(|_| Msg::Redraw);
        html! {
            <>
            <div class="game-container">
                <canvas width="800", height="600" ref=self.node_ref.clone()>
                    { "Canvas not supported" }
                </canvas>
            </div>
            <button onclick=onclick>{ "Redraw" }</button>
            </>
        }
    }
}

impl Freecell {
    fn render(&self) -> Result<(), JsValue> {
        let ctx = self.ctx.as_ref().unwrap();

        ctx.clear_rect(0., 0., 800., 600.);

        let place_pos = vec![15., 95., 175., 255., 475., 555., 635., 715.];
        for x in place_pos {
            ctx.rect(x, 15., 70., 95.);
        }
        ctx.stroke();

        for c in 0..52 {
            let row = c / 8;
            let col = c % 8;

            let x = 15. + 100. * col as f64;
            let y = 130. + 30. * row as f64;
            ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &self.spritesheet,
                0.,
                570.,
                140.,
                190.,
                x,
                y,
                70.,
                95.,
            )?;
        }
        Ok(())
    }
}

async fn fetch_sprites() -> Result<Blob, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = "playingCards.xml";
    let request: Request = Request::new_with_str_and_init(&url, &opts)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let blob: JsValue = JsFuture::from(resp.blob()?).await?;

    Ok(blob.into())
}
