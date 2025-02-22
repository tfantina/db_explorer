// for later implementation
// use sqlx::mysql::MySqlPoolOptions
// use sqlx::sqlite::SqlitePoolOptions
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, Row};
use std::sync::Mutex;
use tauri::State;

pub struct PoolWrapper(pub Mutex<Option<PgPool>>);
// pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Deserialize)]
pub struct QueryRequest {
    query: String,
}

#[derive(Serialize)]
pub struct QueryResponse {
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
}

#[tauri::command]
pub async fn initialize_pool(db_path: &str, state: State<'_, PoolWrapper>) -> Result<String, String> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(20)
        .connect(db_path)
        .await
        .map_err(|e| e.to_string())?;

    let mut pool_wrapper = state.0.lock().unwrap();
    *pool_wrapper = Some(pool);

    Ok("Database pool initialized successfully".to_string())
}

#[tauri::command]
pub fn connect_to_pool(state: State<'_, PoolWrapper>) -> Result<Vec<String>, String> {
    let pool_wrapper = state.0.lock().unwrap();
    let _pool = pool_wrapper
        .as_ref()
        .ok_or("Database pool not initialized")?;


    Ok(vec!["Connected successfully".to_string()])
}

#[tauri::command]
pub async fn execute_query(
    state: State<'_, PoolWrapper>,
    request: QueryRequest,
) -> Result<QueryResponse, String> {
    let pool_wrapper = state.0.lock().unwrap();
    let pool = pool_wrapper
        .as_ref()
        .ok_or("Database pool not initialized")?;

    let resp: Vec<T> = sqlx::query(&request.query).fetch_all(&pool).await.map_err(|e| e.to_string())?;

    let mut columns: Vec<T> = Vec::new();
    let mut rows = Vec::new();

    if let Some(first_row) = resp.get(0) {
        columns = first_row
            .columns()
            .iter()
            .map(|col| col.name().to_string())
            .collect()
    }

    for r in resp {
        let mut row_data = Vec::new();
        for i in 0..r.columns().len() {
            let value: Option<String> = r.try_get(i).unwrap_or(None);
            row_data.push(value.unwrap_or_else(|| "NULL".to_string()));
        }

        rows.push(row_data)
    }

    Ok(QueryResponse {
        columns,
        rows: rows
    })
}
