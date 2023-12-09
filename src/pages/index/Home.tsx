import { getIndex } from '@/api'
import { useState, useEffect } from 'react'
import { List, ListItem, } from '@mui/material';
import Subject from '@/views/subject'

export default ()=>{
    const [login, setLogin] = useState(false);
    const [id, setId] = useState('');
    const [data, setData] = useState({});
    const [watchList, setWatchList] = useState<bgm.subject.Anime[]>([]);
    useEffect(()=>{
        getIndex().then(({login, id, data})=>{
            setLogin(login);
            setId(login?id:'');
            setData(data);
            if(login) {
                setWatchList(data.anime);
            }
            console.debug(login, id, data);
        })
    },[]);

    return <List>{watchList.map((item)=>
        <ListItem key={item.id}>
            <Subject.Anime subject={item} />
        </ListItem>)}
    </List>
}