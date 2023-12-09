export * as auth from './auth'

import { invoke } from '@tauri-apps/api/tauri'

const epAction = async (action: any) =>
    invoke<boolean>('subject_ep_action', { action })

export const epWatched = async (Watched: string[]) => epAction({ Watched })
export const epQueue = async (Queue: string) => epAction({ Queue })
export const epDrop = async (Drop: string) => epAction({ Drop })
export const epRemove = async (Remove: string) => epAction({ Remove })
