import { decode } from "./mod.ts";

Deno.test("decoding", async () => {
  console.log(decode(new TextEncoder().encode((await Deno.readTextFile("metadata")).split("0x").pop()))[1].V14.pallets);
});
