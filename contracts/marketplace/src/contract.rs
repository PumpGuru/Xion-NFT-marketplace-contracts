#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, from_json, to_json_binary, Addr, BankMsg, Binary, Coin, Deps, DepsMut, Env,
    MessageInfo, Response, StdResult, Uint128, Uint64, WasmMsg,
};

use crate::msg::{AuctionListingHookMsg, ExecuteMsg, InstantiateMsg, ListingHookMsg, QueryMsg};
use cosmwasm_std::{ensure, CosmosMsg, StdError};
use cw2::set_contract_version;
use cw721::{Cw721ExecuteMsg, Cw721ReceiveMsg};
use cw_utils::must_pay;

use crate::state::{
    Auction, AuctionStatus, Config, Deposits, Listing, State, ADMINS, AUCTIONS, CONFIG, DEPOSITS,
    LISTINGS, STATE,
};

const CONTRACT_NAME: &str = "crates.io:marketplace";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Define the contract's entry points
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let config = Config {
        native_denom: msg.native_denom,
        royalty: msg.royalty,
    };

    let state = State {
        owner: deps.api.addr_validate(&msg.owner)?,
        collection_fabric_address: deps.api.addr_validate(&msg.collection_fabric_address)?,
        listing_count: 0,
        auction_count: 0,
    };
    STATE.save(deps.storage, &state)?;
    CONFIG.save(deps.storage, &config)?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::ListNftForSale(cw721_msg) => list_nft_for_sale(deps, env, info, cw721_msg),
        ExecuteMsg::CancelListing {
            collection,
            token_id,
        } => cancel_listing(deps, env, info, collection, token_id),
        ExecuteMsg::BuyNft {
            collection,
            token_id,
        } => buy_nft(deps, env, info, collection, token_id),
        ExecuteMsg::BuyBatch { asks } => buy_batch(deps, env, info, asks),
        ExecuteMsg::ListNftForAuction(cw721_msg) => {
            list_nft_for_auction(deps, env, info, cw721_msg)
        }
        ExecuteMsg::StartAuction {
            collection,
            token_id,
        } => start_auction(deps, env, info, collection, token_id),
        ExecuteMsg::CancelAuction {
            collection,
            token_id,
        } => cancel_auction(deps, env, info, collection, token_id),
        ExecuteMsg::BidNft {
            collection,
            token_id,
            price,
        } => bid_nft(deps, env, info, collection, token_id, price),
        ExecuteMsg::ClaimNft {
            collection,
            token_id,
        } => claim_nft(deps, env, info, collection, token_id),
        ExecuteMsg::AddAdmin { account_id } => add_admin(deps, env, info, account_id),
        ExecuteMsg::RemoveAdmin { account_id } => remove_admin(deps, env, info, account_id),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetListingCount {} => to_json_binary(&query_listing_count(deps)?),
        QueryMsg::GetListingByCollectionTokenID {
            collection,
            token_id,
        } => to_json_binary(&query_listing_by_index(deps, collection, token_id)?),
        QueryMsg::GetAuctionCount {} => to_json_binary(&query_auction_count(deps)?),
        QueryMsg::GetAuctionByCollectionTokenID {
            collection,
            token_id,
        } => to_json_binary(&query_auction_by_index(deps, collection, token_id)?),
        QueryMsg::IsAdmin { account_id } => to_json_binary(&query_is_admin(deps, account_id)?),
    }
}

// Implement the contract's logic
pub fn list_nft_for_sale(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw721_msg: Cw721ReceiveMsg,
) -> StdResult<Response> {
    match from_json(&cw721_msg.msg) {
        Ok(ListingHookMsg::SetListing {
            owner,
            collection,
            token_id,
            price,
            royalty,
        }) => {
            execute_list_nft_for_sale(deps, env, info, owner, collection, token_id, price, royalty)
        }
        _ => Err(StdError::generic_err("Invalid ListingHookMsg")),
    }
}

