import { Grid, Box } from '@mui/material'
import Bangumi from '@/assets/bangumi.svg'

export default ()=><Grid container justifyContent="center">
    <Box sx={{paddingY: 1}}>
        <img src={Bangumi} alt="Bangumi" width={100} />
    </Box>
</Grid>