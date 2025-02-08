import Database from '@tauri-apps/plugin-sql'
import type { Connection } from './internalConnector'

export type ConnectionString = {
  name: string
  host: string
  port: number
  login: string
  remember: boolean
  database: string
}

export const postgresQuery = async (connection: Connection) => {
  try {
    const { host, port, login, database } = connection
    const db = await Database.load(
      `postgres://${login}@${host}:${port}/${database}`
    )

    console.log(db)
    const dbConnections = await db.select<Connection[]>('SELECT * FROM users')

    return dbConnections
  } catch (error) {
    return { error: error }
  }
}
