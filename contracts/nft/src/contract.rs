use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Empty;
use cw2::set_contract_version;

use cw721_base::Cw721Contract;
use cw721_base::InstantiateMsg;

// Version info for migration
const CONTRACT_NAME: &str = "crates.io:cw721-metadata-onchain";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

// see: https://docs.opensea.io/docs/metadata-standards
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Metadata {
    pub name: Option<String>,
    pub image: Option<String>,
    pub cohort: Option<String>,
    pub description: Option<String>,
    pub attributes: Option<Vec<Trait>>,
    pub badges: Option<Vec<String>>,
    pub skills: Option<Vec<String>>,
    pub github_url: Option<String>,
    pub is_for_hire: bool,
}

pub type Extension = Option<Metadata>;

pub type Cw721MetadaNonTransferableContract<'a> = Cw721Contract<'a, Extension, Empty>;
pub type Cw721ExecuteMsg = cw721_base::ExecuteMsg<Extension>;
pub type QueryMsg = cw721_base::QueryMsg;

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;

    use crate::msg::ExecuteMsg;
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
    use cw721_base::ContractError;

    // This makes a conscious choice on the various generics used by the contract
    #[entry_point]
    pub fn instantiate(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        let res = Cw721MetadaNonTransferableContract::default().instantiate(
            deps.branch(),
            env,
            info,
            msg,
        )?;
        // Explicitly set contract name and version, otherwise set to cw721-base info
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)
            .map_err(ContractError::Std)?;
        Ok(res)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<Extension>,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::Mint(mint_msg) => {
                Cw721MetadaNonTransferableContract::default().mint(deps, env, info, mint_msg)
            }
            ExecuteMsg::UpdateMetadata {
                token_id,
                token_uri,
                metadata,
            } => execute_update_on_chain_metadata(deps, env, info, token_id, token_uri, metadata),
        }
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        Cw721MetadaNonTransferableContract::default().query(deps, env, msg)
    }

    fn execute_update_on_chain_metadata(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        token_id: String,
        token_uri: String,
        metadata: Metadata
    ) -> Result<Response, ContractError> {
        let tract = Cw721MetadaNonTransferableContract::default();
        let minter = tract.minter.load(deps.storage)?;
        if info.sender != minter {
            Err(ContractError::Unauthorized {})
        } else {
            tract
                .tokens
                .update(deps.storage, &token_id, |token| match token {
                    Some(mut token_info) => {
                        token_info.extension = Some(metadata);
                        token_info.token_uri = Some(token_uri.clone());
                        Ok(token_info)
                    },
                    None => Err(ContractError::Unauthorized {}),
                })?;
            Ok(Response::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cw721::Cw721Query;
    use cw721_base::MintMsg;

    const CREATOR: &str = "creator";

    #[test]
    fn use_metadata_extension() {
        let mut deps = mock_dependencies();
        let contract = Cw721MetadaNonTransferableContract::default();

        let info = mock_info(CREATOR, &[]);
        let init_msg = InstantiateMsg {
            name: "SpaceShips".to_string(),
            symbol: "SPACE".to_string(),
            minter: CREATOR.to_string(),
        };
        contract
            .instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg)
            .unwrap();

        let token_id = "Enterprise";
        let mint_msg = MintMsg {
            token_id: token_id.to_string(),
            owner: "john".to_string(),
            token_uri: Some("https://starships.example.com/Starship/Enterprise.json".into()),
            extension: Some(Metadata {
                description: Some("Spaceship with Warp Drive".into()),
                name: Some("Starship USS Enterprise".to_string()),
                ..Metadata::default()
            }),
        };
        let exec_msg = Cw721ExecuteMsg::Mint(mint_msg.clone());
        contract
            .execute(deps.as_mut(), mock_env(), info, exec_msg)
            .unwrap();

        let res = contract.nft_info(deps.as_ref(), token_id.into()).unwrap();
        assert_eq!(res.token_uri, mint_msg.token_uri);
        assert_eq!(res.extension, mint_msg.extension);
    }
}
