import Database from '@tauri-apps/plugin-sql'

export type Connection = {
  name: string
  host: string
  port: number
  login: string
  remember: boolean
  database: string
}

export const getConnections = async () => {
  try {
    const db = await Database.load('sqlite:dbexplorer.db')
    const dbConnections = await db.select<Connection[]>(
      'SELECT * FROM connections'
    )
    return dbConnections
  } catch (error) {
    return { error: error }
  }
}

export const putConnection = async (data: Connection) => {
  try {
    const db = await Database.load('sqlite:dbexplorer.db')
    console.log('DB', db)
    await db.execute(
      'INSERT INTO connections (name, host, port, login, remember, database) VALUES ($1, $2, $3, $4, $5, $6)',
      [
        data.name,
        data.host,
        data.port,
        data.login,
        data.remember,
        data.database
      ]
    )

    const connections = await getConnections()
    return connections
  } catch (error) {
    console.log(error)
    return { error: error }
  }
}
