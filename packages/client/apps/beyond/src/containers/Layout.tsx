import { useRoutes } from 'react-router-dom';
import Main from './Main';
import { ThemedSuspense } from '@client/ui';
import { routes } from '../routes';

function Layout() {
  const routeElement = useRoutes(routes);

  return (
    <div className="flex flex-col max-w-screen h-screen bg-gray-50 dark:bg-gray-900">
      <div className="flex bg-zinc-100 flex-auto">
        <Main>
          <ThemedSuspense>{routeElement}</ThemedSuspense>
        </Main>
      </div>
    </div>
  );
}

export default Layout;
