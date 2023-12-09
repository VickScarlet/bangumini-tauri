import { type ThemeOptions } from '@mui/material/styles'

export default {
    palette: {
        primary: {
            main: '#f09199',
            contrastText: '#fff',
        },
        secondary: {
            main: '#7F00FF',
            contrastText: '#fff',
        },
    },
    components: {
        MuiCssBaseline: {
            styleOverrides: {
                body: {
                    margin: 0,
                },
            },
        },
    },
} as ThemeOptions
