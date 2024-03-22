use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, export_schema_with_title, remove_schemas, schema_for};

use cw721::{
    msg::{
        AllNftInfoResponse, ApprovalResponse, ApprovalsResponse, Cw721ExecuteMsg, MinterResponse,
        NftInfoResponse, NumTokensResponse, OperatorsResponse, OwnerOfResponse, TokensResponse,
    },
    state::CollectionMetadata,
    CollectionMetadataWrapper, DefaultOptionCollectionMetadataExtension,
    DefaultOptionCollectionMetadataExtensionMsg, DefaultOptionNftMetadataExtension,
    DefaultOptionNftMetadataExtensionMsg,
};
#[allow(deprecated)]
use cw721_non_transferable::{InstantiateMsg, QueryMsg};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema_with_title(
        &schema_for!(InstantiateMsg<DefaultOptionCollectionMetadataExtension>),
        &out_dir,
        "InstantiateMsg",
    );
    export_schema_with_title(
        &schema_for!(Cw721ExecuteMsg<DefaultOptionNftMetadataExtensionMsg, DefaultOptionCollectionMetadataExtensionMsg>),
        &out_dir,
        "ExecuteMsg",
    );
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema_with_title(
        &schema_for!(NftInfoResponse<DefaultOptionNftMetadataExtension>),
        &out_dir,
        "NftInfoResponse",
    );
    export_schema_with_title(
        &schema_for!(AllNftInfoResponse<DefaultOptionNftMetadataExtension>),
        &out_dir,
        "AllNftInfoResponse",
    );
    export_schema(&schema_for!(ApprovalResponse), &out_dir);
    export_schema(&schema_for!(ApprovalsResponse), &out_dir);
    export_schema(&schema_for!(OperatorsResponse), &out_dir);
    export_schema_with_title(
        &schema_for!(CollectionMetadataWrapper<DefaultOptionCollectionMetadataExtension>),
        &out_dir,
        "CollectionMetadata",
    );
    export_schema(&schema_for!(MinterResponse), &out_dir);
    export_schema(&schema_for!(NumTokensResponse), &out_dir);
    export_schema(&schema_for!(OwnerOfResponse), &out_dir);
    export_schema(&schema_for!(TokensResponse), &out_dir);
}
