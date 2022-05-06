use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Blob, MessageEvent, WebSocket, ErrorEvent};
use serde::{Deserialize, Serialize};
use rkyv;

#[derive(Serialize, Deserialize, Debug)]
struct WsEvent<T> {
    event: String,
    data: Option<T>
}

impl<'a, T: Serialize + Deserialize<'a>> WsEvent<T> {
    pub fn new(event: String) -> WsEvent<()> {
        WsEvent::<()> {
            event,
            data: None
        }
    }

    pub fn from_data(event: String, data: T) -> Self {
        WsEvent::<T> {
            event,
            data: Some(data),
        }
    }

    pub fn set_data(&mut self, data: Option<T>) {
        self.data = data;
    }

    // pub fn into_bytes(&self) -> Vec<u8>{
    //     rkyv::to_bytes::<_, 256>(&self).unwrap().to_vec()
    // }

    pub fn into_u8_array(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }

    pub fn into_blob(&self) -> Result<web_sys::Blob, JsValue> {
        let bin: Vec<u8> = self.into_u8_array();
        let bin_js_value = JsValue::from_serde(&bin).unwrap();
        Blob::new_with_u8_array_sequence(&bin_js_value)
    }

    pub fn from_u8_array(data: &'a Vec<u8>) -> Self {
        bincode::deserialize(&data[..]).unwrap()
    }
}

pub fn start_websocket() -> Result<(), JsValue> {
    // Connect to an echo server
    let ws = WebSocket::new("ws://127.0.0.1:3000/ws/")?;
    // For small binary messages, like CBOR, Arraybuffer is more efficient than Blob handling
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
    // create callback
    let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
        // Handle difference Text/Binary,...
        if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
            log::info!("message event, received arraybuffer: {:?}", abuf);
            let array = js_sys::Uint8Array::new(&abuf);
            let len = array.byte_length() as usize;
            log::info!("Arraybuffer received {}bytes: {:?}", len, array.to_vec());

            let event: WsEvent<()> = WsEvent::from_u8_array(&array.to_vec());

            log::info!("struct received: {:?}", event);

        } else {
            log::info!("message event, received Unknown: {:?}", e.data());
        }
    }) as Box<dyn FnMut(MessageEvent)>);
    // set message event handler on WebSocket
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    // forget the callback to keep it alive
    onmessage_callback.forget();

    let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
        log::info!("error event: {:?}", e);
    }) as Box<dyn FnMut(ErrorEvent)>);

    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));

    onerror_callback.forget();

    let cloned_ws = ws.clone();

    let onopen_callback = Closure::wrap(Box::new(move |_| {
        log::info!("socket opened");

        let event = WsEvent::<()>::new("connect".to_string());

        match cloned_ws.send_with_u8_array(&event.into_u8_array()) {
            Ok(_) => log::info!("binary message successfully sent"),
            Err(err) => log::info!("error sending message: {:?}", err),
        }

    }) as Box<dyn FnMut(JsValue)>);

    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));

    onopen_callback.forget();

    Ok(())
}
