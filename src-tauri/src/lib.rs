// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri_plugin_sql::{Migration, MigrationKind};

mod db_connector;
use db_connector::{connect_to_pool, execute_query, initialize_pool, PoolWrapper};
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![Migration {
        version: 1,
        description: "create connections table",
        sql: "CREATE TABLE IF NOT EXISTS connections (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name STRING,
                host STRING,
                port STRING,
                login STRING,
                remember BOOLEAN,
                database STRING
            )",
        kind: MigrationKind::Up,
    }];

    let db_url = "sqlite:dbexplorer.db";
    tauri::Builder::default()
        .manage(PoolWrapper(Mutex::new(None)))
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations(db_url, migrations)
                .build(),
        )
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            initialize_pool,
            connect_to_pool,
            execute_query
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
