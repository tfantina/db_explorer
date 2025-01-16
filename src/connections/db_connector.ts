import Database from "@tauri-apps/plugin-sql";

type Connection = {
    id: number;
    name: string;
    host: string;
    port: string;
    login: string;
    remember: boolean;
    database: string;
}

export const getConnections = async () => {
    try {
        const db = await Database.load("sqlite:test.db");
        const dbConnections = await db.select<Connection[]>("SELECT * FROM connections");
        console.log(dbConnections)
        return dbConnections
    } catch (error) {
        console.log(error)

        return { "error": error }
    }
}


export const putConnection = async (data) => {
    try {
        const db = await Database.load("sqlite:test.db");
        await db.execute("INSERT INTO connections (name, host, port, login, remember, database) VALUES ($1, $2, $3, $4, $5, $6)", 
            data.name,
            data.host,
            data.port,
            data.login,
            data.remember,
            data.database
        )

        getConnections()
    } catch (error) {
        console.log(error)
        return { "error": error }
    }
}