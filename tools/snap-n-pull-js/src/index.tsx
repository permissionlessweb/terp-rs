import {
  OnRpcRequestHandler,
  OnUserInputHandler,
  UserInputEventType,
} from '@metamask/snaps-sdk';

import type { InitOutput } from '@terpnetwork/snap-n-pull';
import { initialiseWasm } from './utils/initialiseWasm';
import { gn, type GenerateNullifierParams } from './rpc/gn';
import { fetchNote, fetchHeadstashRecord, pirFetchNote } from './utils/headstashApi';
import { loadCircuitKey } from './utils/circuitKeys';

let wasm: InitOutput | null = null;

/**
 * Initialize WASM module if not already initialized
 */
async function ensureWasmInitialized(): Promise<InitOutput> {
  if (!wasm) {
    wasm = initialiseWasm();
  }
  return wasm;
}

/**
 * Handle incoming JSON-RPC requests from dApps
 *
 * Supported methods:
 * - generateNullifier: Generate note nullifier for headstash claim
 */
export const onRpcRequest: OnRpcRequestHandler = async ({
  request,
  origin,
}) => {
  // Ensure WASM is initialized
  const wasmModule = await ensureWasmInitialized();

  switch (request.method) {
    case 'generateNullifier': {
      // Validate params
      if (!request.params || typeof request.params !== 'object') {
        throw new Error('Invalid params: expected object');
      }

      const params = request.params as GenerateNullifierParams;

      // Validate required fields
      if (!params.headstashId) {
        throw new Error('Missing required field: headstashId');
      }
      if (!params.noteInputs) {
        throw new Error('Missing required field: noteInputs');
      }
      if (!params.noteInputs.recp) {
        throw new Error('Missing required field: noteInputs.recp');
      }
      if (!params.noteInputs.nd) {
        throw new Error('Missing required field: noteInputs.nd');
      }
      if (!params.noteInputs.v) {
        throw new Error('Missing required field: noteInputs.v');
      }
      if (params.noteInputs.fdi === undefined) {
        throw new Error('Missing required field: noteInputs.fdi');
      }
      if (!params.noteInputs.rho) {
        throw new Error('Missing required field: noteInputs.rho');
      }
      if (!params.noteInputs.rseed) {
        throw new Error('Missing required field: noteInputs.rseed');
      }

      // Generate nullifier
      return await gn(wasmModule, params, origin);
    }

    // ── fetchNote ──────────────────────────────────────────────────────
    case 'fetchNote': {
      const p = request.params as {
        headstashId: string;
        addr: string;
        serverUrl?: string;
        usePir?: boolean;
      };
      if (!p?.headstashId || !p?.addr) {
        throw new Error('fetchNote requires headstashId and addr');
      }

      if (p.usePir) {
        const result = await pirFetchNote(p.headstashId, p.addr, {
          serverUrl: p.serverUrl,
        });
        return { result };
      }

      return fetchNote(p.headstashId, p.addr, { serverUrl: p.serverUrl });
    }

    // ── fetchCircuitKey ────────────────────────────────────────────────
    case 'fetchCircuitKey': {
      const p = request.params as {
        headstashId: string;
        keyType: 'vk' | 'pk';
      };
      if (!p?.headstashId || !p?.keyType) {
        throw new Error('fetchCircuitKey requires headstashId and keyType');
      }
      const key = await loadCircuitKey(p.headstashId, p.keyType);
      return { key };
    }

    // ── checkEligibility ──────────────────────────────────────────────
    case 'checkEligibility': {
      const p = request.params as {
        headstashId: string;
        serverUrl?: string;
      };
      if (!p?.headstashId) {
        throw new Error('checkEligibility requires headstashId');
      }
      // Fetch headstash record (public endpoint — no auth needed)
      const record = await fetchHeadstashRecord(p.headstashId, {
        serverUrl: p.serverUrl,
      });
      return record;
    }

    default:
      throw new Error(`Method not found: ${request.method}`);
  }
};