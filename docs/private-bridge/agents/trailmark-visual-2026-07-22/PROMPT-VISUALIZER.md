# PROMPT — VISUALIZER (Mermaid → SVG)

**Skills:** crypto-protocol-diagram references + mermaid-cli

## Mission

Ship static SVGs for every workflow diagram and embed them in book pages with precise captions.

## Tasks

1. Run `cd docs/plans/spectrum/book && just diagrams-svg` (extend justfile if needed).  
2. Ensure workflows/index.md references all SVGs with short, precise captions.  
3. Add a compact **reject-path** diagram (I1–I6) if missing.  
4. Prefer SVG in book HTML; keep `.mmd` as source of truth.  
5. `STATUS-VISUALIZER.md`.

## Quality

- Sequence diagrams: parties, phases, fail-closed / reject alts  
- No marketing language (“film”, “magic”)  
- Labels use real identifiers: domain_bind, owner_binding, BridgeMintNote  
