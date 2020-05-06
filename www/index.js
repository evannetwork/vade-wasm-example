import * as wasm from "vade-wasm-example";

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