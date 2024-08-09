use cosmwasm_schema::write_api;
use cosmwasm_std::Empty;
use cw1155::msg::{Cw1155ExecuteMsg, Cw1155InstantiateMsg, Cw1155QueryMsg};
use cw721::state::DefaultOptionMetadataExtension;

fn main() {
    write_api! {
        instantiate: Cw1155InstantiateMsg,
        execute: Cw1155ExecuteMsg<DefaultOptionMetadataExtension, Empty>,
        query: Cw1155QueryMsg<Empty, Empty>,
    }
}