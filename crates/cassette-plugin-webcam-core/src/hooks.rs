use std::{
    rc::Rc,
    sync::atomic::{AtomicU64, Ordering},
};

use anyhow::{anyhow, Result};
use cassette_core::cassette::{CassetteContext, CassetteTaskHandle};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{
    window, Blob, BlobEvent, Document, HtmlAnchorElement, MediaDevices, MediaRecorder, MediaStream,
    MediaStreamConstraints, Navigator, Url, Window,
};

use crate::recorder::WebcamRecorder;

pub fn use_webcam(
    ctx: &mut CassetteContext,
    handler: &crate::Handler,
    constraints: &crate::Constraints,
) -> CassetteTaskHandle<Result<WebcamRecorder, String>> {
    let handler_name = "webcam";
    let force_init = false;
    ctx.use_state(handler_name, force_init, || {
        build_webcam(handler, constraints).map_err(|error| error.to_string())
    })
}

fn build_webcam(
    handler: &crate::Handler,
    constraints: &crate::Constraints,
) -> Result<WebcamRecorder> {
    let crate::Constraints { audio, video } = *constraints;

    // Load a global window object
    let window = window().ok_or_else(|| anyhow!("Global window object not found"))?;
    let navigator: Navigator = window.navigator();

    // Load media devices
    let media_devices: MediaDevices = navigator
        .media_devices()
        .map_err(|_| anyhow!("Failed to get media devices"))?;

    // Convert the constraints
    let constraints: MediaStreamConstraints = MediaStreamConstraints::new();
    constraints.set_audio(&JsValue::from_bool(audio));
    constraints.set_video(&JsValue::from_bool(video));

    // Request media access permission
    let webcam = WebcamRecorder {
        media_stream_promise: media_devices
            .get_user_media_with_constraints(&constraints)
            .map_err(|_| anyhow!("Failed to get user media"))?,
        result: Rc::default(),
    };

    // Create a closure for handling session
    let handler = handler.clone();
    let result_store = webcam.result.clone();
    let on_success = Closure::wrap(Box::new(move |media_stream| {
        handle_stream(media_stream, &window, &handler);
        result_store.borrow_mut().replace(Ok(()));
    }) as Box<dyn FnMut(JsValue)>);

    // Create a closure for handling errors
    let result_store = webcam.result.clone();
    let on_error = Closure::wrap(Box::new(move |error: JsValue| {
        ::web_sys::console::error_1(&error);
        result_store.borrow_mut().replace(Err(error
            .as_string()
            .unwrap_or_else(|| "Unknown error".into())));
    }) as Box<dyn FnMut(JsValue)>);

    // Register the closures
    let _ = webcam
        .media_stream_promise
        .then(&on_success)
        .catch(&on_error);
    on_success.forget();
    on_error.forget();

    Ok(webcam)
}

fn handle_stream(media_stream: JsValue, window: &Window, handler: &crate::Handler) {
    let crate::Handler {
        duration,
        interval,
        url,
    } = handler;

    let stream: MediaStream = media_stream.unchecked_into();

    // Configure MediaRecorder
    let recorder =
        MediaRecorder::new_with_media_stream(&stream).expect("MediaRecorder creation failed");

    // Configure Session
    let session = Rc::new(Session::new(url.clone()));

    // Configure the event handler: `ondataavailable`
    {
        let session = session.clone();
        let ondataavailable = Closure::wrap(Box::new(move |event: BlobEvent| {
            let blob = event.data().expect("No data available");
            session.commit(blob)
        }) as Box<dyn FnMut(_)>);

        recorder.set_ondataavailable(Some(ondataavailable.as_ref().unchecked_ref()));
        ondataavailable.forget();
    }

    // Start recording!
    {
        let time_slice = (*interval).try_into().expect("Too large interval");
        recorder
            .start_with_time_slice(time_slice)
            .expect("Failed to start recorder")
    };

    // Set the timeout to break
    if let Some(timeout) = *duration {
        let recorder = recorder.clone();
        let stop_recorder =
            Closure::wrap(
                Box::new(move || recorder.stop().expect("Failed to stop recorder"))
                    as Box<dyn Fn()>,
            );

        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                stop_recorder.as_ref().unchecked_ref(),
                timeout.try_into().expect("Too large interval"),
            )
            .expect("Failed to set timeout");
        stop_recorder.forget();
    }

    // Configure the event handler: `onstop`
    {
        let onstop = Closure::wrap(Box::new(move || session.finalize()) as Box<dyn Fn()>);

        recorder.set_onstop(Some(onstop.as_ref().unchecked_ref()));
        onstop.forget();
    }
}

struct Session {
    index: AtomicU64,
    url: String,
}

impl Session {
    fn new(url: String) -> Self {
        Self {
            index: AtomicU64::default(),
            url,
        }
    }

    fn commit(&self, blob: Blob) {
        let Self { index, url } = self;

        // Generate an index
        let index = index.fetch_add(1, Ordering::SeqCst);

        // Create a download link using Blob
        let url = Url::create_object_url_with_blob(&blob).expect("Failed to create object URL");
        let window = window().expect("Global window object not found");
        let document: Document = window.document().expect("No document on window");
        let a: HtmlAnchorElement = document
            .create_element("a")
            .expect("Failed to create anchor element")
            .unchecked_into();

        // Force to download the recorded data
        a.set_href(&url);
        a.set_download("audio.webm");
        a.click();

        // Release the URL after use
        Url::revoke_object_url(&url).expect("Failed to revoke object URL");

        // todo!()
    }

    fn finalize(&self) {}
}
