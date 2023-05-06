mod gl;
mod math;
mod render_loop;
mod samples;
mod utils;

use crate::render_loop::RenderLoop;
use crate::utils::logging::ExtUnwrapLog;
use crate::utils::web;
use render_loop::OnIterResult;
use utils::logging::init_logger;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;

const FRAMES_PER_SECOND: f64 = 30.0;
const FRAME_TIME: f64 = 1000.0 / FRAMES_PER_SECOND;

const LOGIC_UPDATE_PER_SECOND: f64 = 30.0;
const LOGIC_UPDATE_TIME: f64 = 1000.0 / LOGIC_UPDATE_PER_SECOND;


#[derive(Debug, Default)]
pub struct App {
    canvas_ref: NodeRef,
    fps_counter_ref: NodeRef,
    fps_limiter_ref: NodeRef,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <canvas ref={self.canvas_ref.clone()} id="main-canvas" />
                <div ref={self.fps_counter_ref.clone()} id="fps-counter"> {"fps:___"} </div>
                <div id="fps-limiter">
                    <input ref={self.fps_limiter_ref.clone()} type="checkbox"/>
                    {"limit"}
                </div>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let canvas = self
                .canvas_ref
                .cast::<web_sys::HtmlCanvasElement>()
                .unwrap();

            let fps_counter_ref = self.fps_counter_ref.clone();
            let fps_limiter_ref = self.fps_limiter_ref.clone();

            let mut prev_render_time = js_sys::Date::now();
            let mut prev_fps_update_time = prev_render_time;
            let mut frames_draw = 0;

            RenderLoop::<samples::cubes::Cubes>::create(&canvas)
                .unwrap_log()
                .on_iter(move || {
                    let (width, height) = web::window_size();
                    canvas.set_width(width);
                    canvas.set_height(height);

                    let curr_time = js_sys::Date::now();
                    let time_from_last_render = curr_time - prev_render_time;
                    let time_from_last_fps_update = curr_time - prev_fps_update_time;

                    let fps_limit_enabled = fps_limiter_ref
                        .cast::<HtmlInputElement>()
                        .unwrap()
                        .checked();

                    let update_logic = time_from_last_fps_update >= LOGIC_UPDATE_TIME;
                    let render_frame = !fps_limit_enabled || time_from_last_render >= FRAME_TIME;

                    if update_logic {
                        let fps = frames_draw as f64 * 1000.0 / time_from_last_fps_update + 0.5;
                        let text = format!("fps:{:03}", fps as usize);

                        let time_overhead = time_from_last_fps_update.rem_euclid(LOGIC_UPDATE_TIME);
                        frames_draw = 0;
                        prev_fps_update_time = curr_time - time_overhead;

                        fps_counter_ref
                            .cast::<HtmlElement>()
                            .unwrap()
                            .set_inner_text(text.as_str());
                    }


                    if render_frame {
                        let time_overhead = time_from_last_render.rem_euclid(FRAME_TIME);
                        prev_render_time = curr_time - time_overhead;
                        frames_draw += 1;
                    }

                    OnIterResult {
                        render_frame,
                        update_logic,
                        stop_execution: false,
                    }
                })
                .run();
        }
    }
}

fn main() {
    init_logger(log::LevelFilter::Trace).unwrap();
    yew::Renderer::<App>::new().render();
}
