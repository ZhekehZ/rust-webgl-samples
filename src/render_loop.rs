use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlCanvasElement;

use crate::utils::web;
use crate::{
    gl::{core::instance::GL, error::GLError},
    samples::sample::Sample,
};

#[derive(PartialEq, Eq)]
pub enum OnErrorResult {
    Continue,
    Stop,
}

pub struct OnIterResult {
    pub render_frame: bool,
    pub update_logic: bool,
    pub stop_execution: bool,
}

pub struct RenderLoop<S>
where
    S: Sample + 'static,
{
    sample: S,
    on_error: Box<dyn FnMut(GLError) -> OnErrorResult + 'static>,
    on_iter: Box<dyn FnMut() -> OnIterResult + 'static>,
    last_update_time: f64,
}

impl<S> RenderLoop<S>
where
    S: Sample + 'static,
{
    pub fn create(canvas: &HtmlCanvasElement) -> Result<Self, GLError> {
        let gl = GL::new_for_canvas(canvas);
        Ok(Self {
            sample: S::try_new(gl)?,
            on_error: Box::new(|_| OnErrorResult::Stop),
            on_iter: Box::new(|| OnIterResult {
                render_frame: true,
                update_logic: true,
                stop_execution: false,
            }),
            last_update_time: 0.0,
        })
    }

    pub fn on_error(mut self, cb: impl FnMut(GLError) -> OnErrorResult + 'static) -> Self {
        self.on_error = Box::new(cb);
        self
    }

    pub fn on_iter(mut self, cb: impl FnMut() -> OnIterResult + 'static) -> Self {
        self.on_iter = Box::new(cb);
        self
    }

    pub fn run(mut self) {
        let callback = Rc::new(RefCell::new(None));
        let closure = Closure::wrap(Box::new({
            let callback = Rc::clone(&callback);
            self.last_update_time = js_sys::Date::now();
            move || {
                let OnIterResult {
                    render_frame: render,
                    update_logic,
                    stop_execution,
                } = (self.on_iter)();

                if stop_execution {
                    return;
                }

                if update_logic {
                    let curr_time = js_sys::Date::now();
                    let d_time = curr_time - self.last_update_time;
                    self.last_update_time = curr_time;
                    if let Err(e) = self.sample.update(d_time) {
                        if (self.on_error)(e) == OnErrorResult::Stop {
                            return;
                        }
                    }
                }

                if render {
                    if let Err(e) = self.sample.render() {
                        if (self.on_error)(e) == OnErrorResult::Stop {
                            return;
                        }
                    }
                }

                request_animation_frame(callback.borrow().as_ref().unwrap());
            }
        }) as Box<dyn FnMut()>);

        *callback.borrow_mut() = Some(closure);
        request_animation_frame(callback.borrow().as_ref().unwrap());
    }
}

fn request_animation_frame(closure: &Closure<dyn FnMut()>) {
    web::window()
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .unwrap();
}
