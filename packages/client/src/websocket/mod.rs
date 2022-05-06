use beyond_core::events::Event;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

pub fn start() -> Result<(), JsValue> {
    // Connect to an echo server
    let ws = WebSocket::new("ws://127.0.0.1:3000/ws/")?;
    // For small binary messages, like CBOR, Arraybuffer is more efficient than Blob handling
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
    // create callback
    let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
        // Handle difference Text/Binary,...
        if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
            let array = js_sys::Uint8Array::new(&abuf);
            let len = array.byte_length() as usize;

            let event: Event = Event::from_u8_array(array.to_vec());

            info!("struct received: {:?} | byte length: #{}", event, len);
        } else {
            info!("message event, received Unknown: {:?}", e.data());
        }
    }) as Box<dyn FnMut(MessageEvent)>);
    // set message event handler on WebSocket
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    // forget the callback to keep it alive
    onmessage_callback.forget();

    let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
        info!("error event: {:?}", e);
    }) as Box<dyn FnMut(ErrorEvent)>);

    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));

    onerror_callback.forget();

    let cloned_ws = ws.clone();

    let onopen_callback = Closure::wrap(Box::new(move |_| {
        info!("socket opened");

        let event = Event::GetPlanets;

        match cloned_ws.send_with_u8_array(&event.into_u8_array()) {
            Ok(_) => info!("GetPlanets message successfully sent"),
            Err(err) => info!("error sending message: {:?}", err),
        }
    }) as Box<dyn FnMut(JsValue)>);

    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));

    onopen_callback.forget();

    Ok(())
}
