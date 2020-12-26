const wasm = import("./node_modules/wasm-typescript/wasm_typescript.js")
wasm.then(module => {
  console.log(module.sum(61, 90))
})