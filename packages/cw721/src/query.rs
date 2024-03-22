use cosmwasm_std::{
    to_json_binary, Addr, Binary, BlockInfo, Deps, Empty, Env, Order, StdError, StdResult, Storage,
};
use cw_ownable::Ownership;
use cw_storage_plus::Bound;
use cw_utils::{maybe_addr, Expiration};

use crate::{
    error::Cw721ContractError,
    msg::{
        AllNftInfoResponse, ApprovalResponse, ApprovalsResponse, Cw721QueryMsg, MinterResponse,
        NftInfoResponse, NumTokensResponse, OperatorResponse, OperatorsResponse, OwnerOfResponse,
        TokensResponse,
    },
    state::{
        Approval, CollectionMetadata, CollectionMetadataAndExtension, Cw721Config, NftInfo,
        CREATOR, MINTER,
    },
    traits::{Cw721State, FromAttributes},
    Attribute,
};

pub const DEFAULT_LIMIT: u32 = 10;
pub const MAX_LIMIT: u32 = 1000;

pub fn parse_approval(item: StdResult<(Addr, Expiration)>) -> StdResult<Approval> {
    item.map(|(spender, expires)| Approval { spender, expires })
}

pub fn humanize_approvals<TNftMetadataExtension>(
    block: &BlockInfo,
    nft_info: &NftInfo<TNftMetadataExtension>,
    include_expired_approval: bool,
) -> Vec<Approval>
where
    TNftMetadataExtension: Cw721State,
{
    nft_info
        .approvals
        .iter()
        .filter(|apr| include_expired_approval || !apr.is_expired(block))
        .map(humanize_approval)
        .collect()
}

pub fn humanize_approval(approval: &Approval) -> Approval {
    Approval {
        spender: approval.spender.clone(),
        expires: approval.expires,
    }
}

// --- query helpers ---
#[deprecated(since = "0.19.0", note = "Please use query_minter_ownership instead")]
/// Deprecated: use query_minter_ownership instead! Will be removed in next release!
pub fn query_minter(storage: &dyn Storage) -> StdResult<MinterResponse> {
    let minter = MINTER
        .get_ownership(storage)?
        .owner
        .map(|a| a.into_string());

    Ok(MinterResponse { minter })
}

pub fn query_minter_ownership(storage: &dyn Storage) -> StdResult<Ownership<Addr>> {
    MINTER.get_ownership(storage)
}

pub fn query_creator_ownership(storage: &dyn Storage) -> StdResult<Ownership<Addr>> {
    CREATOR.get_ownership(storage)
}

pub fn query_collection_metadata(storage: &dyn Storage) -> StdResult<CollectionMetadata> {
    let config =
        Cw721Config::<Option<Empty>, Option<Empty>, Option<Empty>, Option<Empty>>::default();
    config.collection_metadata.load(storage)
}

pub fn query_collection_metadata_extension(deps: Deps) -> StdResult<Vec<Attribute>> {
    let config =
        Cw721Config::<Option<Empty>, Option<Empty>, Option<Empty>, Option<Empty>>::default();
    cw_paginate_storage::paginate_map_values(
        deps,
        &config.collection_metadata_extension,
        None,
        None,
        Order::Ascending,
    )
}

pub fn query_collection_metadata_and_extension<TCollectionMetadataExtension>(
    deps: Deps,
) -> Result<CollectionMetadataAndExtension<TCollectionMetadataExtension>, Cw721ContractError>
where
    TCollectionMetadataExtension: FromAttributes,
{
    let collection_metadata = query_collection_metadata(deps.storage)?;
    let attributes = query_collection_metadata_extension(deps)?;
    let extension = FromAttributes::from_attributes(&attributes)?;
    Ok(CollectionMetadataAndExtension {
        name: collection_metadata.name,
        symbol: collection_metadata.symbol,
        updated_at: collection_metadata.updated_at,
        extension,
    })
}

pub fn query_num_tokens(deps: Deps, _env: &Env) -> StdResult<NumTokensResponse> {
    let count =
        Cw721Config::<Option<Empty>, Option<Empty>, Option<Empty>, Option<Empty>>::default()
            .token_count(deps.storage)?;
    Ok(NumTokensResponse { count })
}

pub fn query_nft_info<TNftMetadataExtension>(
    deps: Deps,
    _env: &Env,
    token_id: String,
) -> StdResult<NftInfoResponse<TNftMetadataExtension>>
where
    TNftMetadataExtension: Cw721State,
{
    let info =
        Cw721Config::<TNftMetadataExtension, Option<Empty>, Option<Empty>, Option<Empty>>::default(
        )
        .nft_info
        .load(deps.storage, &token_id)?;
    Ok(NftInfoResponse {
        token_uri: info.token_uri,
        extension: info.extension,
    })
}

pub fn query_owner_of(
    deps: Deps,
    env: &Env,
    token_id: String,
    include_expired_approval: bool,
) -> StdResult<OwnerOfResponse> {
    let nft_info =
        Cw721Config::<Option<Empty>, Option<Empty>, Option<Empty>, Option<Empty>>::default()
            .nft_info
            .load(deps.storage, &token_id)?;
    Ok(OwnerOfResponse {
        owner: nft_info.owner.to_string(),
        approvals: humanize_approvals(&env.block, &nft_info, include_expired_approval),
    })
}

