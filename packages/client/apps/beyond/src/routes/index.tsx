import { lazy } from 'react';
import type { RouteObject } from 'react-router-dom';
import { Navigate } from 'react-router-dom';
import { suspenseRouteWrapper } from '@client/beyond/utils';

const NotFound = lazy(() => import('../pages/404'));
const Galaxy = lazy(() => import('../pages/Dashboard/Galaxy'));

export const routes: RouteObject[] = [
  {
    path: '/',
    element: <Navigate to="/galaxy" />,
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
