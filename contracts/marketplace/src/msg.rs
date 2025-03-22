use cosmwasm_std::{Addr, Uint128, Uint64};
use cw721::Cw721ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Define the contract's messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    ListNftForSale(Cw721ReceiveMsg),
    CancelListing {
        collection: String,
        token_id: String,
    },
    BuyNft {
        collection: String,
        token_id: String,
    },
    BuyBatch {
        asks: Vec<(String, String)>,
    },
    ListNftForAuction(Cw721ReceiveMsg),
    StartAuction {
        collection: String,
        token_id: String,
    },
    CancelAuction {
        collection: String,
        token_id: String,
    },
    BidNft {
        collection: String,
        token_id: String,
        price: Uint128,
    },
    ClaimNft {
        collection: String,
        token_id: String,
    },
    AddAdmin {
        account_id: Addr,
    },
    RemoveAdmin {
        account_id: Addr,
    },
}

// Define the contract's query messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    GetListingCount {},
    GetListingByCollectionTokenID {
        collection: String,
        token_id: String,
    },
    GetAuctionCount {},
    GetAuctionByCollectionTokenID {
        collection: String,
        token_id: String,
    },
    IsAdmin {
        account_id: Addr,
    },
}

// Define the InstantiateMsg
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: String,                     // The owner of the contract
    pub collection_fabric_address: String, // The address of the collection fabric contract
    pub native_denom: String,
    pub royalty: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ListingHookMsg {
    SetListing {
        owner: String,
        token_id: String,
        price: Uint128,
        royalty: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AuctionListingHookMsg {
    SetAuctionListing {
        owner: String,
        token_id: String,
        start_price: Uint128,
        min_bid_step: Uint128,
        start_time: Uint64,
        end_time: Uint64,
        royalty: Uint128,
    },
}
