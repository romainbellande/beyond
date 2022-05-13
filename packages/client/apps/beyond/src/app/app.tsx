// eslint-disable-next-line @typescript-eslint/no-unused-vars
import { lazy, useEffect } from 'react';
import { Routes, Route } from 'react-router-dom';
import { useAppDispatch } from '@client/beyond/store';
import { WsMessage } from '@client/beyond/interfaces/ws-message.interface';
import { w3cwebsocket as W3CWebSocket } from 'websocket';
import { setConnect } from '@client/beyond/store/ws';
import { findPlanets } from '@client/beyond/store/planets';
import { config } from '@client/beyond/config';

const Layout = lazy(() => import('../containers/Layout'));

export function App() {
  const dispatch = useAppDispatch();

  useEffect(() => {
    const ws = new W3CWebSocket(`${config.wsUrl}?jwt=my-jwt`);

    ws.binaryType = 'arraybuffer';

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

    window.ws = ws;
  }, []);

  return (
    <Routes>
      <Route path="/*" element={<Layout />} />
    </Routes>
  );
}

export default App;
