mod utils;

use vade::Vade;
use vade_evan::{
    resolver::{ResolverConfig, SubstrateDidResolverEvan},
    signing::{LocalSigner, Signer},
};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, vade-wasm-example!");
}

#[wasm_bindgen]
pub fn get_did_document(did: JsValue) -> js_sys::Promise {
    future_to_promise(async move {
        // create new vade instance with DID resolver
        let signer: Box<dyn Signer> = Box::new(LocalSigner::new());
        let rde = SubstrateDidResolverEvan::new(ResolverConfig {
            signer,
            target: "13.69.59.185".to_string(),
        });

        let mut vade = Vade::new();
        vade.register_plugin(Box::from(rde));
        // fetch document
        let results = vade.did_resolve(&did.as_string().unwrap()).await.unwrap();

        if results.len() != 1 {
            return Err(JsValue::from("could not get DID document"));
        }
        let did_document = results[0].as_ref().unwrap();

        // convert to JsValue and return
        Ok(JsValue::from(did_document))
    })
}
