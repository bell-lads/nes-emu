use gloo_render::AnimationFrame;
use nes_emu::Nes;
use std::pin::Pin;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::{html, html::Scope, Component, Context, Html, NodeRef};

pub enum Msg {
    Render { timestamp: f64 },
}

pub struct App {
    canvas_ref: NodeRef,
    rendering_context: Option<CanvasRenderingContext2d>,
    _animation_frame: Option<AnimationFrame>,
    nes: Pin<Box<Nes>>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            canvas_ref: NodeRef::default(),
            rendering_context: None,
            _animation_frame: None,
            nes: Nes::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Render { timestamp } => {
                self.render_frame(timestamp);
                self.request_animation_frame(ctx.link().clone());
                false
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ "NES Emulator" }</h1>
                <canvas ref={ self.canvas_ref.clone() }/>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();

        let rendering_context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        self.rendering_context = Some(rendering_context);

        if first_render {
            self.request_animation_frame(ctx.link().clone());
        }
    }
}

impl App {
    fn render_frame(&mut self, timestamp: f64) {
        let rendering_context = self.rendering_context.as_ref().unwrap();

        rendering_context.fill_rect(10.0, 10.0, 200.0, 10.0);
    }

    fn request_animation_frame(&mut self, link: Scope<Self>) {
        let handle = {
            let link = link;
            gloo_render::request_animation_frame(move |time| {
                link.send_message(Msg::Render { timestamp: time })
            })
        };
        self._animation_frame = Some(handle);
    }
}

fn main() {
    yew::start_app::<App>();
}
