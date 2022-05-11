// eslint-disable-next-line @typescript-eslint/no-unused-vars
import { lazy } from 'react';
import { Routes, Route } from 'react-router-dom';
import { useAppDispatch } from '@client/beyond/store';
import { WsMessage } from '@client/beyond/interfaces/ws-message.interface';
import { ws, setConnect } from '@client/beyond/store/ws';
import { findPlanets } from '@client/beyond/store/planets';

const Layout = lazy(() => import('../containers/Layout'));

export function App() {
  const dispatch = useAppDispatch();

  ws.onopen = () => {
    setConnect(true);
    console.info('connected to websocket');
    dispatch(findPlanets());
  };

  ws.onmessage = (e) => {
    const { data } = e;
    const message = WsMessage.decode(data);
    dispatch(message.getAction());
  };

  return (
    <Routes>
      <Route path="/*" element={<Layout />} />
    </Routes>
  );
}

export default App;
