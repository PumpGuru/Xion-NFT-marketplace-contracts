const { encode } = require('base-64');
const { stringify } = require('querystring');

// Step 1: Prepare the ListingHookMsg
const listingHookMsg = {
  set_listing: {
    owner: 'xion1vyv8t7lj96g0pxhct49zzzscpy6jwqedjckzlx', // Replace with the owner's address
    collection: 'xion128xznqk24ll4qh8yt3rde53thtfa45hv23jzug6536n4hv564a5qug8raf',
    token_id: '2', // Replace with the token ID
    price: '1', // Replace with the price
    royalty: '1', // Replace with the royalty percentage
  },
};

// Step 2: Convert ListingHookMsg to JSON string and base64 encode it
const listingHookMsgJson = JSON.stringify(listingHookMsg);
const msgBase64 = encode(listingHookMsgJson);

// Step 3: Construct the final JSON payload
const executeMsg = {
  ListNftForSale: {
    sender: 'xion1vyv8t7lj96g0pxhct49zzzscpy6jwqedjckzlx', // Replace with the sender's address
    token_id: '2', // Replace with the token ID
    msg: msgBase64, // Base64-encoded ListingHookMsg
  },
};

// Step 4: Convert the final payload to JSON string
const executeMsgJson = JSON.stringify(executeMsg, null, 2);

console.log('Final JSON payload:');
console.log(executeMsgJson);