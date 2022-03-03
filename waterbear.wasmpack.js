function handleNewInput(callback, params, wasm, data_link, {preload_files, result_files, buffWasm}) {
  console.log("hni")
  console.log(buffWasm)
    wasm_bindgen(buffWasm).then(
       () => {
              callback(wasm_bindgen.from_wasm(), "", "")
  }
  )
}

function postMessageWrapper(message) {
      try {
            postMessage(message)
      } catch (_) {
            const {parentPort} = require('worker_threads')
            parentPort.postMessage(message)
      }
}

function onWebMessage(e) {
      const callback = (body, stdout, stderr) => postMessageWrapper({body, stdout, stderr})
      handleNewInput(callback, e.data.params, e.data.wasm, e.data.data_link, e.data)
}

function onNodeMessage(data) {
      const callback = (body, stdout, stderr) => postMessageWrapper({body, stdout, stderr})
      handleNewInput(callback, data.params, data.wasm, data.data_link, data)
}

onmessage = onWebMessage
try {
      const {parentPort} = require('worker_threads')
      parentPort.once('message', onNodeMessage);
} catch (_) {}
