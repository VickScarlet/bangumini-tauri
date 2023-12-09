import { type ThemeOptions } from '@mui/material/styles'

export default {
    palette: {
        mode: 'dark',
        background: {
            default: '#1b1d26',
            paper: '#0f111a',
            light: '#20222c',
        },
        epDrop: {
            main: '#ea5d5d',
            contrastText: '#eee',
        },
        epQueue: {
            main: '#20ace6',
            contrastText: '#eee',
        },
        epWatched: {
            main: '#7410ec',
            contrastText: '#eee',
        },
        epAired: {
            main: '#be97f6',
            contrastText: '#eee',
        },
        epOnAir: {
            main: '#2aa411',
            contrastText: '#eee',
        },
        epDisable: {
            main: '#505b64',
            contrastText: '#eee',
        },
        epProgress: {
            main: '#7410ec',
        },
    },
} as ThemeOptions
