'use strict'

const express = require('express')
const cors = require('cors')

/* Set constants. */
const HOST = '127.0.0.1'
const PORT = 3000

/* Initialize application. */
const app = express()

/* Initialize CORS. */
app.use(cors())

/* Initialize JSON parser. */
app.use(express.json())

/* Initialize URL parser. */
app.use(express.urlencoded({ extended: true }))

/* Build welcome message. */
const welcome = `
<html>
<body>

<h2>Welcome to the Subnet Guru API</h2>
<h3>https://api.subnet.guru</h3>

</body>
</html>
`

// TODO: Replace with a "static" site.
app.get('/', (req, res) => {
    res.end(welcome)
})

/* Initialize Administration route. */
app.post('/v1/admin', require('./routes/admin'))

/* Initialize Sessions route. */
app.post('/v1/sessions', require('./routes/sessions'))

/* Initialize Command route. */
app.post('/v1/cmd', require('./routes/cmd'))

/* Initialize Notifications route. */
app.post('/v1/notifs', require('./routes/notifs'))

/* Initialize Magic (Email) Link route. */
app.post('/v1/magiclink', require('./routes/magiclink'))

// TODO: Offer help.
app.get('/v1', (req, res) => {
    res.end('Oops! I think you forgot something.')
})

/* Start listening for connections. */
app.listen(PORT, HOST)

/* Display current environment variables. */
console.info()
console.log(`Running on http://${HOST}:${PORT}`)
console.info()
console.info('Current Environment Variables')
console.info('-----------------------------')
console.info('  - NODE_ENV     :', process.env.NODE_ENV)
console.info('  - ADMIN_KEY    :', process.env.ADMIN_KEY)
console.info('  - COUCHDB_AUTH :', process.env.COUCHDB_AUTH)
console.info('  - SMTP_HOST    :', process.env.SMTP_HOST)
console.info('  - SMTP_USER    :', process.env.SMTP_USER)
console.info('  - SMTP_PASS    :', process.env.SMTP_PASS)
console.info()
