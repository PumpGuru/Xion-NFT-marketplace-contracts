use cw721_base::MintMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::Metadata;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg<T> {
    /// Mint a new NFT, can only be called by the contract minter
    Mint(MintMsg<T>),
    /// Updates metadata of the NFT
    UpdateMetadata { token_id: String, token_uri: String, metadata: Metadata },
    
}