fn execute_list_nft_for_sale(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    owner: String,
    collection: String,
    token_id: String,
    price: Uint128,
    royalty: Uint128,
) -> StdResult<Response> {
    // Check if the NFT is already listed
    if DEPOSITS.has(deps.storage, (&collection, &owner, &token_id)) == true {
        return Err(StdError::generic_err("NFT is already listed"));
    }

    let deposit = Deposits {
        owner: owner.clone(),
        collection: collection.clone(),
        token_id: token_id.clone(),
    };
    DEPOSITS.save(deps.storage, (&collection, &owner, &token_id), &deposit)?;

    let listing = Listing {
        seller: owner.clone(),
        collection: collection.clone(),
        token_id: token_id.clone(),
        price: price.clone(),
        royalty: royalty.clone(),
    };
    let mut state = STATE.load(deps.storage)?;
    state.listing_count += 1;
    LISTINGS.save(deps.storage, (&collection, &token_id), &listing)?;
    STATE.save(deps.storage, &state)?;

    // Transfer the NFT from the seller to the marketplace contract
    let transfer_to_marketplace_msg = Cw721ExecuteMsg::TransferNft {
        recipient: env.contract.address.to_string(), // Marketplace contract address
        token_id: token_id.clone(),
    };

    let execute_transfer_to_marketplace = WasmMsg::Execute {
        contract_addr: collection.to_string(),
        msg: to_json_binary(&transfer_to_marketplace_msg)?,
        funds: vec![],
    };

    // Return the response with the approval message
    Ok(Response::new()
        .add_message(execute_transfer_to_marketplace)
        .add_attribute("method", "list_nft_for_sale")
        .add_attribute("listing_id", state.listing_count.to_string()))
}

pub fn cancel_listing(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    collection: String,
    token_id: String,
) -> StdResult<Response> {
    let owner = info.sender.clone().into_string();
    // Load the listing from storage
    if DEPOSITS.has(deps.storage, (&collection, &owner, &token_id)) == false {
        return Err(StdError::generic_err(
            "Only the owner can cancel the listing",
        ));
    }

    DEPOSITS.remove(deps.storage, (&collection, &owner, &token_id));
    LISTINGS.remove(deps.storage, (&collection, &token_id));
    // Transfer the NFT from the seller to the marketplace contract
    let transfer_from_marketplace_msg = nft::contract::Cw721ExecuteMsg::TransferNft {
        recipient: owner, // Marketplace contract address
        token_id,
    };

    let execute_transfer_from_marketplace = WasmMsg::Execute {
        contract_addr: collection,
        msg: to_json_binary(&transfer_from_marketplace_msg)?,
        funds: vec![],
    };

    // Update the state (optional: decrement listing count if needed)
    let mut state = STATE.load(deps.storage)?;
    state.listing_count = state.listing_count.saturating_sub(1); // Decrement listing count
    STATE.save(deps.storage, &state)?;

    // Return a response with the revoke approval message and attributes
    Ok(Response::new()
        .add_message(execute_transfer_from_marketplace) // Add the revoke approval message to the response
        .add_attribute("method", "cancel_listing"))
}

