import { type ThemeOptions } from '@mui/material/styles'

export default {
    palette: {
        mode: 'light',
        background: {
            default: '#f3f6f9',
            paper: '#ffffff',
        },
        epDrop: {
            main: '#df7a7d',
            contrastText: '#fff',
        },
        epQueue: {
            main: '#58c4eb',
            contrastText: '#fff',
        },
        epWatched: {
            main: '#ac6cff',
            contrastText: '#fff',
        },
        epAired: {
            main: '#dcc3fe',
            contrastText: '#fff',
        },
        epOnAir: {
            main: '#7bdd69',
            contrastText: '#fff',
        },
        epDisable: {
            main: '#f3f6f9',
            contrastText: '#888',
        },
        epProgress: {
            main: '#ac6cff',
        },
    },
} as ThemeOptions
