# pocket-watcher

Generic pagination helpers for gRPC module queries in CosmWasm v3 contracts.

## Why?

CosmWasm v3 (`cosmwasm-std >= 3.0`) removed `BankQuery::AllBalances` from the native query enum.
The underlying gRPC endpoint (`/cosmos.bank.v1beta1.Query/AllBalances`) still exists on every
Cosmos chain — you just need to call it via `QueryRequest::Grpc` and handle protobuf
encoding + pagination yourself.

**pocket-watcher** hides that boilerplate behind a clean API inspired by
`cw-storage-plus`'s `paginate_map`.

## Migration from v2

Replace this (no longer compiles on cosmwasm-std v3):

```rust,ignore
// BEFORE — cosmwasm-std v2
let balances: AllBalancesResponse = deps.querier.query(
    &QueryRequest::Bank(BankQuery::AllBalances {
        address: env.contract.address.to_string(),
    }),
)?;
```

With this:

```rust,ignore
// AFTER — cosmwasm-std v3 + pocket-watcher
use pocket_watcher::query_all_balances;

let balances = query_all_balances(deps.as_ref(), &env.contract.address)
    .map_err(|e| StdError::generic_err(e.to_string()))?;
```

That's it. `query_all_balances` automatically paginates through all pages and
returns `Vec<Coin>`.

## API

### High-level helpers

```rust,ignore
// All balances (paginated automatically)
let coins: Vec<Coin> = query_all_balances(deps, address)?;

// All balances with a max item limit
let coins: Vec<Coin> = query_all_balances_limited(deps, address, 50)?;

// Single denom balance via gRPC
let coin: Coin = query_balance(deps, address, "uterp")?;
```

### Generic paginator

For any gRPC module query that uses cosmos-sdk pagination:

```rust,ignore
use pocket_watcher::{paginate_grpc_query, PageRequest};

let items = paginate_grpc_query(
    deps,
    "/cosmos.staking.v1beta1.Query/DelegatorDelegations",
    |page_req| MyDelegationsRequest {
        delegator_addr: addr.clone(),
        pagination: Some(page_req),
    },
    |resp: MyDelegationsResponse| {
        let next_key = resp.pagination
            .and_then(|p| if p.next_key.is_empty() { None } else { Some(p.next_key) });
        Ok((resp.delegation_responses, next_key))
    },
    None, // fetch all pages
)?;
```

## Cargo.toml

```toml
[dependencies]
pocket-watcher = { path = "../pocket-watcher" }
# or when published:
# pocket-watcher = "0.1"
```

Requires `cosmwasm-std` with `stargate` and `cosmwasm_2_0` features (for `GrpcQuery`).
