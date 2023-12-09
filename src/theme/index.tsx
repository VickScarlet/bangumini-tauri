import '@fontsource/roboto/300.css';
import '@fontsource/roboto/400.css';
import '@fontsource/roboto/500.css';
import '@fontsource/roboto/700.css';
import './index.scss'

import { useMemo } from 'react';
import { useMediaQuery } from '@mui/material';
import { createTheme, ThemeProvider } from '@mui/material/styles';
import { zhCN } from '@mui/material/locale';
import themes from './theme'

export default (props:{children?: React.ReactNode})=>{
    const prefersDarkMode = useMediaQuery('(prefers-color-scheme: dark)');

    const theme = useMemo(
      () =>
        createTheme(prefersDarkMode ? themes.dark : themes.light, zhCN),
      [prefersDarkMode],
    );
    return <ThemeProvider theme={theme}>{props.children}</ThemeProvider>
}