import { Box, CircularProgress } from '@mui/material'

export default ()=><Box
    sx={{
        width: '100%',
        height: '100%',
        display: 'flex',
        flexDirection: 'column',
        justifyContent: 'center',
        alignItems: 'center',
    }}
    bgcolor='background.default'
>
    <CircularProgress />
</Box>