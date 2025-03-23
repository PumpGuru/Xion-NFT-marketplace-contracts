const { encode } = require('base-64');
const { stringify } = require('querystring');

// Step 1: Prepare the ListingHookMsg
const listingHookMsg = {
  set_listing: {
    owner: 'xion1nmx9wtrkmdvfkrnrwkxc5uyduqa4l29wg3vd8e', // Replace with the owner's address
    collection: 'xion16pxfzmt6h9fj8eywgcew5e3j52fj4uaa6vqzjxv0du8xs46fkfjslz6ysk',
    token_id: '0', // Replace with the token ID
    price: '10000', // Replace with the price
    royalty: '1', // Replace with the royalty percentage
  },
};

// Step 2: Convert ListingHookMsg to JSON string and base64 encode it
const listingHookMsgJson = JSON.stringify(listingHookMsg);
const msgBase64 = encode(listingHookMsgJson);

// Step 3: Construct the final JSON payload
const executeMsg = {
  ListNftForSale: {
    sender: 'xion1nmx9wtrkmdvfkrnrwkxc5uyduqa4l29wg3vd8e', // Replace with the sender's address
    token_id: '0', // Replace with the token ID
    msg: msgBase64, // Base64-encoded ListingHookMsg
  },
};

// Step 4: Convert the final payload to JSON string
const executeMsgJson = JSON.stringify(executeMsg, null, 2);

console.log('Final JSON payload:');
console.log(executeMsgJson);