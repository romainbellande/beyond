import './App.css';
import { useAppDispatch } from './store';
import { WsMessage } from './interfaces/ws-message.interface';
import { ws, setConnect } from './store/ws';
import { findPlanets } from './store/planets';
import { Planetarium } from './components/Planetarium';

function App() {
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
    <div className="App">
      <Planetarium />
    </div>
  );
}

export default App;
