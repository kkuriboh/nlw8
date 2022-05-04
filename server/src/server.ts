import express from 'express'
import cors from 'cors'
import { config } from 'dotenv'
import { routes } from './routes'

config()

const app = express()
app.use(cors())
app.use(express.json())
app.use(routes)

app.listen(3000, () => {
	console.log('Server is running on port 3000')
})
