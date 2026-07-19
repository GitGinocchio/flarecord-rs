use worker::{js_sys, wasm_bindgen::prelude::wasm_bindgen};


#[wasm_bindgen]
unsafe extern "C" {
    #[wasm_bindgen(js_name = getMessageQueue)]
    fn get_message_queue() -> js_sys::Array;
}

pub fn init_bridge() {
    let js_code = r#"
        globalThis.messageQueue = [];
        globalThis.handleSocketEvent = (type, data) => {
            globalThis.messageQueue.push({ type, data });
        };
        globalThis.getMessageQueue = () => {
            const msgs = [...globalThis.messageQueue];
            globalThis.messageQueue = [];
            return msgs;
        };
    "#;
    
    js_sys::eval(js_code).expect("Impossibile inizializzare il bridge JS");
}