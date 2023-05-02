use crate::utils::logging::ExtUnwrapLog;

pub fn window() -> web_sys::Window {
    web_sys::window()
        .ok_or("Can't access window")
        .unwrap_log()
}

pub fn document() -> web_sys::Document {
    window()
        .document()
        .ok_or("Can't access document")
        .unwrap_log()
}

pub fn body() -> web_sys::HtmlElement {
    document()
        .body()
        .ok_or("Can't access document body")
        .unwrap_log()
}

pub fn window_size() -> (u32, u32) {
    let width = window().inner_width().unwrap().as_f64().unwrap() as u32;
    let height = window().inner_height().unwrap().as_f64().unwrap() as u32;
    (width, height)
}