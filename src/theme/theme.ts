import base from './theme.base'
import _dark from './theme.dark'
import _light from './theme.light'

const merge = <T = Object>(a: T, b: T) => {
    const merged = { ...a }
    for (const key in b) {
        if (typeof b[key] === 'object') {
            merged[key] = merge(a[key], b[key])
        } else {
            merged[key] = b[key]
        }
    }
    return merged
}

export const dark = merge(base, _dark)
export const light = merge(base, _light)

export default { dark, light }
