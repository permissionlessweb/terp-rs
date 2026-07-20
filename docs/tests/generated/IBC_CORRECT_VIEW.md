# IBC data — correct preferred information (offline rebuild)

Generated: `2026-07-20T17:47:58.782601+00:00`

Source of truth for **channels**: `public/ibc-data/*.json` (last live-ish snapshot on disk).
Derivation rules: ACTIVE transfer only, **direct preferred single-hop** for known natives, honest `hop_count`.

> Live `cargo run -p scripts --bin ibc -- generate` needs `MAIN_MNEMONIC` in the environment (cw-orch). This rebuild does not re-query mainnet.

## Terp preferred transfer channels

| Counterparty | Terp channel (inbound) | Counterparty channel |
|--------------|------------------------|----------------------|
| akash | `channel-9` | `channel-139` |
| atomone | `channel-10` | `channel-13` |
| evmos | `channel-5` | `channel-101` |
| gitopia | `channel-4` | `channel-1` |
| juno | `channel-0` | `channel-393` |
| omniflix | `channel-3` | `channel-33` |
| osmosis | `channel-11` | `channel-110258` |
| secret | `channel-8` | `channel-165` |
| stargaze | `channel-2` | `channel-235` |

## Direct preferred denoms on Terp

| Symbol | Origin | Path on Terp | IBC denom | hop_count |
|--------|--------|--------------|-----------|-----------|
| AKT | akash/uakt | `transfer/channel-9/uakt` | `ibc/ED3433FEFECE0B9EC584F0EBAB31E8AB6A7A187537F3E5B1D8A80998986E7ACF` | 1 |
| ATONE | atomone/uatone | `transfer/channel-10/uatone` | `ibc/AF34593540E81B06145CD80BDBB2149BE068A9B495086284DFD0FB3017AB5254` | 1 |
| OSMO | osmosis/uosmo | `transfer/channel-11/uosmo` | `ibc/4646A60D9F2EC44B281852B38FE269E2D0870DA7A3BBD805DB5F9CD7AF0D6280` | 1 |

## Reverse: Terp natives on counterparties

| Dest | Symbol | Path | IBC denom |
|------|--------|------|-----------|
| akash | TERP | `transfer/channel-139/uterp` | `ibc/25026C21BD8A1BE658581553FAC5844F344270EFFCC59B673584D6C445006C0B` |
| atomone | TERP | `transfer/channel-13/uterp` | `ibc/65F0CFBA8C3A907DFED26BF41AEDA7935166347BDA64E7D2F8F56F4518BBB523` |
| evmos | TERP | `transfer/channel-101/uterp` | `ibc/3A61569885ED9FA1B7076AB1773CD91FB8EBF88DDC20CF553543CACABBB3EF54` |
| gitopia | TERP | `transfer/channel-1/uterp` | `ibc/4AEA7A5C89B744156996AEE30DAB3CCDCD7039A5625D89C8B7178E6D4C068D57` |
| juno | TERP | `transfer/channel-393/uterp` | `ibc/2578547913C40B0DEBAB7E458F191491DE6288A01E1000E9BAE34103DEB189D3` |
| omniflix | TERP | `transfer/channel-33/uterp` | `ibc/159F6BA84F3EA775A015C81E8B0A7C66E38AAFF75F7407DC654F0C365CDF722E` |
| osmosis | TERP | `transfer/channel-110258/uterp` | `ibc/448408EAC3AEDE3F1EC2261D72B8BAF323098D8489AC033DB614C4BAF221F598` |
| secret | TERP | `transfer/channel-165/uterp` | `ibc/AF840D44CC92103AD006850542368B888D29C4D4FFE24086E767F161FBDDCE76` |
| stargaze | TERP | `transfer/channel-235/uterp` | `ibc/E552E64943BD9869EDC59C456AC8E6DBD56150EFE887E7EF0BC27DFBB78A4004` |
| akash | THIOL | `transfer/channel-139/uthiol` | `ibc/EA1305661ED048B1FD81E326E4C65D12A4333D3D4709F6CCB8A09E82AD968584` |
| atomone | THIOL | `transfer/channel-13/uthiol` | `ibc/4E87F0043C9E3CBF85BEC7104B35E3E6FD9974C9086092921D08E7BC8695B601` |
| evmos | THIOL | `transfer/channel-101/uthiol` | `ibc/9EF5C9E824DB3E21009D52B70EDA142B0679AA8D03584224C6E41CEE3D1DEA2F` |
| gitopia | THIOL | `transfer/channel-1/uthiol` | `ibc/869FC01C559DDC2DC2CD241FBF99115CA60FE7AFC7C56EF0B382CA7B0E1CB5B1` |
| juno | THIOL | `transfer/channel-393/uthiol` | `ibc/055F223602B754BF34FE269B729837A02BDF79E069F2387AEEA9CD28DE961BE7` |
| omniflix | THIOL | `transfer/channel-33/uthiol` | `ibc/0EC361789CFCD850904998126FF47D6AB5D16C3B1ADDFCB4C3C89B2BDBF0B4EE` |
| osmosis | THIOL | `transfer/channel-110258/uthiol` | `ibc/F31D4D773FB7A38971C0C7E99B813F8AD34156D366A40570B558C44BB910BE93` |
| secret | THIOL | `transfer/channel-165/uthiol` | `ibc/7477828AC3E19352BA2D63352EA6D0680E3F29C126B87ACBDC27858CF7AF3A64` |
| stargaze | THIOL | `transfer/channel-235/uthiol` | `ibc/A33D9BB00512256156879103BA1426EF39236C9CCB1415C5AD35B5B740D53E72` |

