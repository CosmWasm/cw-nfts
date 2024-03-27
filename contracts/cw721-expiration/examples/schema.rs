use cosmwasm_schema::write_api;
use cosmwasm_std::Empty;
use cw721_base::{DefaultOptionalCollectionExtension, DefaultOptionalNftExtension};
use cw721_expiration::msg::{InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg<DefaultOptionalCollectionExtension>,
        query: QueryMsg<DefaultOptionalNftExtension, DefaultOptionalCollectionExtension, Empty>,
    }
}
