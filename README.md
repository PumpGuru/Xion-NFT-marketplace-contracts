# xion_nft_marketplace_contracts
This is NFT marketplace contracts for Xion chain.

# NFT Marketplace: Cosmos-Based Decentralized Exchange

A decentralized marketplace smart contract built on the Cosmos blockchain using CosmWasm. This contract enables secure and peer-to-peer exchange of NFTs, supporting both Juno native coins and CW20 tokens.
## Overview

- [CosmWasm](https://github.com/CosmWasm/cosmwasm)
- 
## Key Features
* NFT Listing: Intuitive interface for users to create listings for their NFTs, including descriptive metadata, pricing, and media.
* Offer System: Streamlined process for buyers to make offers on listed NFTs, fostering transparent negotiation.
* Secure Transactions: Robust payment mechanisms to guarantee safe transfer of funds (native coins and CW20 tokens) and NFT ownership.
* Dispute Resolution (Optional): [If you've implemented one] Mechanisms to handle potential disputes, ensuring fair outcomes for both buyers and sellers.

## Getting Started


### Offer System:

* Enables potential buyers to make offers on listed items
* Ensures transparent negotiation processes.
* Secure Transactions:
* Employs dispute resolution systems as needed.
make a better readme.md  using readme operators





################################################################################################################
######################################## Contract Deploy #######################################################
################################################################################################################
# xion install
wget https://github.com/burnt-labs/xion/releases/download/v17.0.0/xiond_17.0.0_linux_amd64.tgz

xiond q bank balances xion1vyv8t7lj96g0pxhct49zzzscpy6jwqedjckzlx --node https://rpc.xion-testnet-2.burnt.com:443 

cargo update
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/optimizer:0.16.1
  
RES=$(xiond tx wasm store ./artifacts/marketplace.wasm \
      --chain-id xion-testnet-2 \
      --gas-adjustment 1.3 \
      --gas-prices 0.1uxion \
      --gas auto \
      -y --output json \
      --node https://rpc.xion-testnet-2.burnt.com:443 \
      --from $WALLET)
echo $RES

TXHASH=
CODE_ID=$(xiond query tx $TXHASH \
  --node https://rpc.xion-testnet-2.burnt.com:443 \
  --output json | jq -r '.events[-1].attributes[1].value')
  
MSG='{"native_denom": "uxion", "owner":"xion1vyv8t7lj96g0pxhct49zzzscpy6jwqedjckzlx", "collection_fabric_address": "xion1vyv8t7lj96g0pxhct49zzzscpy6jwqedjckzlx", "royalty":"1"}'

xiond tx wasm instantiate $CODE_ID "$MSG" \
  --from $WALLET \
  --label "MarketPlace" \
  --gas-prices 0.025uxion \
  --gas auto \
  --gas-adjustment 1.3 \
  -y --no-admin \
  --chain-id xion-testnet-2 \
  --node https://rpc.xion-testnet-2.burnt.com:443
  
CONTRACT=$(xiond query tx $TXHASH \
  --node https://rpc.xion-testnet-2.burnt.com:443 \
  --output json | jq -r '.events[] | select(.type == "instantiate") | .attributes[] | select(.key == "_contract_address") | .value')

echo $CONTRACT

QUERY='{"get_all_asks":{}}'
xiond query wasm contract-state smart $CONTRACT "$QUERY" --output json --node https://rpc.xion-testnet-2.burnt.com:443


RES=$(xiond tx wasm store ./artifacts/cw20_impl.wasm \
      --chain-id xion-testnet-2 \
      --gas-adjustment 1.3 \
      --gas-prices 0.1uxion \
      --gas auto \
      -y --output json \
      --node https://rpc.xion-testnet-2.burnt.com:443 \
      --from $WALLET)
echo $RES

MSG='{"name": "GoldToken", "symbol":"GT", "decimals": 6, "initial_balances": [{"address": "xion1vyv8t7lj96g0pxhct49zzzscpy6jwqedjckzlx", "amount": "10000000000"}], }'

RES=$(xiond tx wasm store ./artifacts/nft.wasm \
      --chain-id xion-testnet-2 \
      --gas-adjustment 1.3 \
      --gas-prices 0.1uxion \
      --gas auto \
      -y --output json \
      --node https://rpc.xion-testnet-2.burnt.com:443 \
      --from $WALLET)
echo $RES

MSG='{"name": "GoldNFT", "symbol":"GOLD", "minter": "xion1vyv8t7lj96g0pxhct49zzzscpy6jwqedjckzlx"}'

RES=$(xiond tx wasm store ./artifacts/user.wasm \
      --chain-id xion-testnet-2 \
      --gas-adjustment 1.3 \
      --gas-prices 0.1uxion \
      --gas auto \
      -y --output json \
      --node https://rpc.xion-testnet-2.burnt.com:443 \
      --from $WALLET)
echo $RES

MSG='{"owner": "xion1vyv8t7lj96g0pxhct49zzzscpy6jwqedjckzlx", "user_data":{"field1": "User", "field2":100000}}'

RES=$(xiond tx wasm store ./artifacts/account_manager.wasm \
      --chain-id xion-testnet-2 \
      --gas-adjustment 1.3 \
      --gas-prices 0.1uxion \
      --gas auto \
      -y --output json \
      --node https://rpc.xion-testnet-2.burnt.com:443 \
      --from $WALLET)
echo $RES

MSG='{"user_code_hash": "1234567890abcdefgh", "creator_code_hash":"1234567890abcdefgh"}'





################################################################################################################
###########                                      Actions                                             ###########
################################################################################################################

#///////////// NFT Collection Deploy ///////////////////////////

#------------ Upload Optimized Contract On-chain --------
WALLET=xion1nmx9wtrkmdvfkrnrwkxc5uyduqa4l29wg3vd8e
RES=$(xiond tx wasm store ./artifacts/nft.wasm \
      --chain-id xion-testnet-2 \
      --gas-adjustment 1.3 \
      --gas-prices 0.1uxion \
      --gas auto \
      -y --output json \
      --node https://rpc.xion-testnet-2.burnt.com:443 \
      --from $WALLET)

echo $RES

#------------ Retrieve the Code ID -----------------------
TXHASH="yourHASH"
CODE_ID=$(xiond query tx $TXHASH \
  --node https://rpc.xion-testnet-2.burnt.com:443 \
  --output json | jq -r '.events[-1].attributes[1].value')

echo $CODE_ID

#------------ Instantiate the Contract -------------------
MSG='{
    "minter": "xion1nmx9wtrkmdvfkrnrwkxc5uyduqa4l29wg3vd8e",
    "name": "TestNFTCollection01",
    "symbol": "TNFT01"
}'
xiond tx wasm instantiate $CODE_ID "$MSG" \
  --from $WALLET \
  --label "cw-counter" \
  --gas-prices 0.025uxion \
  --gas auto \
  --gas-adjustment 1.3 \
  -y --no-admin \
  --chain-id xion-testnet-2 \
  --node https://rpc.xion-testnet-2.burnt.com:443

#------------ Retrieve the Contract Address -------------
TXHASH="yourHASH"
NFT_CONTRACT=$(xiond query tx $TXHASH \
  --node https://rpc.xion-testnet-2.burnt.com:443 \
  --output json | jq -r '.events[] | select(.type == "instantiate") | .attributes[] | select(.key == "_contract_address") | .value')

echo $NFT_CONTRACT



#/////////////     NFT  Mint    ////////////////////////
NFT_CONTRACT=xion187vknfjd9a5yyy77nxs432axv2p7rah6rf96fpd70eeyklvgyjxs04r8rx
TRX_COMMAND='{"mint": {
    "token_id": "0",
    "owner": "xion1nmx9wtrkmdvfkrnrwkxc5uyduqa4l29wg3vd8e",
    "token_uri": "https://tokenuri.com",
    "extension": {
        "is_for_hire": true
    }
}}'
xiond tx wasm execute $NFT_CONTRACT "$TRX_COMMAND" \
  --from $WALLET \
  --gas-prices 0.025uxion \
  --gas auto \
  --gas-adjustment 1.3 \
  -y \
  --node https://rpc.xion-testnet-2.burnt.com:443 \
  --chain-id xion-testnet-2



