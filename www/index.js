import * as wasm from "vade-wasm-example";

(async () => {
    // fetch DID document
    const did = prompt("did", "did:evan:0x16d4508f8ee18fa37781c995f39b6f652ed03fb3d24ed12876ada9fd7d950fb9");
    const didDocument = await wasm.get_did_document(did);
    
    // render DID document
    const div = document.createElement('div');
    div.style['white-space'] = 'pre-wrap';
    div.innerText = JSON.stringify(JSON.parse(didDocument), null, 2);
    document.body.appendChild(div);
    console.dir(JSON.parse(didDocument));
})();