import { createBrowserRouter } from 'react-router-dom'

import Nav from './Nav'
import Index from '@/pages/Index'
import Home from "@/pages/index/Home";
import Signup from '@/pages/index/Signup'

export default createBrowserRouter([{
    path: '/',
    element: <Nav />,
    children: [
        {
            path: '/',
            element: <Index />,
            children: [
                {
                    path: '/',
                    element: <Home />,
                },
                {
                    path: '/signup',
                    element: <Signup />,
                },
            ],
        }
    ],
}])