#/////////////     Approve NFT item to marketplace contract     ////////////////////////
MARKETPLACE_CONTRACT=xion1wvdju7dxvxfsmc9v88jvw0yht36mh0hzxcxuzu3ks8up9l8mvgvsq4e37y
NFT_CONTRACT=xion187vknfjd9a5yyy77nxs432axv2p7rah6rf96fpd70eeyklvgyjxs04r8rx
APPROVE_MSG='{"approve":{"spender":"'$MARKETPLACE_CONTRACT'","token_id":"2"}}'
xiond tx wasm execute $NFT_CONTRACT "$APPROVE_MSG" \
  --from $WALLET \
  --gas-prices 0.025uxion \
  --gas auto \
  --gas-adjustment 1.3 \
  -y \
  --node https://rpc.xion-testnet-2.burnt.com:443 \
  --chain-id xion-testnet-2



#/////////////   get owner of NFT item   ////////////////////////
QUERY='{"owner_of":{"token_id":"2"}}'
xiond query wasm contract-state smart $NFT_CONTRACT "$QUERY" --output json --node https://rpc.xion-testnet-2.burnt.com:443




#/////////////     NFT  List    ////////////////////////
WALLET=xion1nmx9wtrkmdvfkrnrwkxc5uyduqa4l29wg3vd8e
MARKETPLACE_CONTRACT=xion1wvdju7dxvxfsmc9v88jvw0yht36mh0hzxcxuzu3ks8up9l8mvgvsq4e37y
LIST_NFT='{
  "ListNftForSale": {
    "sender": "xion1nmx9wtrkmdvfkrnrwkxc5uyduqa4l29wg3vd8e",
    "token_id": "0",
    "msg": "eyJzZXRfbGlzdGluZyI6eyJvd25lciI6Inhpb24xdnl2OHQ3bGo5NmcwcHhoY3Q0OXp6enNjcHk2andxZWRqY2t6bHgiLCJjb2xsZWN0aW9uIjoieGlvbjEyOHh6bnFrMjRsbDRxaDh5dDNyZGU1M3RodGZhNDVodjIzanp1ZzY1MzZuNGh2NTY0YTVxdWc4cmFmIiwidG9rZW5faWQiOiIyIiwicHJpY2UiOiIxIiwicm95YWx0eSI6IjEifX0="
  }
}'
xiond tx wasm execute $MARKETPLACE_CONTRACT "$LIST_NFT" \
  --from $WALLET \
  --gas-prices 0.025uxion \
  --gas auto \
  --gas-adjustment 1.3 \
  -y \
  --node https://rpc.xion-testnet-2.burnt.com:443 \
  --chain-id xion-testnet-2



