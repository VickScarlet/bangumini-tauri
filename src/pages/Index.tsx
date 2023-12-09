import { useState } from 'react';
import { Outlet, Link } from "react-router-dom";
import { BottomNavigation, BottomNavigationAction, Box } from "@mui/material";
import { Home, Person } from '@mui/icons-material';
import { Scrollbars } from 'react-custom-scrollbars-2';

export default ()=> {
    const [value, setValue] = useState('/');
    return <Box sx={{height:'100vh'}} bgcolor='background.default'>
        <Scrollbars style={{height: 'calc(100vh - 50px)'}} autoHide>
            <Outlet />
        </Scrollbars>
        <BottomNavigation sx={{height:50}} value={value} onChange={(_,value)=>setValue(value)}>
            <BottomNavigationAction icon={<Home />} value='/' to='/' component={Link} />
            <BottomNavigationAction icon={<Person />} value='/signup' to='/signup' component={Link} />
        </BottomNavigation >
    </Box>
}
