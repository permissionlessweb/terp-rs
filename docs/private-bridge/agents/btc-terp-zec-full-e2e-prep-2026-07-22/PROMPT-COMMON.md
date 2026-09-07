# Common prompt fragment — gap analysis only

**Workspace:** `/Users/returniflost/abstract/terp-core`  
**Greenlight:** GO for **gap report** (read/code search; no large implementation).  
**Execution:** Grok subagent — not Hermes workers.

## Required output structure (`STATUS-GAP-<TRACK>.md`)

```markdown
# STATUS-GAP-<TRACK>

## 1. Goal for full BTC → Terp → ZEC (this track)
...

## 2. Code reality today (cite paths / commands)
| Surface | Status | Evidence |
...

## 3. Gaps (goal vs code)
| ID | Gap | Severity (P0/P1/P2) | Notes |
...

## 4. Dependencies on other tracks
...

## 5. Recommended P0 slice for this track
...

## 6. Explicit non-claims
...
```

## Shared ground truth (do not invent)

**Green today (`just demo-corridor-ict` default):**

1. BTC **regtest** fund → reporter → `deposit_observed`  
2. ict-rs Terp + Daemon **BridgeMintNote** (`IsBridgeMinted`)  
3. **Pure** cashapp W0–W7 + automation phases (swap **film**, not ZEC chain egress)  

**Not green as full multi-net ZEC:**

- Live Zcash transfer / Zakura broadcast  
- On-chain private DEX settle from minted note  
- Single continuous identity: deposit observation fields → BridgeMint claim (may be sequenced but not fully coupled)  
- Mainnet Cash App / mainnet money  

## Tone

Engineering gap analysis: precise identifiers, tables, paths. No marketing.
