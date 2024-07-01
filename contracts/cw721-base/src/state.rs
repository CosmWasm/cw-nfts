use std::marker::PhantomData;

use cosmwasm_std::Empty;

// expose so other libs dont need to import cw721
pub use cw721::state::*;

use cw721::{
    traits::{Cw721CustomMsg, Cw721State},
    DefaultOptionalCollectionExtension, DefaultOptionalCollectionExtensionMsg,
    DefaultOptionalNftExtension, DefaultOptionalNftExtensionMsg,
};

#[deprecated(since = "0.19.0", note = "Please use `NftInfo`")]
pub type TokenInfo<TNftExtension> = NftInfo<TNftExtension>;

/// Opionated version of `Cw721Contract` with default extensions using:
/// - `DefaultOptionalNftExtension` for NftInfo extension (onchain metadata).
/// - `DefaultOptionalNftExtensionMsg` for NftInfo extension msg for onchain metadata.
/// - `DefaultOptionalCollectionExtension` for CollectionInfo extension (onchain attributes).
/// - `DefaultOptionalCollectionExtensionMsg` for CollectionInfo extension msg for onchain collection attributes.
/// - `Empty` for custom extension msg for custom contract logic.
/// - `Empty` for custom query msg for custom contract logic.
/// - `Empty` for custom response msg for custom contract logic.
pub struct DefaultCw721Contract<'a> {
    pub config: Cw721Config<'a, DefaultOptionalNftExtension>,
    pub(crate) _collection_extension: PhantomData<DefaultOptionalCollectionExtension>,
    pub(crate) _nft_extension_msg: PhantomData<DefaultOptionalNftExtensionMsg>,
    pub(crate) _collection_extension_msg: PhantomData<DefaultOptionalCollectionExtensionMsg>,
    pub(crate) _extension_msg: PhantomData<Empty>,
    pub(crate) _extension_query_msg: PhantomData<Empty>,
    pub(crate) _custom_response_msg: PhantomData<Empty>,
}

impl Default for DefaultCw721Contract<'static> {
    fn default() -> Self {
        Self {
            config: Cw721Config::<DefaultOptionalNftExtension>::default(),
            _collection_extension: PhantomData,
            _nft_extension_msg: PhantomData,
            _collection_extension_msg: PhantomData,
            _extension_msg: PhantomData,
            _extension_query_msg: PhantomData,
            _custom_response_msg: PhantomData,
        }
    }
}

/// Opionated version of `Cw721Contract` with empty extensions using:
/// - `Empty` for NftInfo extension (onchain metadata).
/// - `Empty` for NftInfo extension msg for onchain metadata.
/// - `Empty` for CollectionInfo extension (onchain attributes).
/// - `Empty` for CollectionInfo extension msg for onchain collection attributes.
/// - `Empty` for custom extension msg for custom contract logic.
/// - `Empty` for custom query msg for custom contract logic.
/// - `Empty` for custom response msg for custom contract logic.
pub struct EmptyCw721Contract<'a> {
    pub config: Cw721Config<'a, Empty>,
    pub(crate) _collection_extension: PhantomData<Empty>,
    pub(crate) _nft_extension_msg: PhantomData<Empty>,
    pub(crate) _collection_extension_msg: PhantomData<Empty>,
    pub(crate) _extension_msg: PhantomData<Empty>,
    pub(crate) _extension_query_msg: PhantomData<Empty>,
    pub(crate) _custom_response_msg: PhantomData<Empty>,
}

impl Default for EmptyCw721Contract<'static> {
    fn default() -> Self {
        Self {
            config: Cw721Config::<Empty>::default(),
            _collection_extension: PhantomData,
            _nft_extension_msg: PhantomData,
            _collection_extension_msg: PhantomData,
            _extension_msg: PhantomData,
            _extension_query_msg: PhantomData,
            _custom_response_msg: PhantomData,
        }
    }
}

