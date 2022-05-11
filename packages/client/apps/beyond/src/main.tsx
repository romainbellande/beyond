import { StrictMode } from 'react';
import * as ReactDOM from 'react-dom';
import { BrowserRouter, Route, Routes } from 'react-router-dom';
import { Provider } from 'react-redux';
import { store } from './store';
import App from './app/app';
import { ErrorBoundary, ThemedSuspense } from '@client/ui';

ReactDOM.render(
  <StrictMode>
    <ErrorBoundary>
      <Provider store={store}>
        <BrowserRouter>
          <ThemedSuspense>
            <Routes>
              <Route path="/*" element={<App />} />
            </Routes>
          </ThemedSuspense>
        </BrowserRouter>
      </Provider>
    </ErrorBoundary>
  </StrictMode>,
  document.getElementById('root')
);
