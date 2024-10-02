use std::{cell::RefCell, rc::Rc};

use js_sys::Promise;

pub struct WebcamRecorder {
    pub(crate) media_stream_promise: Promise,
    pub(crate) result: Rc<RefCell<Option<Result<(), String>>>>,
}

impl WebcamRecorder {
    pub fn result(&self) -> Option<Result<(), String>> {
        self.result.borrow().clone()
    }
}