## Contrast: stale `public/ibc_lookup_table.json` (why we rebuilt)

Examples of **incorrect preferred multi-hop** still in the old lookup (hop_count labeled 1 but path is multi-hop):

| Symbol | Old path | Issues |
|--------|----------|--------|
| ATONE | `transfer/channel-11/transfer/channel-94814/uatone` | hop_count=1 but transfer/ segments=2; not direct prefer |
| ETH | `transfer/channel-11/transfer/channel-0/transfer/08-wasm-1369/0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2` | hop_count=1 but transfer/ segments=3; not direct prefer |
| AKT | `transfer/channel-11/transfer/channel-1/uakt` | hop_count=1 but transfer/ segments=2; not direct prefer |
| THIOL | `transfer/channel-10/transfer/channel-13/uthiol` | hop_count=1 but transfer/ segments=2; not direct prefer |
| UM | `transfer/channel-11/transfer/channel-79703/upenumbra` | hop_count=1 but transfer/ segments=2; not direct prefer |
| THIOL | `transfer/channel-11/transfer/channel-110258/uthiol` | hop_count=1 but transfer/ segments=2; not direct prefer |
| THIOL | `transfer/channel-9/transfer/channel-139/uthiol` | hop_count=1 but transfer/ segments=2; not direct prefer |
| TERP | `transfer/channel-11/transfer/channel-110258/uterp` | hop_count=1 but transfer/ segments=2; not direct prefer |
| TERP | `transfer/channel-10/transfer/channel-13/uterp` | hop_count=1 but transfer/ segments=2; not direct prefer |
| TERP | `transfer/channel-9/transfer/channel-139/uterp` | hop_count=1 but transfer/ segments=2; not direct prefer |

### Corrected AKT / ATONE on Terp (direct)

- **AKT**: `transfer/channel-9/uakt` → `ibc/ED3433FEFECE0B9EC584F0EBAB31E8AB6A7A187537F3E5B1D8A80998986E7ACF`
- **ATONE**: `transfer/channel-10/uatone` → `ibc/AF34593540E81B06145CD80BDBB2149BE068A9B495086284DFD0FB3017AB5254`
- **OSMO**: `transfer/channel-11/uosmo` → `ibc/4646A60D9F2EC44B281852B38FE269E2D0870DA7A3BBD805DB5F9CD7AF0D6280`

### Audited community pin (Osmosis AKT)

- Path: `transfer/channel-1/uakt`
- Denom: `ibc/1480B8FD20AD5FCAE81EA87584D269547DD4D436843C1D20F15E00EB64743EF4`

## Files written

- `docs/tests/generated/corrected_preferred_routes.json`
- `public/ibc_lookup_table.corrected.json`
- `docs/tests/generated/IBC_CORRECT_VIEW.md` (this file)