fn buy_nft(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    collection: String,
    token_id: String,
) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;
    let buyer = info.sender.to_string();
    let funds_sent = must_pay(&info, &config.native_denom).map_err(|e| StdError::generic_err(e.to_string()))?;
    // Load the listing
    let listing = LISTINGS.may_load(deps.storage, (&collection, &token_id))?;
    let state = STATE.load(deps.storage)?;

    match listing {
        Some(listing) => {
            // Ensure the buyer is not the seller
            ensure!(
                info.sender != listing.seller,
                StdError::generic_err("You cannot buy your own NFT")
            );
            // Calculate the royalty amount
            let royalty_amount = listing.price.multiply_ratio(listing.royalty, 100u128); // royalty = (listing.price * config.royalty) / 100

            if funds_sent != listing.price {
                Err(StdError::generic_err("Invalid amount"))
            } else {
                // Transfer the NFT from the seller to the buyer
                let transfer_to_buyer_msg = nft::contract::Cw721ExecuteMsg::TransferNft {
                    recipient: buyer.clone(), // Buyer
                    token_id: listing.token_id,
                };

                let execute_transfer_to_buyer = WasmMsg::Execute {
                    contract_addr: collection.clone(), // NFT contract address
                    msg: to_json_binary(&transfer_to_buyer_msg)?,
                    funds: vec![],
                };

                // Transfer funds from the buyer to the seller
                let transfer_funds_to_seller = BankMsg::Send {
                    to_address: listing.seller.to_string(), // Seller
                    amount: vec![Coin {
                        denom: config.native_denom.to_string(), // Replace with your native token denom
                        amount: listing.price - royalty_amount, // Seller receives the listing price
                    }],
                };

                // Transfer royalty to the marketplace contract
                let transfer_royalty_to_marketplace = BankMsg::Send {
                    to_address: state.collection_fabric_address.to_string(), // Marketplace contract
                    amount: vec![Coin {
                        denom: config.native_denom.to_string(), // Replace with your native token denom
                        amount: royalty_amount,                 // Marketplace receives the royalty
                    }],
                };

                // Remove the listing
                DEPOSITS.remove(
                    deps.storage,
                    (&collection, &listing.seller.to_string(), &token_id),
                );
                LISTINGS.remove(deps.storage, (&collection, &token_id));

                // Return the response with the transfer messages
                Ok(Response::new()
                    .add_message(execute_transfer_to_buyer) // Transfer NFT
                    .add_message(transfer_funds_to_seller) // Transfer funds to seller
                    .add_message(transfer_royalty_to_marketplace) // Transfer royalty to marketplace
                    .add_attribute("method", "buy_nft")
                    .add_attribute("buyer", info.sender.into_string())
                    .add_attribute("seller", listing.seller.to_string()))
            }
        }
        None => Err(StdError::generic_err("TokenNotListedForSale")),
    }
}

fn buy_batch(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    asks: Vec<(String, String)>,
) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;
    let buyer = info.sender.to_string();
    let funds_sent = must_pay(&info, &config.native_denom).unwrap();

    let mut messages: Vec<WasmMsg> = vec![];
    let mut total_price = Uint128::zero();
    let mut sellers: Vec<(String, Uint128)> = vec![];

    for ask in &asks {
        // Load the listing from storage
        let listing = LISTINGS.may_load(deps.storage, (&ask.0, &ask.1))?;

        // Use `match` or `if let` to handle the `Option` returned by `may_load`
        match listing {
            Some(listing) => {
                // Ensure the buyer is not the seller
                ensure!(
                    info.sender != listing.seller,
                    StdError::generic_err("You cannot buy your own NFT")
                );

                // Add the listing price to the total price
                total_price += listing.price;
                sellers.push((listing.seller, listing.price));
            }
            None => {
                // Return an error if the token is not listed for sale
                return Err(StdError::generic_err("TokenNotListedForSale"));
            }
        }
    }

    if funds_sent != total_price {
        Err(StdError::generic_err("Invalid amount"))
    } else {
        for ask in &asks {
            let listing = LISTINGS.load(deps.storage, (&ask.0, &ask.1))?;

            // Transfer the NFT to the buyer
            let transfer_to_buyer_msg = WasmMsg::Execute {
                contract_addr: listing.collection.to_string(),
                msg: to_json_binary(&nft::contract::Cw721ExecuteMsg::TransferNft {
                    recipient: buyer.clone(),
                    token_id: listing.token_id.clone(),
                })?,
                funds: vec![],
            };
            messages.push(transfer_to_buyer_msg);

            // Remove the listing
            DEPOSITS.remove(deps.storage, (&ask.0, &listing.seller.to_string(), &ask.1));
            LISTINGS.remove(deps.storage, (&ask.0, &ask.1));
        }

        // Create a list of BankMsg transfers
        let fund_transfers: Vec<BankMsg> = sellers
            .into_iter()
            .map(|(seller, amount)| BankMsg::Send {
                to_address: seller,
                amount: vec![Coin {
                    denom: config.native_denom.to_string(),
                    amount,
                }],
            })
            .collect();

        Ok(Response::new()
            .add_messages(messages)
            .add_messages(fund_transfers)
            .add_attribute("method", "buy_batch")
            .add_attribute("buyer", info.sender.into_string()))
    }
}

