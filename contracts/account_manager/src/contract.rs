use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr,
};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::StdError;
use thiserror::Error;

// Define storage keys
const USER_CODE_HASH: Item<String> = Item::new("user_code_hash");
const CREATOR_CODE_HASH: Item<String> = Item::new("creator_code_hash");
const ACCOUNTS: Map<&Addr, Addr> = Map::new("accounts");
const CREATOR_ACCOUNTS: Map<&Addr, Addr> = Map::new("creator_accounts");
const ADMINS: Map<&Addr, ()> = Map::new("admins");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub user_code_hash: String,
    pub creator_code_hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateAccount {},
    CreateCreatorAccount {},
    SetUserCodeHash { code_hash: String },
    SetCreatorCodeHash { code_hash: String },
    AddAdmin { admin: Addr },
    RemoveAdmin { admin: Addr },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetAccount { account_id: Addr },
    GetCreatorAccount { account_id: Addr },
    GetUserCodeHash {},
    GetCreatorCodeHash {},
    IsAdmin { admin: Addr },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AccountResponse {
    pub account_id: Addr,
    pub contract_id: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CodeHashResponse {
    pub code_hash: String,
}

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Account already exists")]
    AccountExists,
    #[error("Creator account already exists")]
    CreatorAccountExists,
    #[error("Admin already exists")]
    AdminExists,
    #[error("Admin not found")]
    AdminNotFound,
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    USER_CODE_HASH.save(deps.storage, &msg.user_code_hash)?;
    CREATOR_CODE_HASH.save(deps.storage, &msg.creator_code_hash)?;

    // Set the caller as the initial admin
    ADMINS.save(deps.storage, &_info.sender, &())?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateAccount {} => create_account(deps, info),
        ExecuteMsg::CreateCreatorAccount {} => create_creator_account(deps, info),
        ExecuteMsg::SetUserCodeHash { code_hash } => set_user_code_hash(deps, info, code_hash),
        ExecuteMsg::SetCreatorCodeHash { code_hash } => set_creator_code_hash(deps, info, code_hash),
        ExecuteMsg::AddAdmin { admin } => add_admin(deps, info, admin),
        ExecuteMsg::RemoveAdmin { admin } => remove_admin(deps, info, admin),
    }
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetAccount { account_id } => {
            let contract_id = ACCOUNTS.load(deps.storage, &account_id)?;
            let response = AccountResponse {
                account_id,
                contract_id,
            };
            to_json_binary(&response) // Serialize the response into Binary
        }
        QueryMsg::GetCreatorAccount { account_id } => {
            let contract_id = CREATOR_ACCOUNTS.load(deps.storage, &account_id)?;
            let response = AccountResponse {
                account_id,
                contract_id,
            };
            to_json_binary(&response) // Serialize the response into Binary
        }
        QueryMsg::GetUserCodeHash {} => {
            let code_hash = USER_CODE_HASH.load(deps.storage)?;
            let response = CodeHashResponse { code_hash };
            to_json_binary(&response) // Serialize the response into Binary
        }
        QueryMsg::GetCreatorCodeHash {} => {
            let code_hash = CREATOR_CODE_HASH.load(deps.storage)?;
            let response = CodeHashResponse { code_hash };
            to_json_binary(&response) // Serialize the response into Binary
        }
        QueryMsg::IsAdmin { admin } => {
            let is_admin = ADMINS.has(deps.storage, &admin);
            to_json_binary(&is_admin) // Serialize the boolean into Binary
        }
    }
}

fn create_account(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    if ACCOUNTS.has(deps.storage, &info.sender) {
        return Err(ContractError::AccountExists);
    }

    // Simulate contract deployment (in CosmWasm, you would instantiate a new contract here)
    let contract_id = info.sender.clone();

    ACCOUNTS.save(deps.storage, &info.sender, &contract_id)?;

    Ok(Response::new()
        .add_attribute("method", "create_account")
        .add_attribute("account_id", info.sender)
        .add_attribute("contract_id", contract_id))
}

fn create_creator_account(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    if CREATOR_ACCOUNTS.has(deps.storage, &info.sender) {
        return Err(ContractError::CreatorAccountExists);
    }

    // Simulate contract deployment
    let contract_id = info.sender.clone();

    CREATOR_ACCOUNTS.save(deps.storage, &info.sender, &contract_id)?;

    Ok(Response::new()
        .add_attribute("method", "create_creator_account")
        .add_attribute("account_id", info.sender)
        .add_attribute("contract_id", contract_id))
}

fn set_user_code_hash(
    deps: DepsMut,
    info: MessageInfo,
    code_hash: String,
) -> Result<Response, ContractError> {
    if !ADMINS.has(deps.storage, &info.sender) {
        return Err(ContractError::Unauthorized);
    }

    USER_CODE_HASH.save(deps.storage, &code_hash)?;

    Ok(Response::new()
        .add_attribute("method", "set_user_code_hash")
        .add_attribute("code_hash", code_hash))
}

fn set_creator_code_hash(
    deps: DepsMut,
    info: MessageInfo,
    code_hash: String,
) -> Result<Response, ContractError> {
    if !ADMINS.has(deps.storage, &info.sender) {
        return Err(ContractError::Unauthorized);
    }

    CREATOR_CODE_HASH.save(deps.storage, &code_hash)?;

    Ok(Response::new()
        .add_attribute("method", "set_creator_code_hash")
        .add_attribute("code_hash", code_hash))
}

fn add_admin(deps: DepsMut, info: MessageInfo, admin: Addr) -> Result<Response, ContractError> {
    if !ADMINS.has(deps.storage, &info.sender) {
        return Err(ContractError::Unauthorized);
    }

    if ADMINS.has(deps.storage, &admin) {
        return Err(ContractError::AdminExists);
    }

    ADMINS.save(deps.storage, &admin, &())?;

    Ok(Response::new()
        .add_attribute("method", "add_admin")
        .add_attribute("admin", admin))
}

fn remove_admin(deps: DepsMut, info: MessageInfo, admin: Addr) -> Result<Response, ContractError> {
    if !ADMINS.has(deps.storage, &info.sender) {
        return Err(ContractError::Unauthorized);
    }

    if !ADMINS.has(deps.storage, &admin) {
        return Err(ContractError::AdminNotFound);
    }

    ADMINS.remove(deps.storage, &admin);

    Ok(Response::new()
        .add_attribute("method", "remove_admin")
        .add_attribute("admin", admin))
}
