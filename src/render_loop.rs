use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlCanvasElement;

use crate::{
    gl::{core::instance::GL, error::GLError},
    samples::sample::Sample,
};
use crate::utils::web;

#[derive(PartialEq, Eq)]
pub enum OnErrorResult {
    Continue,
    Stop,
}

pub struct RenderLoop<S>
where
    S: Sample + 'static,
{
    sample: S,
    on_error: Box<dyn FnMut(GLError) -> OnErrorResult + 'static>,
    on_iter: Box<dyn FnMut() + 'static>,
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
            on_iter: Box::new(|| ()),
        })
    }

    pub fn on_error(mut self, cb: impl FnMut(GLError) -> OnErrorResult + 'static) -> Self {
        self.on_error = Box::new(cb);
        self
    }

    pub fn on_iter(mut self, cb: impl FnMut() + 'static) -> Self {
        self.on_iter = Box::new(cb);
        self
    }

    pub fn run(mut self) {
        let callback = Rc::new(RefCell::new(None));
        let closure = Closure::wrap(Box::new({
            let callback = Rc::clone(&callback);
            move || {
                if let Err(error) = self.sample.render() {
                    if (self.on_error)(error) == OnErrorResult::Stop {
                        return;
                    }
                }
                (self.on_iter)();
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
