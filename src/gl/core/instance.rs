use std::ops::Deref;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

use crate::{gl::error::GLError, utils::logging::ExtUnwrapLog};

#[repr(transparent)]
#[derive(Debug)]
pub struct GL {
    context: Rc<WebGl2RenderingContext>,
}

#[allow(unused)]
impl GL {
    pub fn new_for_canvas(canvas: &HtmlCanvasElement) -> Self {
        Self {
            context: Rc::new(
                canvas
                    .get_context("webgl2")
                    .unwrap()
                    .unwrap()
                    .dyn_into()
                    .map_err(GLError::GL2ContextError)
                    .unwrap_log(),
            ),
        }
    }
}

impl Deref for GL {
    type Target = WebGl2RenderingContext;

    fn deref(&self) -> &Self::Target {
        self.context.as_ref()
    }
}
