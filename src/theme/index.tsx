import '@fontsource/roboto/300.css';
import '@fontsource/roboto/400.css';
import '@fontsource/roboto/500.css';
import '@fontsource/roboto/700.css';
import './index.scss'

import { createTheme, ThemeProvider } from '@mui/material/styles';
import { zhCN } from '@mui/material/locale';

const theme = createTheme({
    palette: {
        primary: {
            main: '#f09199',
            contrastText: '#fff',
        },
        secondary: {
            main: '#7F00FF',
            contrastText: '#fff',
        },
        pagecenter: { main: '#eef2f6' },
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
}, zhCN);


export default (props:{children?: React.ReactNode})=><ThemeProvider theme={theme}>{props.children}</ThemeProvider>