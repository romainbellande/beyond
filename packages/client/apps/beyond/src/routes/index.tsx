import { lazy } from 'react';
import type { RouteObject } from 'react-router-dom';
import { Navigate } from 'react-router-dom';
import { suspenseRouteWrapper } from '@client/beyond/utils';

const NotFound = lazy(() => import('../pages/404'));
const Galaxy = lazy(() => import('../pages/Dashboard/Galaxy'));
const Portal = lazy(() => import('../pages/Portal'));
const Login = lazy(() => import('../pages/Portal/Login'));
const Register = lazy(() => import('../pages/Portal/Register'));

export const routes: RouteObject[] = [
  {
    path: '/',
    element: <Navigate to="/galaxy" />,
  },
  {
    path: '/portal',
    element: <Portal />,
    children: [
      {
        path: '',
        element: <Navigate to="login" />,
      },
      {
        path: 'login',
        element: <Login />,
      },
      {
        path: 'register',
        element: <Register />,
      },
    ],
  },
  {
    path: '/galaxy',
    element: <Galaxy />,
  },
  {
    path: '*',
    element: <NotFound />,
  },
].map(suspenseRouteWrapper);
