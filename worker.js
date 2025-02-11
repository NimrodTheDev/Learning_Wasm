import * as wasm from "./pkg/WasmExamples.js"

self.addEventListener('message', function({data: bytes}) {
  wasm.initSync({module: bytes})
  wasm.add(1, 2)
  wasm.run()
}, false);

self.postMessage({action: "fetch_wasm"})