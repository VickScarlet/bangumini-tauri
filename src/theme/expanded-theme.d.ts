import '@mui/material/styles/createPalette'
declare module '@mui/material/styles/createPalette' {
    interface PaletteOptions {
        epProgress?: PaletteColorOptions
        epDrop?: PaletteColorOptions
        epQueue?: PaletteColorOptions
        epWatched?: PaletteColorOptions
        epAired?: PaletteColorOptions
        epOnAir?: PaletteColorOptions
        epDisable?: PaletteColorOptions
    }
}

import '@mui/material/LinearProgress'
declare module '@mui/material/LinearProgress' {
    interface LinearProgressPropsColorOverrides {
        epProgress: true
    }
}
