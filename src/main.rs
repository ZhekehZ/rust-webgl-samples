mod gl;
mod math;
mod render_loop;
mod samples;
mod utils;
mod web;

use crate::render_loop::RenderLoop;
use crate::utils::logging::ExtUnwrapLog;
use utils::logging::init_logger;
use yew::prelude::*;

pub struct App {
    canvas_ref: NodeRef,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let body_style = web::body().style();

        body_style.set_property("margin", "0").unwrap_log();
        body_style.set_property("width", "100%").unwrap_log();
        body_style.set_property("height", "100%").unwrap_log();
        body_style.set_property("left", "0").unwrap_log();
        body_style.set_property("left", "0").unwrap_log();
        body_style.set_property("position", "fixed").unwrap_log();
        
        Self {
            canvas_ref: NodeRef::default(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <canvas ref={self.canvas_ref.clone()} width="100%" height="100%" />
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let canvas = self
                .canvas_ref
                .cast::<web_sys::HtmlCanvasElement>()
                .unwrap();

            RenderLoop::<samples::cubes::Cubes>::create(&canvas)
                .unwrap_log()
                .on_iter(move || {
                    let (width, height) = web::window_size();
                    canvas.set_width(width);
                    canvas.set_height(height);
                })
                .run();
        }
    }
}

fn main() {
    init_logger(log::LevelFilter::Trace).unwrap();
    yew::Renderer::<App>::new().render();
}
