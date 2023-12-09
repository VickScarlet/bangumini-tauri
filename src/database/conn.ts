import models from './models'

let instance: IDBDatabase | null = null
export const db = () => {
    if (instance) return instance
    else throw new Error('Database not initialized')
}

export default async function (name: string, version: number) {
    if (instance) return instance
    return new Promise<IDBDatabase>((resolve, reject) => {
        const req = indexedDB.open(name, version)
        req.addEventListener('error', () => reject(req.error))
        req.addEventListener('success', () => {
            instance = req.result
            resolve(req.result)
        })
        req.addEventListener('upgradeneeded', () => {
            for (const model of models) {
                if (req.result.objectStoreNames.contains(model.name)) continue
                const store = req.result.createObjectStore(model.name)
                for (const index of model.indexes)
                    store.createIndex(index.name, index.keyPath, index.options)
            }
        })
    })
}
