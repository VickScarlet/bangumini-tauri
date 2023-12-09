import { invoke } from '@tauri-apps/api/tauri'

export const getFormhash = () => invoke<string>('get_formhash')
export const getCaptcha = () =>
    invoke<bgm.auth.signup.CaptchaBuffer>('get_captcha')

export const signup = (params: bgm.auth.signup.Params) =>
    invoke<bgm.auth.signup.Result>('signup', { params })
export const logout = () => invoke<bgm.auth.logout.Result>('logout')
