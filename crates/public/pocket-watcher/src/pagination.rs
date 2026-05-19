use cosmwasm_std::{Binary, Deps, GrpcQuery, QueryRequest};
use prost::Message;

use crate::error::{QueryError, QueryResult};

// ====================== Common Pagination Types ======================

#[derive(Clone, PartialEq, Message)]
pub struct PageRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub key: Vec<u8>,
    #[prost(uint64, tag = "2")]
    pub offset: u64,
    #[prost(uint64, tag = "3")]
    pub limit: u64,
    #[prost(bool, tag = "4")]
    pub count_total: bool,
    #[prost(bool, tag = "5")]
    pub reverse: bool,
}

#[derive(Clone, PartialEq, Message)]
pub struct PageResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub next_key: Vec<u8>,
    #[prost(uint64, tag = "2")]
    pub total: u64,
}

// ====================== Helpers ======================

pub(crate) fn to_binary<T: Message>(msg: &T) -> Result<Binary, QueryError> {
    let mut buf = Vec::new();
    msg.encode(&mut buf)?;
    Ok(Binary::from(buf))
}

pub(crate) fn from_binary<T: Message + Default>(data: Binary) -> Result<T, QueryError> {
    T::decode(data.as_ref()).map_err(Into::into)
}

// ====================== Generic Paginator ======================

/// Generic paginated gRPC query helper, inspired by cw-storage-plus `paginate_map`.
///
/// - `path`: gRPC path (e.g. `"/cosmos.bank.v1beta1.Query/AllBalances"`)
/// - `make_request`: builds the request protobuf for a given [`PageRequest`]
/// - `extract`: extracts result items + next pagination key from the response
/// - `limit`: max total items to return (`None` = fetch all pages)
pub fn paginate_grpc_query<Req, Resp, Item, F1, F2>(
    deps: Deps,
    path: &str,
    mut make_request: F1,
    mut extract: F2,
    limit: Option<u32>,
) -> QueryResult<Vec<Item>>
where
    Req: Message,
    Resp: Message + Default,
    F1: FnMut(PageRequest) -> Req,
    F2: FnMut(Resp) -> QueryResult<(Vec<Item>, Option<Vec<u8>>)>,
{
    let mut all_items = Vec::new();
    let mut next_key: Option<Vec<u8>> = None;
    let page_limit = limit.unwrap_or(100).min(1000) as u64;

    loop {
        let page_req = PageRequest {
            key: next_key.unwrap_or_default(),
            offset: 0,
            limit: page_limit,
            count_total: false,
            reverse: false,
        };

        let req = make_request(page_req);
        let query: QueryRequest = QueryRequest::Grpc(GrpcQuery {
            path: path.to_string(),
            data: to_binary(&req)?,
        });

        let raw_resp: Binary = deps.querier.query(&query)?;
        let resp: Resp = from_binary(raw_resp)?;
        let (items, new_next_key) = extract(resp)?;

        all_items.extend(items);

        if let Some(max) = limit {
            if all_items.len() >= max as usize {
                all_items.truncate(max as usize);
                break;
            }
        }

        next_key = new_next_key;
        if next_key.as_ref().map_or(true, |k| k.is_empty()) {
            break;
        }
    }

    Ok(all_items)
}
