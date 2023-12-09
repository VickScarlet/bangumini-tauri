import { useState } from "react";
import { Outlet, Link } from "react-router-dom";
import { Home, Person } from '@mui/icons-material';
import { SwipeableDrawer, Box, List, ListItem, ListItemButton, ListItemIcon, ListItemText } from '@mui/material';

export default ()=> {
    const [open, setOpen] = useState(false);

    return <>
        <Outlet />
        <SwipeableDrawer
            anchor="left"
            open={open}
            onClose={()=>{}}
            onOpen={()=>{}}
        >
            <Box sx={{ width: 250 }}>
                <List>
                    <ListItem disablePadding>
                        <ListItemButton to='/' component={Link}>
                            <ListItemIcon><Home /></ListItemIcon>
                            <ListItemText>Home</ListItemText>
                        </ListItemButton>
                    </ListItem>
                    <ListItem disablePadding>
                        <ListItemButton to='/signup' component={Link}>
                            <ListItemIcon><Person /></ListItemIcon>
                            <ListItemText>Sign Up</ListItemText>
                        </ListItemButton>
                    </ListItem>
                </List>
            </Box>
        </SwipeableDrawer>
    </>
}