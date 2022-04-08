import * as hex from "https://deno.land/std@0.134.0/encoding/hex.ts";
import { decode as decodeRaw } from "./target/wasm/wasm_bindgen.js";

export const decode = (bytes: Uint8Array) => {
  return JSON.parse(decodeRaw(hex.decode(bytes)));
};
