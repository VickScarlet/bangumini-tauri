import { useState, useMemo } from 'react'
import { Card, CardMedia, CardContent, Box, Typography, Collapse, LinearProgress } from '@mui/material'
import { Episodes } from './Episode'
import { subject } from '@/api'

interface SubjectProps {
    subject: bgm.subject.Anime
    onStateChange?: (subjectId: string)=>void
}

export default ({ subject }: SubjectProps)=>{
    const [click, setClick] = useState(false);
    const mainEp = useMemo(()=>subject.eps.find(e => e.def)?.eps??[],[subject])
    const watched = useMemo(()=>mainEp.reduce((acc, {state}) => state?acc+1:acc, 0)/mainEp.length*100,[mainEp])
    const aired = useMemo(()=>mainEp.reduce((acc, {on_air}) => !!on_air && new Date(on_air).getTime() < Date.now()?acc+1:acc, 0)/mainEp.length*100,[mainEp]);
    const [timeoutId, setTimeoutId] = useState<ReturnType<typeof setTimeout>|null>(null);
    const onClick = async (ep: string, e: React.MouseEvent)=>{
        e.preventDefault();
        e.stopPropagation();
        if(timeoutId) clearTimeout(timeoutId);
        setTimeoutId(setTimeout(()=>{
            setTimeoutId(null);
            console.debug(e.detail);
        }, 200));
        // if(!await epWatched([ep])) return;
        // console.debug(e.detail);
    }
    return <Card
        sx={{
            width: '100%',
            display: 'flex',
            flexDirection: 'column',
        }}
        onClick={()=>setClick(!click)}
        variant='outlined'
        square
    >
        <Box sx={{ display: 'flex' }}>
            <CardMedia
                component='img'
                image={subject.img}
                sx={{height: 120, width:80}}
            />
            <CardContent sx={{ flex: '1 0 auto', padding: 1 }}>
                <Typography component="div" variant="subtitle2" color="primary" fontWeight={900}>
                    {subject.name_cn}
                </Typography>
                <Typography variant="caption" color="text.secondary" component="div">
                    @{subject.id} {subject.name}
                </Typography>
                <Typography variant="caption" color="text.secondary" component="div">
                    {subject.hot} 在看
                </Typography>
            </CardContent>
        </Box>
        <Collapse in={click}>
            <CardContent>
                <Episodes eps={subject.eps} onClick={onClick} />
            </CardContent>
        </Collapse>
        <LinearProgress variant="buffer" color="epProgress" value={watched} valueBuffer={aired} />
    </Card>
}