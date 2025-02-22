<script lang="ts">
  import type { PageProps } from './$types'
  import { invoke } from '@tauri-apps/api/core'
  import {
    getConnections,
    putConnection
  } from '../connections/internalConnector'
  import type { Connection } from '../connections/internalConnector'
  import { postgresQuery } from '../connections/externalConnector'
  import Database from '@tauri-apps/plugin-sql'
  import '../styles/style.css'

  enum Engines {
    Mysql = 'Mysql',
    Postgres = 'Postgres',
    Sqlite = 'Sqlite'
  }
  interface QueryResponse {
    columns: string[]
    rows: string[][]
  }

  let name: string = $state(''),
    host: string = $state(''),
    port: number = $state(0),
    username: string = $state(''),
    database: string = $state(''),
    remember: boolean = $state(false),
    querystr: string = $state(''),
    engine: Engines = $state(Engines.Mysql),

  let { data }: PageProps = $props()
  let localData = { ...data }



  async function createConnection() {
    const data = {
      name: name,
      host: host,
      port: port,
      login: username,
      remember: remember,
      database: database,
      kind: engine
    }

    localData = await putConnection(data)
  }

  async function refresh() {
    const test: Connection = {
      name: 'localhost',
      host: 'localhost',
      port: 5432,
      login: 'postgres',
      remember: false,
      database: 'postbox_dev'
    }

    const rest = await invoke('initialize_pool', {
      dbPath: `postgres://${test.login}@${test.host}:${test.port}/${test.database}`
    })

    console.log(rest)
  }

  async function query() {
    const res = await invoke<QueryResponse>('execute_query', {
      request: {
        query: querystr
      }
    })

    console.log(res)
  }
</script>

<main class="container">
  <div class="section">
    <input type="textarea" bind:value={querystr} />
    <button onclick={query}>Execute</button>
  </div>
  <div class="col">
    <ul>
      hi
      {#each localData.connections as connection}
        <li>{connection.name}</li>
      {/each}
    </ul>
  </div>
  <div class="col">
    <form class="row d-flex f-column">
      <div class="input-group">
        <button onclick={refresh}>Refresh</button>
      </div>
      <div class="input-group">
        <button onclick={query}>Query</button>
      </div>
      <dib class="input-group">
          <input type="select">
          <option>MySQL</option>
          <option>Postgres</option>
          <option>SQLite</option>
          </input>
      </div>
      <div class="input-group">
        <input
          id="name"
          name="name"
          bind:value={name}
          placeholder="Enter a name..."
        />
      </div>
      <div class="input-group">
        <input
          id="host"
          name="host"
          bind:value={host}
          placeholder="Enter a host..."
        />
      </div>
      <div class="input-group">
        <input
          id="port"
          name="port"
          bind:value={port}
          placeholder="Enter a port..."
        />
      </div>
      <div class="input-group">
        <input
          id="username"
          name="username"
          bind:value={username}
          placeholder="Enter a username.."
        />
      </div>
      <div class="input-group">
        <input
          type="checkbox"
          id="remember"
          bind:checked={remember}
          name="remember"
        />
      </div>
      <div class="input-group">
        <input type="database" bind:value={database} name="database" />
      </div>
      <div class="input-group">
        <button type="submit" onclick={createConnection}>Greet</button>
      </div>
    </form>
  </div>
</main>
