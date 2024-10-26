import StellarSdk from '@stellar/stellar-sdk';
const fetch = require('node-fetch');

const account = StellarSdk.Keyaccount.random();

console.log("Account Public Key:", account.publicKey());
console.log("Account Secret Key:", account.secret());

async function fundAccount(publicKey: string) {
    try {
        const response = await fetch(`https://friendbot.stellar.org?addr=${encodeURIComponent(publicKey)}`);
        const responseJSON = await response.json();
        console.log("SUCCESS! You have a new account :)\n", responseJSON);
    } catch (e) {
        console.error("ERROR!", e);
    }
};

fundAccount(account.publicKey());