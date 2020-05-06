mod utils;

use vade::Vade;
use vade_evan::plugin::rust_vcresolver_evan::RustVcResolverEvan;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, vade-wasm-example!");
}

#[wasm_bindgen]
pub fn get_vc_document(vc_name: JsValue) -> js_sys::Promise  {
    future_to_promise(async move {
        // create new vade instance with VC resolver
        let rde = RustVcResolverEvan::new();
        let mut vade = Vade::new();
        vade.register_vc_resolver(Box::from(rde));
    
        // fetch document
        let vc_document = vade.get_vc_document(&vc_name.as_string().unwrap()).await.unwrap();

        // convert to JsValue and return
        Ok(JsValue::from(&vc_document))
    })
}
