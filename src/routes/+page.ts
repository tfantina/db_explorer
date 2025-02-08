import type { PageLoad } from '/$types'
import { getConnections, putConnection } from '../connections/internalConnector'
import Database from '@tauri-apps/plugin-sql'

export const load: PageLoad = async ({ _params }) => {
  return {
    connections: await getConnections()
  }
}