pub fn list_nft_for_auction(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw721_msg: Cw721ReceiveMsg,
) -> StdResult<Response> {
    match from_json(&cw721_msg.msg) {
        Ok(AuctionListingHookMsg::SetAuctionListing {
            owner,
            collection,
            token_id,
            start_price,
            start_time,
            end_time,
            min_bid_step,
            royalty,
        }) => execute_list_nft_for_auction(
            deps,
            env,
            info,
            owner,
            collection,
            token_id,
            start_price,
            min_bid_step,
            start_time,
            end_time,
            royalty,
        ),
        _ => Err(StdError::generic_err("Invalid AuctionListingHookMsg")),
    }
}

pub fn execute_list_nft_for_auction(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    owner: String,
    collection: String,
    token_id: String,
    start_price: Uint128,
    min_bid_step: Uint128,
    start_time: Uint64,
    end_time: Uint64,
    royalty: Uint128,
) -> StdResult<Response> {
    // Check if the caller is the NFT owner
    let nft_owner: Addr = deps.querier.query_wasm_smart(
        collection.clone(),
        &nft::contract::QueryMsg::OwnerOf {
            token_id: token_id.clone(),
            include_expired: None,
        },
    )?;

    if nft_owner != owner {
        return Err(StdError::generic_err("CallerIsNotNFTOwner"));
    }

    // Validate auction parameters
    if start_price.is_zero() {
        return Err(StdError::generic_err("AuctionPriceIsZero"));
    }

    if min_bid_step.is_zero() {
        return Err(StdError::generic_err("AuctionMinBidStepIsZero"));
    }

    if end_time < start_time {
        return Err(StdError::generic_err("AuctionEndTimeIsBeforeStartTime"));
    }

    if start_time < Uint64::from(env.block.time.seconds()) {
        return Err(StdError::generic_err("AuctionStartTimeIsBeforeNow"));
    }

    // Create the auction
    let mut state = STATE.load(deps.storage)?;
    state.auction_count += 1;
    STATE.save(deps.storage, &state)?;

    let auction = Auction {
        seller: owner.clone(),
        collection: collection.clone(),
        token_id: token_id.clone(),
        start_price: start_price.clone(),
        min_bid_step: min_bid_step.clone(),
        start_time: start_time.clone(),
        end_time: end_time.clone(),
        current_price: Uint128::zero(),
        current_bidder: None,
        status: AuctionStatus::WaitingAuction,
        royalty: royalty.clone(),
    };

    AUCTIONS.save(
        deps.storage,
        (&collection.clone(), &token_id.clone()),
        &auction,
    )?;
    let deposit = Deposits {
        owner: owner.clone(),
        collection: collection.clone(),
        token_id: token_id.clone(),
    };
    DEPOSITS.save(
        deps.storage,
        (&collection.clone(), &owner.clone(), &token_id.clone()),
        &deposit,
    )?;

    // Transfer the NFT from the seller to the marketplace contract
    let transfer_to_marketplace_msg = Cw721ExecuteMsg::TransferNft {
        recipient: env.contract.address.to_string(), // Marketplace contract address
        token_id: token_id.clone(),
    };

    let execute_transfer_to_marketplace = WasmMsg::Execute {
        contract_addr: collection.to_string(),
        msg: to_json_binary(&transfer_to_marketplace_msg)?,
        funds: vec![],
    };
    // Emit an event (using attributes in CosmWasm)
    let response = Response::new()
        .add_message(execute_transfer_to_marketplace)
        .add_attribute("action", "list_nft_for_auction")
        .add_attribute("auction_id", state.auction_count.to_string())
        .add_attribute("creator", owner.clone())
        .add_attribute("collection", collection.clone())
        .add_attribute("token_id", token_id.clone())
        .add_attribute("start_price", start_price.to_string())
        .add_attribute("min_bid_step", min_bid_step.to_string())
        .add_attribute("start_time", start_time.to_string())
        .add_attribute("end_time", end_time.to_string());

    Ok(response)
}

