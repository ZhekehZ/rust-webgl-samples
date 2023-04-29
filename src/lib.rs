mod gl;
mod math;
mod samples;
mod utils;

use std::cell::RefCell;
use std::rc::Rc;

pub use samples::simple_cube::App as SimpleCubeApp;

use utils::logging::init_logger;
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast, JsValue,
};

const MAX_FPS: f64 = 60.0;

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    init_logger(log::LevelFilter::Trace).map_err(|_| "Set logger error")?;

    let window = web_sys::window().ok_or("No global `window` exists")?;
    let document = window
        .document()
        .ok_or("should have a document on window")?;
    let body = document.body().ok_or("document should have a body")?;

    document.set_title("WebGL samples");

    body.style().set_property("margin", "0")?;
    body.style().set_property("height", "100%")?;

    let canvas = {
        let element = document
            .create_element("canvas")?
            .dyn_into::<web_sys::HtmlCanvasElement>()?;
        element.set_id("canvas");
        element.style().set_property("width", "100%")?;
        element.style().set_property("height", "100%")?;
        element.style().set_property("display", "block")?;
        element
    };
    body.append_child(&canvas)?;

    let simple_cube = Rc::new(RefCell::new(SimpleCubeApp::new()?));

    let mut prev_draw_time = js_sys::Date::now();

    let anim_cb = AnimationCallback::new(Box::new({
        let simple_cube_ptr = Rc::clone(&simple_cube);
        move |cb| {
            let window = &web_sys::window().unwrap();

            let width = window.inner_width()?;
            let height = window.inner_height()?;

            canvas.set_width(width.as_f64().unwrap() as u32);
            canvas.set_height(height.as_f64().unwrap() as u32);

            if js_sys::Date::now() - prev_draw_time >= 1000.0 / MAX_FPS {
                simple_cube_ptr.borrow_mut().render()?;
                prev_draw_time = js_sys::Date::now();
            }

            cb.request_animation_frame(window)?;
            Ok(())
        }
    }));
    anim_cb.request_animation_frame(&window)?;

    Ok(())
}

#[allow(clippy::type_complexity)]
struct AnimationCallback {
    inner: Rc<RefCell<Option<Closure<dyn FnMut()>>>>,
}

#[allow(clippy::type_complexity)]
impl AnimationCallback {
    pub fn new(mut f: Box<dyn FnMut(&AnimationCallback) -> Result<(), JsValue>>) -> Self {
        let ptr = Rc::new(RefCell::new(None));
        let ptr_copy = Rc::clone(&ptr);

        *ptr.borrow_mut() = Some(Closure::new(move || {
            f(&Self {
                inner: Rc::clone(&ptr_copy),
            })
            .unwrap();
        }));

        Self { inner: ptr }
    }

    pub fn request_animation_frame(&self, window: &web_sys::Window) -> Result<(), JsValue> {
        window.request_animation_frame(
            self.inner
                .borrow()
                .as_ref()
                .unwrap()
                .as_ref()
                .unchecked_ref(),
        )?;
        Ok(())
    }
}
