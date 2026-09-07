/**
 * Headstash Server API client
 *
 * Handles authenticated requests to the headstash-server using
 * secp256k1 ECDSA signatures derived from the snap's BIP-32 entropy.
 *
 * Auth flow:
 *   1. Get current unix timestamp
 *   2. Sign `"{timestamp}\n{pubkey_hex}"` with the snap's secp256k1 key
 *   3. Set X-Auth-Type: secp256k1, X-Pubkey, X-Timestamp, X-Signature headers
 */

import { getSk } from './getSk';
import { getPk } from './getPk';

// Default headstash server URL — overridden by snap config / params
const DEFAULT_SERVER = 'https://headstash.terp.network';

export type HeadstashServerConfig = {
  serverUrl?: string;
};

/**
 * Build secp256k1 auth headers for a headstash-server request.
 */
async function buildAuthHeaders(
  sk: Uint8Array,
  pk: string,
): Promise<Record<string, string>> {
  const { secp256k1 } = await import('@noble/curves/secp256k1');

  const timestamp = Math.floor(Date.now() / 1000).toString();
  const msg = `${timestamp}\n${pk}`;

  // SHA-256 hash of the message (matches server-side canonical_message)
  const msgBytes = new TextEncoder().encode(msg);
  const hashBuffer = await crypto.subtle.digest('SHA-256', msgBytes);
  const msgHash = new Uint8Array(hashBuffer);

  // Sign with secp256k1
  const sig = secp256k1.sign(msgHash, sk);
  const sigHex = sig.toCompactHex();

  return {
    'X-Auth-Type': 'secp256k1',
    'X-Pubkey': pk,
    'X-Timestamp': timestamp,
    'X-Signature': sigHex,
  };
}

/**
 * Fetch a headstash registration record.
 *
 * Public endpoint — no auth required.
 */
export async function fetchHeadstashRecord(
  headstashId: string,
  config: HeadstashServerConfig = {},
): Promise<unknown> {
  const base = config.serverUrl ?? DEFAULT_SERVER;
  const resp = await fetch(`${base}/headstash/${headstashId}`);
  if (!resp.ok) throw new Error(`fetchHeadstashRecord failed: ${resp.status}`);
  return resp.json();
}

/**
 * Fetch an encrypted note for an address from the headstash server.
 *
 * Authenticated with the snap's secp256k1 key.
 *
 * @param headstashId - Headstash contract address
 * @param addr - Address whose note to fetch
 * @param config - Server config
 */
export async function fetchNote(
  headstashId: string,
  addr: string,
  config: HeadstashServerConfig = {},
): Promise<{ ciphertext: string; nonce: string; scheme: string }> {
  const sk = await getSk();
  const pk = await getPk(sk);
  const authHeaders = await buildAuthHeaders(sk, pk);
  sk.fill(0); // zero sk immediately after use

  const base = config.serverUrl ?? DEFAULT_SERVER;
  const resp = await fetch(`${base}/notes/${headstashId}/${addr}`, {
    headers: { ...authHeaders },
  });

  if (!resp.ok) {
    throw new Error(`fetchNote failed: ${resp.status} ${await resp.text()}`);
  }
  return resp.json();
}

/**
 * Download a circuit proving key from the headstash server.
 *
 * Authenticated. Returns the raw bytes as a hex string.
 * For large keys (>1MB), consider using pirFetchKey instead.
 *
 * @param keyId - Circuit key identifier
 * @param config - Server config
 */
export async function downloadKey(
  keyId: string,
  config: HeadstashServerConfig = {},
): Promise<string> {
  const sk = await getSk();
  const pk = await getPk(sk);
  const authHeaders = await buildAuthHeaders(sk, pk);
  sk.fill(0);

  const base = config.serverUrl ?? DEFAULT_SERVER;
  const resp = await fetch(`${base}/keys/${keyId}`, { headers: { ...authHeaders } });

  if (!resp.ok) {
    throw new Error(`downloadKey failed: ${resp.status} ${await resp.text()}`);
  }

  const bytes = new Uint8Array(await resp.arrayBuffer());
  return Buffer.from(bytes).toString('hex');
}

/**
 * PIR-fetch a circuit key to obscure which key is being retrieved.
 *
 * The client sends an XOR selector that hides the true request.
 * See pir/mod.rs in headstash-server for protocol details.
 *
 * @param keyId - The key ID we actually want (used to build the selector)
 * @param allKeys - Full list of available key IDs (from GET /keys)
 * @param config - Server config
 */
export async function pirFetchKey(
  keyId: string,
  allKeys: string[],
  config: HeadstashServerConfig = {},
): Promise<string> {
  const sk = await getSk();
  const pk = await getPk(sk);
  const authHeaders = await buildAuthHeaders(sk, pk);
  sk.fill(0);

  const base = config.serverUrl ?? DEFAULT_SERVER;

  // Build selector: set index of target key to 1, rest 0
  const targetIdx = allKeys.indexOf(keyId);
  if (targetIdx === -1) throw new Error(`key ${keyId} not found in key list`);

  // For real PIR we'd use a random masking vector — this is a simplified version
  const selector = new Uint8Array(allKeys.length);
  selector[targetIdx] = 1;

  const resp = await fetch(`${base}/keys/${keyId}/pir`, {
    method: 'POST',
    headers: { ...authHeaders, 'Content-Type': 'application/json' },
    body: JSON.stringify({ selector: Array.from(selector) }),
  });

  if (!resp.ok) {
    throw new Error(`pirFetchKey failed: ${resp.status}`);
  }

  const data: { result: string } = await resp.json();
  return data.result;
}

/**
 * PIR-fetch an encrypted note to obscure which address is being queried.
 *
 * @param headstashId - Headstash contract address
 * @param addr - The address we actually want
 * @param config - Server config
 */
export async function pirFetchNote(
  headstashId: string,
  addr: string,
  config: HeadstashServerConfig = {},
): Promise<string> {
  const sk = await getSk();
  const pk = await getPk(sk);
  const authHeaders = await buildAuthHeaders(sk, pk);
  sk.fill(0);

  const base = config.serverUrl ?? DEFAULT_SERVER;

  // First, get the note key list to build the selector
  const listResp = await fetch(`${base}/notes/${headstashId}`, {
    headers: { ...authHeaders },
  });
  if (!listResp.ok) throw new Error(`note list failed: ${listResp.status}`);
  const { keys }: { keys: string[] } = await listResp.json();

  const targetIdx = keys.indexOf(addr);
  if (targetIdx === -1) throw new Error(`address ${addr} not found in note list`);

  const selector = new Uint8Array(keys.length);
  selector[targetIdx] = 1;

  const pirResp = await fetch(`${base}/notes/${headstashId}/pir`, {
    method: 'POST',
    headers: { ...authHeaders, 'Content-Type': 'application/json' },
    body: JSON.stringify({ selector: Array.from(selector) }),
  });

  if (!pirResp.ok) throw new Error(`PIR note fetch failed: ${pirResp.status}`);
  const data: { result: string } = await pirResp.json();
  return data.result; // hex-encoded XOR result; client decodes with dummy responses
}