pub fn start_auction(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
    token_id: String, // account_manager_contract_addr: String,
) -> StdResult<Response> {
    let caller = info.sender;
    let current_time = Uint64::from(env.block.time.seconds());

    let auction = AUCTIONS.may_load(deps.storage, (&collection, &token_id))?;
    match auction {
        Some(auction) => {
            if auction.seller != caller && !is_admin(deps.as_ref(), caller.clone())? {
                return Err(StdError::generic_err(
                    "Caller is not the auction owner or an admin",
                ));
            }

            // Check if the auction is in the correct status
            if auction.status != AuctionStatus::WaitingAuction {
                return Err(StdError::generic_err("AuctionNotWaiting"));
            }

            // Check if the auction start time is valid
            if auction.start_time > current_time {
                return Err(StdError::generic_err("AuctionStartTimeIsBeforeNow"));
            }

            // Update the auction status
            let updated_auction = Auction {
                status: AuctionStatus::InAuction,
                ..auction
            };

            AUCTIONS.save(deps.storage, (&collection, &token_id), &updated_auction)?;

            // Emit an event (using attributes in CosmWasm)
            let response = Response::new()
                .add_attribute("action", "start_auction")
                .add_attribute("collection", collection.clone())
                .add_attribute("token_id", token_id.clone())
                .add_attribute("caller", caller.to_string());

            Ok(response)
        }
        None => Err(StdError::generic_err("TokenNotListedForSale")),
    }
}

pub fn cancel_auction(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    collection: String,
    token_id: String,
) -> StdResult<Response> {
    let caller = info.sender;

    // Load the auction
    let auction = AUCTIONS.may_load(deps.storage, (&collection, &token_id))?;
    match auction {
        Some(auction) => {
            // Check if the caller is the auction creator or an admin
            if auction.seller != caller && !is_admin(deps.as_ref(), caller.clone())? {
                return Err(StdError::generic_err("Caller is not the auction owner"));
            }

            // Check if the auction is in the correct status
            if auction.status != AuctionStatus::WaitingAuction {
                return Err(StdError::generic_err("Auction is not in waiting status"));
            }

            // Update the auction status to Cancelled
            let updated_auction = Auction {
                status: AuctionStatus::Cancelled,
                ..auction.clone()
            };

            AUCTIONS.save(deps.storage, (&collection, &token_id), &updated_auction)?;

            DEPOSITS.remove(
                deps.storage,
                (&collection, &auction.seller.to_string(), &token_id),
            );

            // Transfer the NFT back to the creator
            let transfer_msg = nft::contract::Cw721ExecuteMsg::TransferNft {
                recipient: auction.seller.to_string(),
                token_id: auction.token_id.clone(),
            };

            let execute_transfer = WasmMsg::Execute {
                contract_addr: auction.collection.to_string(),
                msg: to_json_binary(&transfer_msg)?,
                funds: vec![],
            };

            // Emit an event (using attributes in CosmWasm)
            let response = Response::new()
                .add_message(execute_transfer) // Transfer NFT back to creator
                .add_attribute("action", "cancel_auction")
                .add_attribute("collection", collection.clone())
                .add_attribute("token_id", token_id.clone())
                .add_attribute("caller", caller.to_string());

            Ok(response)
        }
        None => Err(StdError::generic_err("TokenNotListedForSale")),
    }
}

