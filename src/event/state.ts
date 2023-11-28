import { listen } from '@tauri-apps/api/event'
import { emit } from '.'

export const Events = {
    Login: 'login',
    Logout: 'logout',
    LoginChange: 'login-change',
}

const stateHandler = {
    login(value: boolean) {
        emit(Events.LoginChange, value)
        if (value) {
            emit(Events.Login)
        } else {
            emit(Events.Logout)
        }
    },
}

interface Payload {
    state: keyof typeof stateHandler
    value: any
}
listen<Payload>('state-update', event => {
    const { state, value } = event.payload
    console.log('state-update', state, value)
    stateHandler[state]?.(value)
})
