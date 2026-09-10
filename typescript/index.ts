import Fastify from 'fastify'
import { z } from 'zod'

const app = Fastify()

const payloadSchema = z.object({
  client_id: z.string().uuid(),
  email: z.string().email(),
  is_active: z.boolean(),
  appointments: z.array(z.object({
    id: z.number(),
    date: z.string(),
    duration_minutes: z.number(),
    rate: z.number()
  }))
})

app.get('/', async () => {
  return "Hello, world!"
})

app.get('/loop', async () => {
  let sum = 0
  for (let i = 0; i < 1_000_000; i++) {
    sum += i
  }
  return sum.toString()
})

app.post('/process', async (req, reply) => {
  const parsed = payloadSchema.parse(req.body)
  const total_cost = parsed.appointments.reduce((acc, app) => acc + (app.duration_minutes / 60.0) * app.rate, 0.0)
  return { client_id: parsed.client_id, total_cost }
})

app.listen({ port: 3001, host: '0.0.0.0' })
