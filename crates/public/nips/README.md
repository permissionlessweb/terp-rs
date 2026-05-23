# Cw721-NIPS

## TODO
- prelude helper design
- minimize type/trait bloat surface area for implement nip `Kind` (blanet impl of PartialEq,From,Into for u16)
- test suite w/ small nostr network instance
- trailscale model review diagrams and spec
- demo **ALL** connection methods with nostr relay / relay client with node 

## How Do We Use This Library?

All nip metadata objects are expected to implement two traits:

1. `NipMetadata`: This provides useful NIP-01 compatible helpers in regards to event serialization and validation. 
2. `NostrExt` : Allows design to respect users decision to make use of on or off chain metadata.

## How Do We Test The Logic In This Library?

With `ict-rs`, we manage local nostr relayer and client containers alongside any chain scripting. OUr scripts are optimized for compatibility with `cw-orchestrator`.

## How Do We Add New Support For This Library?



## Nip Support Matrix 

| NIP | Name | Status | Implementation Notes |
|-----|------|--------|---------------------|
| NIP-01 | Basic protocol flow description | 🔄 | Core protocol events and message types |
| NIP-52 | Calendar Events | 📅 | Scheduling |
| NIP-15 | Nostr Marketplace | 📦 | Vending |
<!-- | NIP-02 | Follow List | 📋 | Social graph functionality |
| NIP-04 | Encrypted Direct Message | 🔐 | Basic encryption (consider NIP-17 instead) |
| NIP-05 | Mapping Nostr keys to DNS identifiers | 🌐 | Identity verification |
| NIP-07 | window.nostr capability | 🖥️ | Browser integration |
| NIP-09 | Event Deletion Request | 🗑️ | Content management |
| NIP-10 | Text Notes and Threads | 💬 | Conversation threading |
| NIP-11 | Relay Information Document | ℹ️ | Relay metadata |
| NIP-19 | bech32-encoded entities | 📝 | Key/address encoding |
| NIP-21 | nostr: URI scheme | 🔗 | Deep linking |
| NIP-13 | Proof of Work | ⛏️ | Content validation |
| NIP-17 | Private Direct Messages | 🔒 | Modern DM implementation |
| NIP-18 | Reposts | 🔄 | Content sharing |
| NIP-23 | Long-form Content | 📄 | Article support |
| NIP-25 | Reactions | 👍 | Social interactions |
| NIP-26 | Delegated Event Signing | 📜 | Delegation support |
| NIP-27 | Text Note References | 🔗 | Mention/quote system |
| NIP-28 | Public Chat | 💬 | Group communication |
| NIP-40 | Expiration Timestamp | ⏰ | Time-limited content |
| NIP-42 | Authentication | 🔐 | Relay access control |
| NIP-29 | Relay-based Groups | 👥 | Community management |
| NIP-30 | Custom Emoji | 😀 | Rich content |
| NIP-32 | Labeling | 🏷️ | Content categorization |
| NIP-34 | git stuff | 📦 | Version control |
| NIP-36 | Sensitive Content | ⚠️ | Content warnings |
| NIP-38 | User Statuses | 📊 | Presence indicators |
| NIP-44 | Encrypted Payloads (Versioned) | 🔐 | Advanced encryption |
| NIP-45 | Counting results | 🔢 | Query optimization |
| NIP-46 | Nostr Remote Signing | 🔑 | Key management |
| NIP-47 | Nostr Wallet Connect | 💰 | Lightning integration |
| NIP-51 | Lists | 📋 | User-managed collections |
| NIP-53 | Live Activities | 🎥 | Real-time events |
| NIP-54 | Wiki | 📖 | Knowledge base |
| NIP-57 | Lightning Zaps | ⚡ | Micropayments |
| NIP-58 | Badges | 🏆 | Achievement system |
| NIP-59 | Gift Wrap | 🎁 | Encrypted content |
| NIP-65 | Relay List Metadata | 📋 | Relay management |
| NIP-70 | Protected Events | 🔒 | Access control | -->
