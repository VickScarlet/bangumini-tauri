import { styled } from '@mui/material/styles'
import { Grid } from '@mui/material'

const Background = styled('div')(({ theme }) => ({
    backgroundColor: theme.palette.pagecenter.main,
    minHeight: '100vh',
}));

export default (props:{children?: React.ReactNode})=><Background>
    <Grid container direction="column" justifyContent="center" sx={{ minHeight: '100vh' }}>
        <Grid container justifyContent="center">{props.children}</Grid>
    </Grid>
</Background>;