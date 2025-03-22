use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr, Event,
};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Define the UserData struct
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserData {
    pub field1: String,
    pub field2: u64,
    // Add more fields as needed
}

// Define the contract's state
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
    pub user_data: UserData,
}

// Storage keys
pub const STATE: Item<State> = Item::new("state");

// Instantiate message
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: String,
    pub user_data: UserData,
}

// Execute messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetUserData { user_data: UserData },
}

// Query messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetUserData {},
}

// Instantiate entry point
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = State {
        owner: deps.api.addr_validate(&msg.owner)?,
        user_data: msg.user_data,
    };
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", msg.owner))
}

// Execute entry point
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::SetUserData { user_data } => set_user_data(deps, info, user_data),
    }
}

// Set user data function
fn set_user_data(
    deps: DepsMut,
    info: MessageInfo,
    user_data: UserData,
) -> StdResult<Response> {
    let mut state = STATE.load(deps.storage)?;

    // Check if the caller is the owner
    if info.sender != state.owner {
        return Err(cosmwasm_std::StdError::generic_err("Unauthorized"));
    }

    // Update user data
    state.user_data = user_data;
    STATE.save(deps.storage, &state)?;

    // Emit an event
    let event = Event::new("user_data_set")
        .add_attribute("field1", state.user_data.field1.clone())
        .add_attribute("field2", state.user_data.field2.to_string());

    Ok(Response::new()
        .add_event(event)
        .add_attribute("method", "set_user_data"))
}

// Query entry point
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetUserData {} => to_json_binary(&query_user_data(deps)?),
    }
}

// Query user data function
fn query_user_data(deps: Deps) -> StdResult<UserData> {
    let state = STATE.load(deps.storage)?;
    Ok(state.user_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::Addr;

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("owner", &[]);

        let msg = InstantiateMsg {
            owner: "owner".to_string(),
            user_data: UserData {
                field1: "test".to_string(),
                field2: 123,
            },
        };

        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(res.attributes[0].value, "instantiate");

        let state = STATE.load(&deps.storage).unwrap();
        assert_eq!(state.owner, Addr::unchecked("owner"));
        assert_eq!(state.user_data.field1, "test");
        assert_eq!(state.user_data.field2, 123);
    }

    #[test]
    fn test_set_user_data() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("owner", &[]);

        let msg = InstantiateMsg {
            owner: "owner".to_string(),
            user_data: UserData {
                field1: "test".to_string(),
                field2: 123,
            },
        };

        instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let new_user_data = UserData {
            field1: "new_test".to_string(),
            field2: 456,
        };

        let msg = ExecuteMsg::SetUserData {
            user_data: new_user_data.clone(),
        };

        let res = execute(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(res.attributes[0].value, "set_user_data");

        let state = STATE.load(&deps.storage).unwrap();
        assert_eq!(state.user_data, new_user_data);
    }
}