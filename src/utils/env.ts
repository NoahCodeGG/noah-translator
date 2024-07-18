import { getVersion } from '@tauri-apps/api/app'
import { arch as archFn, type, version } from '@tauri-apps/api/os'

export let osType = ''
export let arch = ''
export let osVersion = ''
export let appVersion = ''

export async function initEnv() {
  osType = await type()
  arch = await archFn()
  osVersion = await version()
  appVersion = await getVersion()
}
