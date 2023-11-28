import { useState, useEffect } from 'react'
import { getFormhash, getCaptcha, signup } from '@/api/auth'
import { getImageUriFromArray } from '@/utils'
import { FormGroup, FormControl, OutlinedInput, InputAdornment, Button, InputLabel, Grid } from '@mui/material'
import { Email, Password, VerifiedUser } from '@mui/icons-material';
import Image from '@/components/Image'

export default ()=>{
    const [email, setEmail] = useState("")
    const [password, setPassword] = useState("")
    const [captcha, setCaptcha] = useState("")
    const [formhash, setFormhash] = useState("")
    const [captchaSrc, setCaptchaSrc] = useState("")
    const submit = async ()=>{
        const data = { email, password, captcha, formhash };
        console.debug(data);
        const ret = await signup(data)
        console.debug(ret);
    }

    useEffect(()=>{
        getFormhash().then(async (formhash) => {
            setFormhash(formhash)
            const captcha = await getCaptcha();
            setCaptchaSrc(getImageUriFromArray(captcha))
        })
    },[]);

    return <form>
        <FormGroup sx={{width: 350}}>
            <FormControl margin='dense'>
                <InputLabel htmlFor="signin-email-input">Email</InputLabel>
                <OutlinedInput
                    type='email' name='email' autoComplete='email'
                    id='signin-email-input' label='Email' placeholder='Email'
                    startAdornment={<InputAdornment position='start'><Email /></InputAdornment>}
                    value={email} onChange={(e) => setEmail(e.target.value)}
                />
            </FormControl>
            <FormControl margin='dense'>
                <InputLabel htmlFor="signin-password-input">Password</InputLabel>
                <OutlinedInput
                    type='password' name='password' autoComplete='current-password'
                    id='signin-password-input' label='Password' placeholder='Password'
                    startAdornment={<InputAdornment position='start'><Password /></InputAdornment>}
                    value={password} onChange={(e) => setPassword(e.target.value)}
                />
            </FormControl>
            <FormControl margin='dense'>
                <Grid container spacing={1}>
                    <Grid item xs>
                        <InputLabel htmlFor="signin-captcha-input">Captcha</InputLabel>
                        <OutlinedInput
                            type='captcha' name='captcha'
                            id='signin-captcha-input' label='Captcha' placeholder='Captcha'
                            startAdornment={<InputAdornment position='start'><VerifiedUser /></InputAdornment>}
                            value={captcha} onChange={(e) => setCaptcha(e.target.value)}
                        />
                    </Grid>
                    <Grid item xs={5}>
                        <Image src={captchaSrc} width='100%' height={56} style={{objectFit:'contain'}} />
                    </Grid>
                </Grid>
            </FormControl>
            <FormControl margin='dense'>
                <Button
                    type="button" size='large' variant='contained' color='secondary' fullWidth
                    onClick={submit}
                >Login</Button>
            </FormControl>
        </FormGroup>
    </form>
};