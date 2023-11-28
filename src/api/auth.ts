import { invoke } from '@tauri-apps/api/tauri'

export const getFormhash = () => invoke<string>('get_formhash')
export const getCaptcha = () => invoke<number[]>('get_captcha')

interface SignUpParams {
    formhash: string
    email: string
    password: string
    captcha: string
}
interface SignUpResult {
    success: boolean
    message?: string
}
export const signup = (params: SignUpParams) =>
    invoke<SignUpResult>('signup', { params })
export const logout = () => invoke<void>('logout')

export const isLogin = () => invoke<boolean>('is_login')
