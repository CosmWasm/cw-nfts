use cosmwasm_schema::write_api;
use cosmwasm_std::Empty;
use cw721_base::{DefaultOptionCollectionMetadataExtension, DefaultOptionNftMetadataExtension};
use cw721_expiration::msg::{InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg<DefaultOptionCollectionMetadataExtension>,
        query: QueryMsg<DefaultOptionNftMetadataExtension, DefaultOptionCollectionMetadataExtension>,
    }
}
