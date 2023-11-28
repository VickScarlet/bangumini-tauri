import { createBrowserRouter, RouterProvider } from 'react-router-dom'

import Index from '@/pages/Index'
import Signup from '@/pages/Signup'

export const router = createBrowserRouter([
    {
        path: '/',
        // element: <Index />,
        element: <Signup />,
    },
    {
        path: '/signup',
        element: <Signup />,
    },
])

export default ()=><RouterProvider router={router} />
