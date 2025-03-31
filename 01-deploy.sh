# deploy

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