use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CustomMsg, Empty, StdError};
pub use cw721_base::{InstantiateMsg, MinterResponse};
use thiserror::Error;

// Version info for migration
const CONTRACT_NAME: &str = "crates.io:{{project-name}}";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Implements extended on-chain metadata, by default cw721 NFTs only store a
// token_uri, which is a URL to off-chain metadata (same as ERC721).
#[cw_serde]
#[derive(Default)]
pub struct MetadataExt {}

// This is the custom Execute message extension for this contract.
// Use it to implement custom functionality.
#[cw_serde]
pub enum ExecuteExt {}
impl CustomMsg for ExecuteExt {}

// This is the custom Query message type for this contract.
// Use it to implement custom query messages.
#[cw_serde]
pub enum QueryExt {}
impl CustomMsg for QueryExt {}

// This contrains default cw721 logic with extensions.
// If you don't need a particular extension, replace it with an
// `Empty` type.
pub type Cw721Contract<'a> =
    cw721_base::Cw721Contract<'a, MetadataExt, Empty, ExecuteExt, QueryExt>;

// The execute message type for this contract.
// If you don't need the Metadata and Execute extensions, you can use the
// `Empty` type.
pub type ExecuteMsg = cw721_base::ExecuteMsg<MetadataExt, ExecuteExt>;

// The query message type for this contract.
// If you don't need the QueryExt extension, you can use the
// `Empty` type.
pub type QueryMsg = cw721_base::QueryMsg<QueryExt>;

/// Custom errors for this contract, add additional errors here.
#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    /// This inherits from cw721-base::ContractError to handle the base contract errors
    #[error("{0}")]
    Cw721Error(#[from] cw721_base::ContractError),
}

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;

    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    // This makes a conscious choice on the various generics used by the contract
    #[entry_point]
    pub fn instantiate(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        // Instantiate the base contract
        Cw721Contract::default().instantiate(deps.branch(), env, info, msg)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            // Optionally override the default cw721-base behavior
            // ExecuteMsg::Burn { token_id } => unimplemented!(),

            // Implment extension messages here, remove if you don't wish to use
            // An ExecuteExt extension
            ExecuteMsg::Extension { msg } => match msg {
                _ => unimplemented!(),
            },

            // Use the default cw721-base implementation
            _ => Cw721Contract::default()
                .execute(deps, env, info, msg)
                .map_err(Into::into),
        }
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            // Optionally override a default cw721-base query
            // QueryMsg::Minter {} => unimplemented!(),
            QueryMsg::Extension { msg } => match msg {
                _ => unimplemented!(),
            },

            // Use default cw721-base query implementation
            _ => Cw721Contract::default().query(deps, env, msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    /// Make sure cw2 version info is properly initialized during instantiation,
    /// and NOT overwritten by the base contract.
    #[test]
    fn proper_cw2_initialization() {
        let mut deps = mock_dependencies();

        entry::instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("larry", &[]),
            InstantiateMsg {
                name: "".into(),
                symbol: "".into(),
                minter: "larry".into(),
            },
        )
        .unwrap();

        let version = cw2::get_contract_version(deps.as_ref().storage).unwrap();
        assert_eq!(version.contract, CONTRACT_NAME);
        assert_ne!(version.contract, cw721_base::CONTRACT_NAME);
    }
}