fn bid_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
    token_id: String,
    price: Uint128,
) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;
    let auction = AUCTIONS.may_load(deps.storage, (&collection, &token_id))?;
    match auction {
        Some(auction) => {
            // Ensure auction is active
            ensure!(
                auction.status == AuctionStatus::InAuction,
                StdError::generic_err("Auction is not active")
            );
            ensure!(
                Uint64::from(env.block.time.seconds()) < auction.end_time,
                StdError::generic_err("Auction has ended")
            );

            // Ensure bid is high enough
            ensure!(
                price >= auction.start_price,
                StdError::generic_err("Bid price too low")
            );
            let min_bid = auction
                .current_price
                .checked_add(auction.min_bid_step)
                .unwrap_or(Uint128::MAX); // Handle overflow gracefully
            ensure!(
                price >= min_bid || auction.current_bidder.is_none(),
                StdError::generic_err("Bid price too low")
            );

            // Transfer bid amount to contract
            let transfer_funds_msg = BankMsg::Send {
                to_address: env.contract.address.to_string(),
                amount: vec![Coin {
                    denom: config.native_denom.to_string(),
                    amount: price,
                }],
            };

            let mut messages: Vec<BankMsg> = vec![transfer_funds_msg];

            // Refund previous bidder if exists
            if let Some(prev_bidder) = auction.current_bidder.clone() {
                let refund_msg = BankMsg::Send {
                    to_address: prev_bidder.to_string(),
                    amount: vec![Coin {
                        denom: config.native_denom.to_string(),
                        amount: auction.current_price,
                    }],
                };
                messages.push(refund_msg);
            }

            // Update auction state
            let updated_auction = Auction {
                current_price: price,
                current_bidder: Some(info.sender.clone()),
                ..auction.clone()
            };
            AUCTIONS.save(deps.storage, (&collection, &token_id), &updated_auction)?;

            // Return response with messages
            Ok(Response::new()
                .add_messages(messages)
                .add_attribute("method", "bid_nft")
                .add_attribute("collection", collection.clone())
                .add_attribute("token_id", token_id.clone())
                .add_attribute("bidder", info.sender.to_string())
                .add_attribute("price", price.to_string()))
        }
        None => Err(StdError::generic_err("TokenNotListedForSale")),
    }
}

fn claim_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
    token_id: String,
) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;
    let auction = AUCTIONS.may_load(deps.storage, (&collection, &token_id))?;
    match auction {
        Some(auction) => {
            // Ensure auction has ended
            ensure!(
                auction.status == AuctionStatus::InAuction,
                StdError::generic_err("Auction is not active")
            );
            ensure!(
                Uint64::from(env.block.time.seconds()) >= auction.end_time,
                StdError::generic_err("Auction has not ended")
            );

            let mut messages: Vec<CosmosMsg> = vec![];

            if let Some(bidder) = auction.current_bidder.clone() {
                // Transfer NFT to highest bidder
                let transfer_nft_msg = CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: auction.collection.to_string(),
                    msg: to_json_binary(&nft::contract::Cw721ExecuteMsg::TransferNft {
                        recipient: bidder.to_string(),
                        token_id: auction.token_id.clone(),
                    })?,
                    funds: vec![],
                });
                messages.push(transfer_nft_msg);

                // Compute fee and seller's earnings
                let fee = auction
                    .current_price
                    .multiply_ratio(auction.royalty, 100u128); // royalty = (listing.price * config.royalty) / 100
                let without_fee = auction.current_price - fee;

                let state = STATE.load(deps.storage)?;

                // Transfer funds to seller
                let transfer_to_creator_msg = CosmosMsg::Bank(BankMsg::Send {
                    to_address: auction.seller.to_string(),
                    amount: vec![Coin {
                        denom: config.native_denom.to_string(),
                        amount: without_fee,
                    }],
                });
                messages.push(transfer_to_creator_msg);

                // Transfer royalty fee
                let transfer_fee_msg = CosmosMsg::Bank(BankMsg::Send {
                    to_address: state.collection_fabric_address.to_string(),
                    amount: vec![Coin {
                        denom: config.native_denom.to_string(),
                        amount: fee,
                    }],
                });
                messages.push(transfer_fee_msg);
            } else {
                // No bids, return NFT to creator
                let transfer_nft_msg = CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: auction.collection.to_string(),
                    msg: to_json_binary(&nft::contract::Cw721ExecuteMsg::TransferNft {
                        recipient: auction.seller.to_string(),
                        token_id: auction.token_id.clone(),
                    })?,
                    funds: vec![],
                });
                messages.push(transfer_nft_msg);
            }

            // Update auction status
            let updated_auction = Auction {
                status: AuctionStatus::Ended,
                ..auction.clone()
            };

            AUCTIONS.save(deps.storage, (&collection, &token_id), &updated_auction)?;

            // Return response with messages
            Ok(Response::new()
                .add_messages(messages)
                .add_attribute("method", "claim_nft")
                .add_attribute("collection", collection.clone())
                .add_attribute("token_id", token_id.clone())
                .add_attribute("claimer", info.sender.to_string()))
        }
        None => Err(StdError::generic_err("TokenNotListedForSale")),
    }
}

