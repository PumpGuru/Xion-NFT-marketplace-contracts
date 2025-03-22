use cosmwasm_std::{Addr, Uint128, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_storage_plus::{Item, Map};

// Define the contract's state
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub native_denom: String,
    pub royalty: u128
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
    pub collection_fabric_address: Addr,
    pub listing_count: u128,
    pub auction_count: u128,
}

// Define the Listing struct
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Listing {
    pub seller: String,
    pub collection: String,
    pub token_id: String,
    pub price: Uint128,
    /// Royalty of the listing, set automatically by deriving the value from the collection contract.
    pub royalty: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum AuctionStatus {
    /// The auction is waiting for auction.
    WaitingAuction,
    /// The auction is in auction.
    InAuction,
    /// The auction is waiting for claim.
    WaitingForClaim,
    /// The auction is ended.
    Ended,
    /// The auction is cancelled.
    Cancelled,
}

// Define the Auction struct
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Auction {
    pub seller: String,
    pub collection: String,
    pub token_id: String,
    pub start_price: Uint128,
    pub min_bid_step: Uint128,
    pub start_time: Uint64,
    pub end_time: Uint64,
    pub current_price: Uint128,
    pub current_bidder: Option<Addr>,
    pub status: AuctionStatus,
    pub royalty: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Deposits {
    pub owner: String,
    pub collection: String,
    pub token_id: String,
}

// Define the storage keys
pub const CONFIG: Item<Config> = Item::new("config");
pub const STATE: Item<State> = Item::new("state");
pub const LISTINGS: Map<(&str, &str), Listing> = Map::new("listings");
pub const AUCTIONS: Map<(&str, &str), Auction> = Map::new("auctions");
pub const ADMINS: Item<Vec<Addr>> = Item::new("admins");
//contract, owner, token_id
pub const DEPOSITS: Map<(&str, &str, &str), Deposits> = Map::new("deposits");
