export * as auth from './auth'

import { invoke } from '@tauri-apps/api/tauri'

export const getIndex = () => invoke<any>('index')
// globalThis.getIndex = getIndex