#///////////////     Get Listing Count     ////////////////////////
MARKETPLACE_CONTRACT=xion1wvdju7dxvxfsmc9v88jvw0yht36mh0hzxcxuzu3ks8up9l8mvgvsq4e37y
QUERY='{"GetListingCount":{}}'
xiond query wasm contract-state smart $MARKETPLACE_CONTRACT "$QUERY" --output json --node https://rpc.xion-testnet-2.burnt.com:443

#///////////////     Get Listing Item      ////////////////////////
QUERY='{"GetListingByIndex":{"index":"0"}}'
xiond query wasm contract-state smart $MARKETPLACE_CONTRACT "$QUERY" --output json --node https://rpc.xion-testnet-2.burnt.com:443






#/////////////     Buy NFT     ////////////////////////
WALLET=xion1nmx9wtrkmdvfkrnrwkxc5uyduqa4l29wg3vd8e
MARKETPLACE_CONTRACT=xion1wvdju7dxvxfsmc9v88jvw0yht36mh0hzxcxuzu3ks8up9l8mvgvsq4e37y
BUY_NFT='{"BuyNft": {"collection":"'$NFT_CONTRACT'", "token_id":"0"}}'
xiond tx wasm execute $MARKETPLACE_CONTRACT "$BUY_NFT" \
  --amount 1000000uxion \
  --from $WALLET \
  --gas-prices 0.025uxion \
  --gas auto \
  --gas-adjustment 1.3 \
  -y \
  --node https://rpc.xion-testnet-2.burnt.com:443 \
  --chain-id xion-testnet-2
















#////////////////  Transfer Xion   /////////////////////////
xiond tx bank send xion1lz9v7xqwvn28engpl2qlqslc8lk9u8rfppwwxz xion1nmx9wtrkmdvfkrnrwkxc5uyduqa4l29wg3vd8e 2xion \
  --from mykey1 \
  --chain-id xion-testnet-2 \
  --node https://rpc.xion-testnet-2.burnt.com:443 \
  --gas-prices 0.025uxion \
  --gas 500000 \
  --gas-adjustment 1.3 \
  -y




NFT_Marketplace: xion1wvdju7dxvxfsmc9v88jvw0yht36mh0hzxcxuzu3ks8up9l8mvgvsq4e37y
CW20 token impl   xion1nndkf299q2sfkjxxp60zfpqz87ju7374durk9haeqjuncclw64dqa0geqj
NFT token impl: xion187vknfjd9a5yyy77nxs432axv2p7rah6rf96fpd70eeyklvgyjxs04r8rx
User: xion1nmx9wtrkmdvfkrnrwkxc5uyduqa4l29wg3vd8e
AccountManager: xion18fjq9gnzjmjgs5tmle94654uhu8vjfj25d82n8skxk7cu0jksf4qztqqkn