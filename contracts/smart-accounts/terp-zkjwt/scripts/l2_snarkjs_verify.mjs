#!/usr/bin/env node
/**
 * L2 crypto gate: snarkjs verify of committed jwt-auth fixtures.
 * Does not require the 700MB zkey (verify uses vkey only).
 *
 *   node scripts/l2_snarkjs_verify.mjs
 */
import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";
import { createRequire } from "module";

const require = createRequire(import.meta.url);
const snarkjs = require("snarkjs");

const __dirname = dirname(fileURLToPath(import.meta.url));
const fix = join(__dirname, "../src/fixtures/l2");

const vkey = JSON.parse(readFileSync(join(fix, "jwt-auth_vkey.json"), "utf8"));
const proof = JSON.parse(readFileSync(join(fix, "proof.json"), "utf8"));
const pub = JSON.parse(readFileSync(join(fix, "public.json"), "utf8"));

const ok = await snarkjs.groth16.verify(vkey, pub, proof);
if (!ok) {
  console.error("L2 FAIL: snarkjs groth16.verify returned false");
  process.exit(1);
}
console.log("L2 OK: snarkjs verify true");
console.log("  nPublic =", pub.length);
console.log("  nullifier (idx4) =", pub[4]);
console.log("  accountSalt (idx26) =", pub[26]);
process.exit(0);
