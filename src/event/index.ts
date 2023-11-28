import EventEmitter from 'eventemitter3'
export { Events as StateEvents } from './state'

const eventEmitter = new EventEmitter()

export default eventEmitter
export const on = eventEmitter.on.bind(eventEmitter)
export const once = eventEmitter.once.bind(eventEmitter)
export const off = eventEmitter.off.bind(eventEmitter)
export const emit = eventEmitter.emit.bind(eventEmitter)
