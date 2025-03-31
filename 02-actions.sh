# actions

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
TXHASH="8847D974709C4A95E522C1A2B029F07437F4EB47D003E7A89D55F096D589E6D3"
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
TXHASH="2D7C3C532120F7786D1660876F02E508F096503D4ED817F4508AF5ABDB247DBB"
NFT_CONTRACT=$(xiond query tx $TXHASH \
  --node https://rpc.xion-testnet-2.burnt.com:443 \
  --output json | jq -r '.events[] | select(.type == "instantiate") | .attributes[] | select(.key == "_contract_address") | .value')

echo $NFT_CONTRACT



#/////////////     NFT  Mint    ////////////////////////
TRX_COMMAND='{"mint": {
    "token_id": "0",
    "owner": "xion1nmx9wtrkmdvfkrnrwkxc5uyduqa4l29wg3vd8e",
    "token_uri": "https://token_uri.com",
    "extension": {
        "name":"testItem01",
        "is_for_hire": true
    }
}}'
xiond tx wasm execute $NFT_CONTRACT "$TRX_COMMAND" \
  --from $WALLET \
  --gas-prices 0.025uxion \
  --gas auto \
  --gas-adjustment 1.5 \
  -y \
  --node https://rpc.xion-testnet-2.burnt.com:443 \
  --chain-id xion-testnet-2



#/////////////     Approve NFT item to marketplace contract     ////////////////////////
MARKETPLACE_CONTRACT=xion14j55majznaasw9upyf7ddpflrpy4qakag034wgs7wjdfn0q52fkqjvavld
APPROVE_MSG='{"approve":{"spender":"'$MARKETPLACE_CONTRACT'","token_id":"0"}}'
xiond tx wasm execute $NFT_CONTRACT "$APPROVE_MSG" \
  --from $WALLET \
  --gas-prices 0.025uxion \
  --gas auto \
  --gas-adjustment 1.3 \
  -y \
  --node https://rpc.xion-testnet-2.burnt.com:443 \
  --chain-id xion-testnet-2



#/////////////     NFT  List    ////////////////////////
WALLET=xion1nmx9wtrkmdvfkrnrwkxc5uyduqa4l29wg3vd8e
LIST_NFT='{
  "ListNftForSale": {
    "sender": "xion1nmx9wtrkmdvfkrnrwkxc5uyduqa4l29wg3vd8e",
    "token_id": "0",
    "msg": "eyJzZXRfbGlzdGluZyI6eyJvd25lciI6Inhpb24xbm14OXd0cmttZHZma3JucndreGM1dXlkdXFhNGwyOXdnM3ZkOGUiLCJjb2xsZWN0aW9uIjoieGlvbjE2cHhmem10Nmg5Zmo4ZXl3Z2NldzVlM2o1MmZqNHVhYTZ2cXpqeHYwZHU4eHM0NmZrZmpzbHo2eXNrIiwidG9rZW5faWQiOiIwIiwicHJpY2UiOiIxMDAwMCIsInJveWFsdHkiOiIxIn19"
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


#/////////////   get owner of NFT item   ////////////////////////
QUERY='{"owner_of":{"token_id":"0"}}'
xiond query wasm contract-state smart $NFT_CONTRACT "$QUERY" --output json --node https://rpc.xion-testnet-2.burnt.com:443


#///////////////     Get Listing Count     ////////////////////////
MARKETPLACE_CONTRACT=xion14j55majznaasw9upyf7ddpflrpy4qakag034wgs7wjdfn0q52fkqjvavld
QUERY='{"GetListingCount":{}}'
xiond query wasm contract-state smart $MARKETPLACE_CONTRACT "$QUERY" --output json --node https://rpc.xion-testnet-2.burnt.com:443

#///////////////     Get Listing Item      ////////////////////////
QUERY='{"GetListingByIndex":{"index":"0"}}'
xiond query wasm contract-state smart $MARKETPLACE_CONTRACT "$QUERY" --output json --node https://rpc.xion-testnet-2.burnt.com:443






#/////////////     Buy NFT     ////////////////////////
WALLET=xion18zw77p54acp7rduyp082dfx5c99em5yc0t72qs
BUY_NFT='{"BuyNft": {"collection":"'$NFT_CONTRACT'", "token_id":"0"}}'
xiond tx wasm execute $MARKETPLACE_CONTRACT "$BUY_NFT" \
  --from $WALLET \
  --gas-prices 0.025uxion \
  --gas auto \
  --gas-adjustment 1.5 \
  -y \
  --node https://rpc.xion-testnet-2.burnt.com:443 \
  --chain-id xion-testnet-2 \
  --amount 10000uxion













#////////////////  Transfer Xion   /////////////////////////
xiond tx bank send xion1lz9v7xqwvn28engpl2qlqslc8lk9u8rfppwwxz xion1nmx9wtrkmdvfkrnrwkxc5uyduqa4l29wg3vd8e 2xion \
  --from mykey1 \
  --chain-id xion-testnet-2 \
  --node https://rpc.xion-testnet-2.burnt.com:443 \
  --gas-prices 0.025uxion \
  --gas 500000 \
  --gas-adjustment 1.3 \
  -y




NFT_Marketplace: xion14j55majznaasw9upyf7ddpflrpy4qakag034wgs7wjdfn0q52fkqjvavld
CW20 token impl   xion1nndkf299q2sfkjxxp60zfpqz87ju7374durk9haeqjuncclw64dqa0geqj
NFT token impl: xion1s6mele75a298hrvn29l0eac5gs0fsqqjlw0pxzehhfjlfkpnjahsswsc8u
User: xion1nmx9wtrkmdvfkrnrwkxc5uyduqa4l29wg3vd8e
AccountManager: xion18fjq9gnzjmjgs5tmle94654uhu8vjfj25d82n8skxk7cu0jksf4qztqqkn