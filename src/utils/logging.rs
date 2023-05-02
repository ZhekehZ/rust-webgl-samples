use wasm_bindgen::JsValue;

use crate::gl::{error::GLError, shader::error::ShaderError};

mod ffi {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        pub fn log(s: &str);
    }
}

struct JSLogger;

impl log::Log for JSLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let message = format!(
                "╭[{file}:{line}]\n│ module: {module}\n│ message: {message}\n╰───────────",
                file = record.file().unwrap_or("???"),
                line = record.line().unwrap_or(0),
                module = record.module_path().unwrap_or("???"),
                message = record.args().to_string().as_str(),
            );
            ffi::log(message.as_str());
        }
    }

    fn flush(&self) {}
}

pub fn init_logger(level: log::LevelFilter) -> Result<(), log::SetLoggerError> {
    static LOGGER: JSLogger = JSLogger;
    log::set_logger(&LOGGER)?;
    log::set_max_level(level);
    Ok(())
}

pub trait ExtUnwrapLog {
    type Output;

    fn unwrap_log(self) -> Self::Output;
}

impl<T, E: std::fmt::Debug> ExtUnwrapLog for Result<T, E> {
    type Output = T;

    fn unwrap_log(self) -> Self::Output {
        match self {
            Ok(value) => value,
            Err(error) => {
                log::error!("{:#?}", error);
                panic!("Unexpected error");
            },
        }
    }
}
