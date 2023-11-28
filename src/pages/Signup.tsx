import { Box, Card } from '@mui/material'
import { BangumiLogo, PageCenter } from '@/components';
import SignUpForm from '@/views/auth/SignUpForm'

export default ()=><PageCenter>
    <Card sx={{boxShadow: 0}}>
        <Box sx={{ p: { xs: 2, sm: 3, xl: 5 }}}>
            <BangumiLogo />
            <SignUpForm />
        </Box>
    </Card>
</PageCenter>