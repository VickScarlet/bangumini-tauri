export * as auth from './auth'
export * as subject from './subject'

import { invoke } from '@tauri-apps/api/tauri'

// export const getIndex = () => invoke<IndexResult>('index')
export const getIndex = async (): Promise<bgm.index.Result> => {
    // if (localStorage.getItem('index') != null) {
    //     return JSON.parse(localStorage.getItem('index') as string)
    // }
    const data = await invoke<bgm.index.Result>('index')
    // if (data.login) {
    //     localStorage.setItem('index', JSON.stringify(data))
    // }
    return data
}
