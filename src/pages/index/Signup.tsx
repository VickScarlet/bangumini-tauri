import { Card, CardContent } from '@mui/material'
import { BangumiLogo, } from '@/components';
import SignUpForm from '@/views/auth/SignUpForm'

export default ()=>(
    <Card
        sx={{
            width: 400,
            display: 'flex',
            flexDirection: 'column',
            justifyContent: 'center',
            alignItems: 'center',
            margin: 'auto',
            marginTop: 10,
        }}
        variant='outlined'
        square
    >
        <CardContent>
            <BangumiLogo />
            <SignUpForm />
        </CardContent>
    </Card>
)