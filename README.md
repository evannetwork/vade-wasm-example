# vade-wasm-example
## About this project
This project is an example about how to use the [`vade`] library in wasm projects.

This readme describes how to create such a project on basis of the [Rust and WebAssembly] book and the project itself can be used as a starting point for trying out vade in wasm with zero implementation effort or as a reference when building own projects.

## Using vade in WASM
[`vade`] itself is used in the Rust part of your projects and its results can then be exposed to the JavaScript part of your application.

As already mentioned, we're following the tutorial from [Rust and WebAssembly] book. This section is intended for getting started from scratch. If you already have a project you can go to the next section.

### Creating the hello world project

Check if you have all requirements available as described in the [setup section].

After this follow the steps described in [Hello, World!], we named our project `vade-wasm-example` instead of `wasm-game-of-life`, but followed the other steps as documented.

Your project should now resemble the project from the tag ["new-wasm-project"]. We made the following minor changes to the code:

- we deleted the folder `www/.git` to keep the code in a single project
- we added `package-lock.json` to the gitignore file `www/.gitignore` to keep the lock file out of the project

### Adding vade to you WASM project (Rust part)
For this project we will use the [`VcResolver`] from [`vade-evan`] to fetch and validate VC documents from evan.network.

#### Dependencies
Open your in your `Cargo.toml` and add these new `[dependencies]`:

```toml
js-sys = "0.3.39"
vade = "0.0.6"
vade-evan = "0.0.5"
wasm-bindgen-futures = "0.4.12"
```
(Or check for newer versions on crates.io.)

#### Add vade to your WASM file
Open your `src/lib.rs` file and add new use declarations:

```rust
use vade::Vade;
use vade_evan::plugin::rust_vcresolver_evan::RustVcResolverEvan;
use wasm_bindgen_futures::future_to_promise;
```

`Vade` and `RustVcResolverEvan` are here for the obvious reason that we want to use them and `future_to_promise` are here because we will be converting the future from the vade request to a JavaScript promise.

#### Getting our VC document in Rust

Add this example function, to fetch a VC document.

```rust
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
```

To build your project again and to update the WASM files run:

```rust
wasm-pack build
```

With this the Rust side should be covered, so let's head to the JavaScript part.

#### Use our new function in JavaScript

Open `www/index.js` and add a block, that uses the new function:

```js
(async () => {
    // fetch VC
    const vc = prompt("vc", "vc:evan:testcore:0x75956ef9b3ea7d7230cf007b8ee042bcaa2a4dad8c043fa77ecf51262ee4f7a9");
    const vcDocument = await wasm.get_vc_document(vc);
    
    // render VC
    const div = document.createElement('div');
    div.style['white-space'] = 'pre-wrap';
    div.innerText = JSON.stringify(JSON.parse(vcDocument), null, 2);
    document.body.appendChild(div);
    console.dir(JSON.parse(vcDocument));
})();
```

We also removed the `wasm.greet();` part, one popup should be enough.

Your project should now resemble the state from the tag ["vade-in-wasm-project"].

When you reload your page, you should get a prompt asking for a VC ID. You can enter a VC ID from evan.network testcore or just use the suggested VC ID. After this the VC is fetched via the wasm module and then rendered in the browser window. As you might guess from the outstanding prettiness of the web page, the author of this guide is not a front-end developer ;).

With this you should now have learned how to add [`vade`] as a wasm module to your web project, how the basic flow works and how to convert the futures from [`vade`] to JavaScript promises.

["new-wasm-project"]: https://github.com/evannetwork/vade-wasm-example/tree/new-wasm-project
["vade-in-wasm-project"]: https://github.com/evannetwork/vade-wasm-example/tree/vade-in-wasm-project
[`vade-evan`]: https://docs.rs/vade-evan
[`vade`]: https://docs.rs/vade
[`VcResolver`]: https://docs.rs/vade-evan/*/vade_evan/plugin/rust_vcresolver_evan/struct.RustVcResolverEvan.html
[Hello, World!]: https://rustwasm.github.io/book/game-of-life/hello-world.html
[Rust and WebAssembly]: https://rustwasm.github.io/book
[setup section]: https://rustwasm.github.io/book/game-of-life/setup.html
