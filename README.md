# 🗺️ DB Explorer 🕵️‍♀️
A modern tool for querying/managing SQL Databases.

## 🧗‍♂️ Goals
I'd like to make something similar to tools like TablePlus or SQLPro Studio; both of which are
far superior to any other tools out there from a design standpoint.  However, both are, in my
opinion lacking some features that I think would massively improve the user experience.

## Planned Features (roadmap)
1. Multiple pools can be open at the same time, queries will go to whichever DB you have selected
2. Searchable History - allow users to set a limit 100, 500, 1000 queries back
3. History is not specific to a DB, any query can be run across any DB (this is to make queries more reusable)
4. Auto suggestions are contextual to the DB
5. Anticipatory queries: if the user starts typing `select * from users where id` DB Explorer should
run a query `select id from users order by updated_at desc limit 5` and display those ids. This will
take a bit of effort to pull off properly but the more we can anticipate specific queries.
6. Configurable (possibly via YAML or an actual menu) turn off anticipatory queries, set history, custom themes etc.

## 🏗️ Setup

Install yarn dependencies `yarn install`
Navigate to `/src-tauri` and run `cargo install`
Back in the main directory run `yarn tauri dev` to start the application.
The application's SQLite DB, on Mac, is located in `Library/Application Support/com.dbexplorer` I'm not
positive where it would be on Windows.

## 🤰Dependencies
Rust & TS based on Tauri convention
Check out cargo.toml for more details
Svelte for the front end
