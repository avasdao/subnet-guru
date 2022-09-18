/**
 * Plaintext Email Template
 *
 * This version will accompany the message and will be displayed in the event
 * that HTML is unavailable or disabled by the user.
 */
const plaintextTemplate = (_msgDetails) => {
    return `
        Subnet Guru Event
        ----------------------------------------

        We just saw a transaction on your watched wallet.
        ${_msgDetails.txid}

        ________________________________________
        https://subnet.guru
        brought to you with ❤️ from Ava's DAO
    `
}

/* Export module. */
module.exports = plaintextTemplate
