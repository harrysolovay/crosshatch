use js_sys::Uint8Array;

use {
  console_error_panic_hook::set_once,
  frame_metadata::RuntimeMetadataPrefixed,
  js_sys::JsString,
  parity_scale_codec::Decode,
  wasm_bindgen::{prelude::wasm_bindgen, JsError},
  wee_alloc::WeeAlloc,
};

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
pub fn decode(bytes: Uint8Array) -> Result<JsString, JsError> {
  set_once();
  let mut slice = vec![0; bytes.length() as usize];
  bytes.copy_to(&mut slice);
  let decoded = RuntimeMetadataPrefixed::decode(&mut &*slice)?;
  let serialized = serde_json::to_string(&decoded)?;
  Ok(JsString::from(serialized))
}