/// `cw721-base` with `TNftExtension` and `TCollectionExtension` allowing contract handling with:
/// - no extensions: `TNftExtension: Empty` and `TCollectionExtension: Empty`
/// - opionated `DefaultOptionalNftExtension` and `DefaultOptionalCollectionExtension`.
///   - `DefaultOptionalNftExtension`: either with nft metadata (`Some<NftExtension>`) or none `None`.
///   - `DefaultOptionalCollectionExtension`: either with collection metadata (`Some<CollectionExtension>`) or none `None`.
///
/// Example:
/// ```rust
/// // instantiate:
/// let contract = Cw721Contract::<
///     DefaultOptionalNftExtension, // use `Option<Empty>` or `Empty` for no nft metadata
///     DefaultOptionalNftExtensionMsg, // use `Option<Empty>` or `Empty` for no nft metadata
///     DefaultOptionalCollectionExtension, // use `Option<Empty>` or `Empty` for no collection metadata
///     DefaultOptionalCollectionExtensionMsg, // use `Option<Empty>` or `Empty` for no collection metadata
///     Empty, // no custom extension msg
///     Empty, // no custom query msg
///     Empty, // no custom response msg
/// >::default();
/// let info = mock_info(CREATOR, &[]);
/// let init_msg = Cw721InstantiateMsg {
///     name: "SpaceShips".to_string(),
///     symbol: "SPACE".to_string(),
///     collection_info_extension: None,
///     minter: None,
///     creator: None,
///     withdraw_address: None,
/// };
/// //...
/// // mint:
/// let token_id = "Enterprise";
/// let token_uri = Some("https://starships.example.com/Starship/Enterprise.json".into());
/// let extension = Some(NftExtensionMsg {
///     description: Some("description1".into()),
///     name: Some("name1".to_string()),
///     attributes: Some(vec![Trait {
///         display_type: None,
///         trait_type: "type1".to_string(),
///         value: "value1".to_string(),
///     }]),
///     ..NftExtensionMsg::default()
/// });
/// let exec_msg = Cw721ExecuteMsg::<
///     DefaultOptionalNftExtensionMsg,
///     DefaultOptionalCollectionExtensionMsg,
///     Empty,
/// >::Mint {
///     token_id: token_id.to_string(),
///     owner: "john".to_string(),
///     token_uri: token_uri.clone(),
///     extension: extension.clone(), // use `extension: None` for no metadata
/// };
/// //...
/// ```
pub struct Cw721Contract<
    'a,
    // NftInfo extension (onchain metadata).
    TNftExtension,
    // NftInfo extension msg for onchain metadata.
    TNftExtensionMsg,
    // CollectionInfo extension (onchain attributes).
    TCollectionExtension,
    // CollectionInfo extension msg for onchain collection attributes.
    TCollectionExtensionMsg,
    // Custom extension msg for custom contract logic. Default implementation is a no-op.
    TExtensionMsg,
    // Custom query msg for custom contract logic. Default implementation returns an empty binary.
    TExtensionQueryMsg,
    // Defines for `CosmosMsg::Custom<T>` in response. Barely used, so `Empty` can be used.
    TCustomResponseMsg,
> where
    TNftExtension: Cw721State,
    TNftExtensionMsg: Cw721CustomMsg,
    TCollectionExtension: Cw721State,
    TCollectionExtensionMsg: Cw721CustomMsg,
{
    pub config: Cw721Config<'a, TNftExtension>,
    pub(crate) _collection_extension: PhantomData<TCollectionExtension>,
    pub(crate) _nft_extension_msg: PhantomData<TNftExtensionMsg>,
    pub(crate) _collection_extension_msg: PhantomData<TCollectionExtensionMsg>,
    pub(crate) _extension_msg: PhantomData<TExtensionMsg>,
    pub(crate) _extension_query_msg: PhantomData<TExtensionQueryMsg>,
    pub(crate) _custom_response_msg: PhantomData<TCustomResponseMsg>,
}

impl<
        TNftExtension,
        TNftExtensionMsg,
        TCollectionExtension,
        TCollectionExtensionMsg,
        TExtensionMsg,
        TExtensionQueryMsg,
        TCustomResponseMsg,
    > Default
    for Cw721Contract<
        'static,
        TNftExtension,
        TNftExtensionMsg,
        TCollectionExtension,
        TCollectionExtensionMsg,
        TExtensionMsg,
        TExtensionQueryMsg,
        TCustomResponseMsg,
    >
where
    TNftExtension: Cw721State,
    TNftExtensionMsg: Cw721CustomMsg,
    TCollectionExtension: Cw721State,
    TCollectionExtensionMsg: Cw721CustomMsg,
{
    fn default() -> Self {
        Self {
            config: Cw721Config::default(),
            _collection_extension: PhantomData,
            _nft_extension_msg: PhantomData,
            _collection_extension_msg: PhantomData,
            _extension_msg: PhantomData,
            _extension_query_msg: PhantomData,
            _custom_response_msg: PhantomData,
        }
    }
}