/// operator returns the approval status of an operator for a given owner if exists
pub fn query_operator(
    deps: Deps,
    env: &Env,
    owner: String,
    operator: String,
    include_expired_approval: bool,
) -> StdResult<OperatorResponse> {
    let owner_addr = deps.api.addr_validate(&owner)?;
    let operator_addr = deps.api.addr_validate(&operator)?;

    let info = Cw721Config::<Option<Empty>, Option<Empty>, Option<Empty>, Option<Empty>>::default()
        .operators
        .may_load(deps.storage, (&owner_addr, &operator_addr))?;

    if let Some(expires) = info {
        if !include_expired_approval && expires.is_expired(&env.block) {
            return Err(StdError::not_found("Approval not found"));
        }

        return Ok(OperatorResponse {
            approval: Approval {
                spender: operator_addr,
                expires,
            },
        });
    }

    Err(StdError::not_found("Approval not found"))
}

/// operators returns all operators owner given access to
pub fn query_operators(
    deps: Deps,
    env: &Env,
    owner: String,
    include_expired_approval: bool,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<OperatorsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start_addr = maybe_addr(deps.api, start_after)?;
    let start = start_addr.as_ref().map(Bound::exclusive);

    let owner_addr = deps.api.addr_validate(&owner)?;
    let res: StdResult<Vec<_>> =
        Cw721Config::<Option<Empty>, Option<Empty>, Option<Empty>, Option<Empty>>::default()
            .operators
            .prefix(&owner_addr)
            .range(deps.storage, start, None, Order::Ascending)
            .filter(|r| {
                include_expired_approval
                    || r.is_err()
                    || !r.as_ref().unwrap().1.is_expired(&env.block)
            })
            .take(limit)
            .map(parse_approval)
            .collect();
    Ok(OperatorsResponse { operators: res? })
}

pub fn query_approval(
    deps: Deps,
    env: &Env,
    token_id: String,
    spender: String,
    include_expired_approval: bool,
) -> StdResult<ApprovalResponse> {
    let token =
        Cw721Config::<Option<Empty>, Option<Empty>, Option<Empty>, Option<Empty>>::default()
            .nft_info
            .load(deps.storage, &token_id)?;

    // token owner has absolute approval
    if token.owner == spender {
        let approval = Approval {
            spender: token.owner,
            expires: Expiration::Never {},
        };
        return Ok(ApprovalResponse { approval });
    }

    let filtered: Vec<_> = token
        .approvals
        .into_iter()
        .filter(|t| t.spender == spender)
        .filter(|t| include_expired_approval || !t.is_expired(&env.block))
        .map(|a| Approval {
            spender: a.spender,
            expires: a.expires,
        })
        .collect();

    if filtered.is_empty() {
        return Err(StdError::not_found("Approval not found"));
    }
    // we expect only one item
    let approval = filtered[0].clone();

    Ok(ApprovalResponse { approval })
}

/// approvals returns all approvals owner given access to
pub fn query_approvals(
    deps: Deps,
    env: &Env,
    token_id: String,
    include_expired_approval: bool,
) -> StdResult<ApprovalsResponse> {
    let token =
        Cw721Config::<Option<Empty>, Option<Empty>, Option<Empty>, Option<Empty>>::default()
            .nft_info
            .load(deps.storage, &token_id)?;
    let approvals: Vec<_> = token
        .approvals
        .into_iter()
        .filter(|t| include_expired_approval || !t.is_expired(&env.block))
        .map(|a| Approval {
            spender: a.spender,
            expires: a.expires,
        })
        .collect();

    Ok(ApprovalsResponse { approvals })
}

pub fn query_tokens(
    deps: Deps,
    _env: &Env,
    owner: String,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<TokensResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(|s| Bound::ExclusiveRaw(s.into()));

    let owner_addr = deps.api.addr_validate(&owner)?;
    let tokens: Vec<String> =
        Cw721Config::<Option<Empty>, Option<Empty>, Option<Empty>, Option<Empty>>::default()
            .nft_info
            .idx
            .owner
            .prefix(owner_addr)
            .keys(deps.storage, start, None, Order::Ascending)
            .take(limit)
            .collect::<StdResult<Vec<_>>>()?;

    Ok(TokensResponse { tokens })
}

pub fn query_all_tokens(
    deps: Deps,
    _env: &Env,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<TokensResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(|s| Bound::ExclusiveRaw(s.into()));

    let tokens: StdResult<Vec<String>> =
        Cw721Config::<Option<Empty>, Option<Empty>, Option<Empty>, Option<Empty>>::default()
            .nft_info
            .range(deps.storage, start, None, Order::Ascending)
            .take(limit)
            .map(|item| item.map(|(k, _)| k))
            .collect();

    Ok(TokensResponse { tokens: tokens? })
}

pub fn query_all_nft_info<TNftMetadataExtension>(
    deps: Deps,
    env: &Env,
    token_id: String,
    include_expired_approval: bool,
) -> StdResult<AllNftInfoResponse<TNftMetadataExtension>>
where
    TNftMetadataExtension: Cw721State,
{
    let nft_info =
        Cw721Config::<TNftMetadataExtension, Option<Empty>, Option<Empty>, Option<Empty>>::default(
        )
        .nft_info
        .load(deps.storage, &token_id)?;
    Ok(AllNftInfoResponse {
        access: OwnerOfResponse {
            owner: nft_info.owner.to_string(),
            approvals: humanize_approvals(&env.block, &nft_info, include_expired_approval),
        },
        info: NftInfoResponse {
            token_uri: nft_info.token_uri,
            extension: nft_info.extension,
        },
    })
}

pub fn query_withdraw_address(deps: Deps) -> StdResult<Option<String>> {
    Cw721Config::<Option<Empty>, Option<Empty>, Option<Empty>, Option<Empty>>::default()
        .withdraw_address
        .may_load(deps.storage)
}
