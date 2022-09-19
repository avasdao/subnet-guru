/* Import modules. */
const Client = require('bitcoin-core')
const moment = require('moment')
const nodemailer = require('nodemailer')
const PouchDB = require('pouchdb')
const util = require('util')
const { v4: uuidv4 } = require('uuid')

/* Initialize databases. */
// const logsDb = new PouchDB(`http://${process.env.COUCHDB_AUTH}@localhost:5984/logs`)
// const sessionsDb = new PouchDB(`http://${process.env.COUCHDB_AUTH}@localhost:5984/sessions`)
const notifsDb = new PouchDB('dbs/notifs')
const sessionsDb = new PouchDB('dbs/sessions')

/* Initialize new Bitcoin client. */
const client = new Client({
    port: process.env.GURU_RPC_PORT || 7227, // Testnet RPC port is 7229
    host: process.env.GURU_RPC_HOST || '127.0.0.1',
    username: process.env.GURU_RPC_USER || 'user',
    password: process.env.GURU_RPC_PASS || 'password',
})



/**
 * Notifications
 */
const notifs = async function (req, res) {
    let id
    let address
    let body
    let response
    let results

    body = req.body
    console.log('BODY', body)

    /* Validate body. */
    if (!body) {
        /* Set status. */
        res.status(400)

        /* Return error. */
        return res.json({
            error: 'Missing body parameter.'
        })
    }

    address = body.address
    console.log('\nNotification address:', address)

    result = await client.validateAddress(address)
    console.log('\nIs address valid:', result.isvalid, result)

    /* Validate address. */
    if (!result.isvalid) {
        /* Set status. */
        res.status(400)

        /* Return error. */
        return res.json({
            error: 'Your Avalanche address is invalid.'
        })
    }

    const balance = await client.getBalance('*', 0)
    console.log('\nBALANCE', balance)

    // client.getInfo().then((help) => console.log(help))
return res.json({ status: 'done!', balance, })

    /* Generate id. */
    id = uuidv4()

    /* Add record to database. */
    response = await notifsDb
        .put({
            _id: id,
            ...body,
        })
        .catch(err => {
            console.error(err)

            return res.json(err)
        })

    const msgFrom = '"Subnet Guru" <notify@subnet.guru'

    const msgRecipient = 'notify@subnet.guru'

    const msgSubject = 'Subnet Guru Transaction Event'

    const msgDetails = {
        txid: 'd465b82e9d9e74a19b5ea0ac09308be93be8e5f3b46ad8ceb0da99005b7e9b2e',
    }

    // create reusable transporter object using the default SMTP transport
    const transporter = nodemailer.createTransport({
        host: process.env.SMTP_HOST,
        port: 587,
        secure: false, // true for 465, false for other ports
        auth: {
            user: process.env.SMTP_USER,
            pass: process.env.SMTP_PASS,
        },
    })

    // send mail with defined transport object
    const info = await transporter.sendMail({
        from: msgFrom,
        to: msgRecipient,
        subject: msgSubject,
        text: require('../templates/plaintext')(msgDetails),
        html: require('../templates/html')(msgDetails),
    })
    console.log('Message sent: %s', info.messageId)

    /* Send response back to client. */
    res.json({
        databaseId: response.id,
        messageId: info.messageId,
    })
}
