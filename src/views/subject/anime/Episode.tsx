import { useMemo } from 'react'
import { Stack, Box, Typography } from '@mui/material'

const OneDay = 1000*60*60*24*7;

interface EpisodeProps {
    ep: bgm.subject.anime.Episode,
    onClick?: (epId: string, event: React.MouseEvent) => void,
}

export const Episode = ({ep, onClick}:EpisodeProps)=>{
    const state = useMemo(()=>{
        switch(ep.state) {
            case 'Watched': return 'epWatched';
            case 'Drop': return 'epDrop';
            case 'Queue': return 'epQueue';
        }
        if(!ep.on_air) return 'epDisable';
        const diff = Date.now() - new Date(ep.on_air).getTime();
        if(diff > OneDay) return 'epAired';
        if(diff > 0) return 'epOnAir';
        return 'epDisable';
    },[ep]);
    return <Box
        sx={{
            minWidth:22,
            height:22,
            display:'flex',
            justifyContent:'center',
            alignItems:'center',
            marginRight: 0.5,
            marginBottom: 0.5,
            cursor: 'pointer',
        }}
        bgcolor={state+'.main'}
        onClick={onClick?(e)=>onClick(ep.id, e):undefined}
    >
        <Typography variant="caption" color={state+'.contrastText'} component="div">
            {ep.ep}
        </Typography>
    </Box>
}

interface EpisodesProps {
    eps: bgm.subject.anime.TypedEpisode[],
    onClick?: EpisodeProps['onClick'],
}

export const Episodes = ({eps, onClick}: EpisodesProps)=><Stack sx={{padding:0}}>{eps.map((item)=>
    <Stack direction='row' key={item.typed??'def'} sx={{flexWrap: "wrap"}}>
        {item.eps.map((ep)=><Episode ep={ep} key={ep.id} onClick={onClick} />)}
    </Stack>)}
</Stack>

export default { Episode, Episodes }