fn add_admin(deps: DepsMut, _env: Env, info: MessageInfo, account_id: Addr) -> StdResult<Response> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(StdError::generic_err("AdminAccessError"));
    }

    let mut admins = ADMINS.load(deps.storage).unwrap_or_default();

    // Check if the account is already an admin
    if admins.contains(&account_id) {
        return Err(StdError::generic_err("AdminAccessError"));
    }

    // Add the new admin
    admins.push(account_id.clone());
    ADMINS.save(deps.storage, &admins)?;

    Ok(Response::new()
        .add_attribute("action", "add_admin")
        .add_attribute("caller", info.sender.to_string())
        .add_attribute("admin", account_id.to_string()))
}

fn remove_admin(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    account_id: Addr,
) -> StdResult<Response> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(StdError::generic_err("Admin access error"));
    }

    // Load the current list of admins
    let mut admins = ADMINS.load(deps.storage).unwrap_or_default();

    // Check if the account is an admin
    if !admins.contains(&account_id) {
        return Err(StdError::generic_err("AdminAccessError"));
    }

    // Remove the admin
    admins.retain(|admin| admin != &account_id);
    ADMINS.save(deps.storage, &admins)?;

    Ok(Response::new()
        .add_attribute("action", "remove_admin")
        .add_attribute("caller", info.sender.to_string())
        .add_attribute("admin", account_id.to_string()))
}

pub fn is_admin(deps: Deps, account_id: Addr) -> StdResult<bool> {
    let admins = ADMINS.load(deps.storage)?; // Use `?` to propagate errors
    Ok(admins.contains(&account_id))
}

// Implement the contract's query functions
fn query_listing_count(deps: Deps) -> StdResult<u128> {
    let state = STATE.load(deps.storage)?;
    Ok(state.listing_count)
}

fn query_listing_by_index(deps: Deps, collection: String, token_id: String) -> StdResult<Listing> {
    let listing = LISTINGS.may_load(deps.storage, (&collection, &token_id))?;
    match listing {
        Some(listing) => Ok(listing),
        None => Err(StdError::generic_err("TokenNotListedForSale")),
    }
}

fn query_auction_count(deps: Deps) -> StdResult<u128> {
    let state = STATE.load(deps.storage)?;
    Ok(state.auction_count)
}

fn query_auction_by_index(deps: Deps, collection: String, token_id: String) -> StdResult<Auction> {
    let auction = AUCTIONS.may_load(deps.storage, (&collection, &token_id))?;
    match auction {
        Some(auction) => Ok(auction),
        None => Err(StdError::generic_err("TokenNotListedForSale")),
    }
}

fn query_is_admin(deps: Deps, account_id: Addr) -> StdResult<bool> {
    is_admin(deps, account_id)
}
