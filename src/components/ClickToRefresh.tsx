import { Box, Typography } from '@mui/material'

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
    <Typography variant="subtitle1" color='text.secondary' component="div" fontWeight={900}>
        点击刷新
    </Typography>
</Box>