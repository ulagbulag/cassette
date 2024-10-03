use std::rc::Rc;

use anyhow::{anyhow, Result};
use cassette_core::cassette::{CassetteContext, CassetteTaskHandle};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{
    window, Blob, BlobEvent, ErrorEvent, Event, MediaDevices, MediaRecorder, MediaRecorderOptions,
    MediaStream, MediaStreamConstraints, WebSocket, Window,
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
    let navigator = window.navigator();

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

    // Configure WebSocket connection
    let ws = WebSocket::new(url).expect("Failed to create WebSocket");

    // Configure Session
    let session = Rc::new(Session::new(ws));

    // Configure MediaRecorder
    let stream: MediaStream = media_stream.unchecked_into();
    let options = MediaRecorderOptions::new();
    options.set_mime_type("audio/webm;codecs=opus");

    let recorder =
        MediaRecorder::new_with_media_stream_and_media_recorder_options(&stream, &options)
            .expect("MediaRecorder creation failed");

    // Configure the event handler: `ondataavailable`
    {
        let session = session.clone();
        let ondataavailable = Closure::wrap(Box::new(move |event: BlobEvent| {
            let blob = event.data().expect("No data available");
            session.commit(&blob)
        }) as Box<dyn FnMut(_)>);

        recorder.set_ondataavailable(Some(ondataavailable.as_ref().unchecked_ref()));
        ondataavailable.forget()
    }

    // Configure the event handler: `onstop`
    {
        let session = session.clone();
        let onstop = Closure::wrap(Box::new(move || session.finalize()) as Box<dyn Fn()>);

        recorder.set_onstop(Some(onstop.as_ref().unchecked_ref()));
        onstop.forget()
    }

    // Configure the main loop
    let start_recording = {
        let duration = *duration;
        let time_slice = (*interval).try_into().expect("Too large interval");
        let window = window.clone();
        move || {
            // Start recording!
            recorder
                .start_with_time_slice(time_slice)
                .expect("Failed to start recorder");

            // Set the timeout to break
            if let Some(timeout) = duration {
                let recorder = recorder.clone();
                let stop_recorder = Closure::wrap(Box::new(move || {
                    recorder.stop().expect("Failed to stop recorder")
                }) as Box<dyn Fn()>);

                window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        stop_recorder.as_ref().unchecked_ref(),
                        timeout.try_into().expect("Too large interval"),
                    )
                    .expect("Failed to set timeout");
                stop_recorder.forget()
            }
        }
    };

    // Configure the event handler: `onopen`
    {
        let onopen = Closure::wrap(Box::new(move |_: Event| {
            ::web_sys::console::log_1(&"WebSocket connection opened".into());
            start_recording()
        }) as Box<dyn FnMut(_)>);

        session.ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
        onopen.forget()
    }

    // Configure the event handler: `onerror`
    {
        let onerror = Closure::wrap(Box::new(move |e: ErrorEvent| {
            ::web_sys::console::error_1(&format!("WebSocket error: {e:?}").into());
        }) as Box<dyn FnMut(_)>);

        session
            .ws
            .set_onerror(Some(onerror.as_ref().unchecked_ref()));
        onerror.forget()
    }

    // Configure the event handler: `onclose`
    {
        let onclose = Closure::wrap(Box::new(move |_: Event| {
            ::web_sys::console::log_1(&"WebSocket connection closed".into());
        }) as Box<dyn FnMut(_)>);

        session
            .ws
            .set_onclose(Some(onclose.as_ref().unchecked_ref()));
        onclose.forget()
    }
}

struct Session {
    ws: WebSocket,
}

impl Session {
    fn new(ws: WebSocket) -> Self {
        Self { ws }
    }

    fn commit(&self, blob: &Blob) {
        let Self { ws } = self;

        // Start sending Blob
        if ws.ready_state() == WebSocket::OPEN {
            ws.send_with_blob(&blob)
                .expect("Failed to send data via WebSocket")
        }
    }

    fn finalize(&self) {
        let Self { ws } = self;

        // Close websocket
        ws.close().expect("Failed to close WebSocket")
    }
}
