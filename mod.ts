import * as hex from "std/encoding/hex.ts";
import { decode as decodeRaw } from "./target/wasm/wasm_bindgen.js";

export const decode = (bytes: Uint8Array) => {
  return JSON.parse(decodeRaw(hex.decode(bytes)));
};
